use crate::connection::PeerConnection;
use crate::messages::ProtocolMessage;
use crate::MESSAGING_PORT;
use lan_chat_core::{ChatEvent, Message, PeerRegistry, UserProfile};
use lan_chat_crypto::{HybridEncryption, KeyPair};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};

/// Messaging server
pub struct MessagingServer {
    profile: UserProfile,
    keypair: KeyPair,
    peer_registry: PeerRegistry,
    connections: Arc<RwLock<HashMap<uuid::Uuid, PeerConnection>>>,
    event_tx: mpsc::UnboundedSender<ChatEvent>,
}

impl MessagingServer {
    pub fn new(
        profile: UserProfile,
        keypair: KeyPair,
        peer_registry: PeerRegistry,
        event_tx: mpsc::UnboundedSender<ChatEvent>,
    ) -> Self {
        Self {
            profile,
            keypair,
            peer_registry,
            connections: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
        }
    }

    /// Start the messaging server
    pub async fn start(self: Arc<Self>) -> lan_chat_core::Result<()> {
        let addr = SocketAddr::from(([0, 0, 0, 0], MESSAGING_PORT));
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        info!("Messaging server listening on {}", addr);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    debug!("New connection from {}", addr);
                    let server = Arc::clone(&self);
                    tokio::spawn(async move {
                        if let Err(e) = server.handle_connection(stream).await {
                            error!("Error handling connection from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                }
            }
        }
    }

    /// Handle incoming connection
    async fn handle_connection(&self, stream: TcpStream) -> lan_chat_core::Result<()> {
        let mut conn = PeerConnection::new(stream);

        // Perform handshake
        let peer_id = self.perform_handshake(&mut conn).await?;
        conn.set_peer_id(peer_id);

        // Store connection
        {
            let mut connections = self.connections.write().await;
            connections.insert(peer_id, conn);
        }

        // Notify connection established
        if let Some(peer) = self.peer_registry.get_peer(&peer_id).await {
            let _ = self.event_tx.send(ChatEvent::PeerConnected(peer));
        }

        // Handle messages
        loop {
            let mut connections = self.connections.write().await;
            let conn = connections.get_mut(&peer_id).unwrap();

            match conn.receive_message().await {
                Ok(message) => {
                    drop(connections); // Release lock before handling
                    if let Err(e) = self.handle_message(peer_id, message).await {
                        error!("Error handling message from {}: {}", peer_id, e);
                    }
                }
                Err(e) => {
                    error!("Connection error with {}: {}", peer_id, e);
                    drop(connections);
                    self.remove_connection(&peer_id).await;
                    let _ = self.event_tx.send(ChatEvent::PeerDisconnected(peer_id));
                    break;
                }
            }
        }

        Ok(())
    }

    /// Perform handshake with peer
    async fn perform_handshake(
        &self,
        conn: &mut PeerConnection,
    ) -> lan_chat_core::Result<uuid::Uuid> {
        // Wait for handshake from peer
        let message = conn.receive_message().await?;

        match message {
            ProtocolMessage::Handshake {
                version,
                user_id,
                public_key,
            } => {
                if version != crate::messages::PROTOCOL_VERSION {
                    return Err(lan_chat_core::ChatError::Protocol(format!(
                        "Unsupported protocol version: {}",
                        version
                    )));
                }

                // Send handshake acknowledgment
                let ack = ProtocolMessage::HandshakeAck {
                    user_id: self.profile.user_id,
                    public_key: self.keypair.export_public_key_bytes()
                        .map_err(|e| lan_chat_core::ChatError::Crypto(e.to_string()))?,
                };

                conn.send_message(&ack).await?;

                // Update peer's public key
                if let Some(mut peer) = self.peer_registry.get_peer(&user_id).await {
                    peer.public_key = Some(public_key);
                    self.peer_registry.add_peer(peer).await;
                }

                Ok(user_id)
            }
            _ => Err(lan_chat_core::ChatError::Protocol(
                "Expected handshake message".into(),
            )),
        }
    }

    /// Handle incoming protocol message
    async fn handle_message(
        &self,
        peer_id: uuid::Uuid,
        message: ProtocolMessage,
    ) -> lan_chat_core::Result<()> {
        match message {
            ProtocolMessage::Message {
                mut message,
                encrypted_key,
                encrypted_data,
            } => {
                // Decrypt if encrypted
                if let (Some(key), Some(data)) = (encrypted_key, encrypted_data) {
                    let plaintext =
                        HybridEncryption::decrypt(self.keypair.private_key(), &key, &data)
                            .map_err(|e| lan_chat_core::ChatError::Crypto(e.to_string()))?;
                    message.content = String::from_utf8(plaintext)
                        .map_err(|e| lan_chat_core::ChatError::InvalidData(e.to_string()))?;
                }

                // Send acknowledgment
                self.send_to_peer(
                    &peer_id,
                    &ProtocolMessage::MessageAck {
                        message_id: message.id,
                    },
                )
                .await?;

                // Emit event
                let _ = self.event_tx.send(ChatEvent::MessageReceived(message));
            }

            ProtocolMessage::MessageAck { message_id } => {
                let _ = self
                    .event_tx
                    .send(ChatEvent::MessageDelivered { message_id });
            }

            ProtocolMessage::MessageDelivered { message_id } => {
                let _ = self
                    .event_tx
                    .send(ChatEvent::MessageDelivered { message_id });
            }

            ProtocolMessage::MessageRead { receipt } => {
                let _ = self.event_tx.send(ChatEvent::MessageRead(receipt));
            }

            ProtocolMessage::Typing { indicator } => {
                let _ = self.event_tx.send(ChatEvent::TypingIndicator(indicator));
            }

            ProtocolMessage::Ping => {
                self.send_to_peer(&peer_id, &ProtocolMessage::Pong)
                    .await?;
            }

            ProtocolMessage::Pong => {
                // Keep-alive acknowledged
            }

            _ => {
                warn!("Unhandled message type from {}", peer_id);
            }
        }

        Ok(())
    }

    /// Send a message to a specific peer
    pub async fn send_to_peer(
        &self,
        peer_id: &uuid::Uuid,
        message: &ProtocolMessage,
    ) -> lan_chat_core::Result<()> {
        let mut connections = self.connections.write().await;

        if let Some(conn) = connections.get_mut(peer_id) {
            conn.send_message(message).await
        } else {
            Err(lan_chat_core::ChatError::PeerNotFound(
                peer_id.to_string(),
            ))
        }
    }

    /// Send an encrypted message to a peer
    pub async fn send_encrypted_message(
        &self,
        peer_id: &uuid::Uuid,
        message: Message,
    ) -> lan_chat_core::Result<()> {
        // Get peer's public key
        let peer = self
            .peer_registry
            .get_peer(peer_id)
            .await
            .ok_or_else(|| lan_chat_core::ChatError::PeerNotFound(peer_id.to_string()))?;

        let peer_public_key = peer
            .public_key
            .as_ref()
            .ok_or_else(|| lan_chat_core::ChatError::Encryption("No public key for peer".into()))?;

        let public_key = KeyPair::import_public_key_bytes(peer_public_key)
            .map_err(|e| lan_chat_core::ChatError::Crypto(e.to_string()))?;

        // Encrypt message content
        let (encrypted_key, encrypted_data) =
            HybridEncryption::encrypt(&public_key, message.content.as_bytes())
                .map_err(|e| lan_chat_core::ChatError::Crypto(e.to_string()))?;

        let protocol_message = ProtocolMessage::Message {
            message,
            encrypted_key: Some(encrypted_key),
            encrypted_data: Some(encrypted_data),
        };

        self.send_to_peer(peer_id, &protocol_message).await
    }

    /// Remove a connection
    async fn remove_connection(&self, peer_id: &uuid::Uuid) {
        let mut connections = self.connections.write().await;
        connections.remove(peer_id);
    }

    /// Connect to a peer
    pub async fn connect_to_peer(
        &self,
        peer_id: &uuid::Uuid,
    ) -> lan_chat_core::Result<()> {
        // Get peer address
        let peer = self
            .peer_registry
            .get_peer(peer_id)
            .await
            .ok_or_else(|| lan_chat_core::ChatError::PeerNotFound(peer_id.to_string()))?;

        let addr = peer.address.to_socket_addr();

        // Connect
        let stream = TcpStream::connect(addr)
            .await
            .map_err(|e| lan_chat_core::ChatError::Network(e.to_string()))?;

        let mut conn = PeerConnection::new(stream);

        // Send handshake
        let handshake = ProtocolMessage::Handshake {
            version: crate::messages::PROTOCOL_VERSION,
            user_id: self.profile.user_id,
            public_key: self.keypair.export_public_key_bytes()
                .map_err(|e| lan_chat_core::ChatError::Crypto(e.to_string()))?,
        };

        conn.send_message(&handshake).await?;

        // Wait for acknowledgment
        match conn.receive_message().await? {
            ProtocolMessage::HandshakeAck {
                user_id,
                public_key,
            } => {
                conn.set_peer_id(user_id);

                // Update peer's public key
                if let Some(mut peer) = self.peer_registry.get_peer(&user_id).await {
                    peer.public_key = Some(public_key);
                    self.peer_registry.add_peer(peer.clone()).await;
                }

                // Store connection
                {
                    let mut connections = self.connections.write().await;
                    connections.insert(user_id, conn);
                }

                let _ = self.event_tx.send(ChatEvent::PeerConnected(peer));

                Ok(())
            }
            _ => Err(lan_chat_core::ChatError::Protocol(
                "Expected handshake acknowledgment".into(),
            )),
        }
    }
}
