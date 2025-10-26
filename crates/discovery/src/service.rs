use crate::protocol::{DiscoveryMessage, DISCOVERY_PORT, MULTICAST_ADDR_V4};
use lan_chat_core::{ChatEvent, NetworkAddress, Peer, PeerRegistry, UserProfile};
use socket2::{Domain, Protocol, Socket, Type};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};
use tracing::{debug, error, info, warn};

const BUFFER_SIZE: usize = 8192;
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
const PEER_TIMEOUT: i64 = 45; // seconds

/// Peer discovery service using UDP multicast
pub struct DiscoveryService {
    profile: UserProfile,
    listen_address: NetworkAddress,
    peer_registry: PeerRegistry,
    event_tx: mpsc::UnboundedSender<ChatEvent>,
    public_key: Option<Vec<u8>>,
}

impl DiscoveryService {
    pub fn new(
        profile: UserProfile,
        listen_address: NetworkAddress,
        peer_registry: PeerRegistry,
        event_tx: mpsc::UnboundedSender<ChatEvent>,
    ) -> Self {
        Self {
            profile,
            listen_address,
            peer_registry,
            event_tx,
            public_key: None,
        }
    }

    pub fn with_public_key(mut self, public_key: Vec<u8>) -> Self {
        self.public_key = Some(public_key);
        self
    }

    /// Start the discovery service
    pub async fn start(self: Arc<Self>) -> lan_chat_core::Result<()> {
        info!("Starting discovery service on port {}", DISCOVERY_PORT);

        // Create multicast socket
        let socket = self.create_multicast_socket()?;
        let socket = Arc::new(socket);

        // Spawn receiver task
        let receiver_handle = {
            let service = Arc::clone(&self);
            let socket = Arc::clone(&socket);
            tokio::spawn(async move {
                service.receive_loop(socket).await;
            })
        };

        // Spawn heartbeat task
        let heartbeat_handle = {
            let service = Arc::clone(&self);
            let socket = Arc::clone(&socket);
            tokio::spawn(async move {
                service.heartbeat_loop(socket).await;
            })
        };

        // Spawn cleanup task
        let cleanup_handle = {
            let service = Arc::clone(&self);
            tokio::spawn(async move {
                service.cleanup_loop().await;
            })
        };

        // Send initial announcement
        self.announce(socket.as_ref()).await?;

        // Wait for tasks
        tokio::select! {
            _ = receiver_handle => warn!("Receiver task ended"),
            _ = heartbeat_handle => warn!("Heartbeat task ended"),
            _ = cleanup_handle => warn!("Cleanup task ended"),
        }

        Ok(())
    }

    /// Create and configure multicast socket
    fn create_multicast_socket(&self) -> lan_chat_core::Result<Socket> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        socket
            .set_reuse_address(true)
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), DISCOVERY_PORT);
        socket
            .bind(&addr.into())
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        // Join multicast group
        let multicast_addr: Ipv4Addr = MULTICAST_ADDR_V4
            .parse()
            .map_err(|e| lan_chat_core::ChatError::Network(format!("Invalid multicast address: {}", e)))?;

        socket
            .join_multicast_v4(&multicast_addr, &Ipv4Addr::UNSPECIFIED)
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        // Set multicast TTL (important for routing)
        socket
            .set_multicast_ttl_v4(32)
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        // Enable multicast loopback (receive our own messages for testing)
        socket
            .set_multicast_loop_v4(true)
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        // Set the outgoing interface for multicast - use the local IP
        if let IpAddr::V4(local_ipv4) = self.listen_address.ip {
            socket
                .set_multicast_if_v4(&local_ipv4)
                .map_err(|e| lan_chat_core::ChatError::Network(format!("Failed to set multicast interface: {}", e)))?;
            info!("Set multicast interface to: {}", local_ipv4);
        }

        socket
            .set_nonblocking(true)
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        Ok(socket)
    }

    /// Receive loop for incoming discovery messages
    async fn receive_loop(&self, socket: Arc<Socket>) {
        let mut buffer = vec![0u8; BUFFER_SIZE];

        loop {
            let std_socket: std::net::UdpSocket = socket.try_clone().unwrap().into();
            let tokio_socket = tokio::net::UdpSocket::from_std(std_socket).unwrap();

            match tokio_socket.recv_from(&mut buffer).await {
                Ok((len, addr)) => {
                    if let Err(e) = self.handle_message(&buffer[..len], addr).await {
                        debug!("Error handling message from {}: {}", addr, e);
                    }
                }
                Err(e) => {
                    error!("Error receiving discovery message: {}", e);
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    /// Handle incoming discovery message
    async fn handle_message(&self, data: &[u8], from: SocketAddr) -> lan_chat_core::Result<()> {
        let message = DiscoveryMessage::from_bytes(data)
            .map_err(|e| lan_chat_core::ChatError::Protocol(e.to_string()))?;

        match message {
            DiscoveryMessage::Announce {
                profile,
                address,
                public_key,
            } => {
                // Ignore our own announcements
                if profile.user_id == self.profile.user_id {
                    return Ok(());
                }

                debug!("Peer announced: {}", profile.display_name);

                let mut peer = Peer::new(profile, address);
                peer.public_key = public_key;

                self.peer_registry.add_peer(peer.clone()).await;
                let _ = self.event_tx.send(ChatEvent::PeerDiscovered(peer));
            }

            DiscoveryMessage::DiscoveryRequest => {
                debug!("Discovery request from {}", from);
                // Respond with our info (implemented via heartbeat)
            }

            DiscoveryMessage::DiscoveryResponse {
                profile,
                address,
                public_key,
            } => {
                if profile.user_id == self.profile.user_id {
                    return Ok(());
                }

                let mut peer = Peer::new(profile, address);
                peer.public_key = public_key;

                self.peer_registry.add_peer(peer.clone()).await;
                let _ = self.event_tx.send(ChatEvent::PeerDiscovered(peer));
            }

            DiscoveryMessage::Goodbye { user_id } => {
                if user_id == self.profile.user_id {
                    return Ok(());
                }

                debug!("Peer going offline: {}", user_id);
                self.peer_registry.remove_peer(&user_id).await;
                let _ = self.event_tx.send(ChatEvent::PeerDisconnected(user_id));
            }

            DiscoveryMessage::Heartbeat { user_id, status } => {
                if user_id == self.profile.user_id {
                    return Ok(());
                }

                self.peer_registry.update_peer_status(&user_id, status).await;

                if let Some(_peer) = self.peer_registry.get_peer(&user_id).await {
                    let _ = self.event_tx.send(ChatEvent::PeerStatusChanged { user_id, status });
                }
            }
        }

        Ok(())
    }

    /// Send announcement to the network
    async fn announce(&self, socket: &Socket) -> lan_chat_core::Result<()> {
        let message = DiscoveryMessage::Announce {
            profile: self.profile.clone(),
            address: self.listen_address.clone(),
            public_key: self.public_key.clone(),
        };

        self.send_multicast(socket, &message).await
    }

    /// Heartbeat loop to maintain presence
    async fn heartbeat_loop(&self, socket: Arc<Socket>) {
        let mut ticker = interval(HEARTBEAT_INTERVAL);

        loop {
            ticker.tick().await;

            let message = DiscoveryMessage::Heartbeat {
                user_id: self.profile.user_id,
                status: self.profile.status,
            };

            if let Err(e) = self.send_multicast(socket.as_ref(), &message).await {
                error!("Failed to send heartbeat: {}", e);
            }
        }
    }

    /// Cleanup loop to remove offline peers
    async fn cleanup_loop(&self) {
        let mut ticker = interval(Duration::from_secs(30));

        loop {
            ticker.tick().await;
            self.peer_registry.cleanup_offline_peers(PEER_TIMEOUT).await;
        }
    }

    /// Send a message to the multicast group
    async fn send_multicast(
        &self,
        socket: &Socket,
        message: &DiscoveryMessage,
    ) -> lan_chat_core::Result<()> {
        let data = message
            .to_bytes()
            .map_err(|e| lan_chat_core::ChatError::Protocol(e.to_string()))?;

        let multicast_addr: Ipv4Addr = MULTICAST_ADDR_V4
            .parse()
            .map_err(|e| lan_chat_core::ChatError::Network(format!("Invalid multicast address: {}", e)))?;

        let dest = SocketAddr::new(IpAddr::V4(multicast_addr), DISCOVERY_PORT);

        socket
            .send_to(&data, &dest.into())
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        Ok(())
    }

    /// Send goodbye message before shutting down
    pub async fn shutdown(&self, socket: &Socket) -> lan_chat_core::Result<()> {
        let message = DiscoveryMessage::Goodbye {
            user_id: self.profile.user_id,
        };

        self.send_multicast(socket, &message).await
    }
}
