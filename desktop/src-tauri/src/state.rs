use lan_chat_core::{
    ChatEvent, Message, NetworkAddress, PeerRegistry, UserProfile,
};
use lan_chat_crypto::KeyPair;
use lan_chat_discovery::DiscoveryService;
use lan_chat_protocol::MessagingServer;
use lan_chat_transfer::TransferService;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tauri::Window;

extern crate dirs;

pub struct AppState {
    pub user_profile: Arc<RwLock<Option<UserProfile>>>,
    pub keypair: Arc<RwLock<Option<KeyPair>>>,
    pub peer_registry: PeerRegistry,
    pub messages: Arc<RwLock<Vec<Message>>>,
    pub event_tx: mpsc::UnboundedSender<ChatEvent>,
    pub event_rx: Arc<RwLock<mpsc::UnboundedReceiver<ChatEvent>>>,
}

impl AppState {
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();

        Self {
            user_profile: Arc::new(RwLock::new(None)),
            keypair: Arc::new(RwLock::new(None)),
            peer_registry: PeerRegistry::new(),
            messages: Arc::new(RwLock::new(Vec::new())),
            event_tx,
            event_rx: Arc::new(RwLock::new(event_rx)),
        }
    }

    pub async fn start_services(&self) -> anyhow::Result<()> {
        tracing::info!("Starting chat services...");
        
        // Generate keypair
        let keypair = KeyPair::generate()?;
        let public_key = keypair.export_public_key_bytes()?;
        *self.keypair.write().await = Some(keypair.clone());

        // Get profile
        let profile = self
            .user_profile
            .read()
            .await
            .clone()
            .unwrap_or_else(|| UserProfile::new("user".into(), "User".into()));

        tracing::info!("User profile: {} ({})", profile.display_name, profile.user_id);

        // Get local IP
        let local_ip = match get_local_ip() {
            Some(ip) => {
                tracing::info!("✅ Local IP: {}", ip);
                ip
            }
            None => {
                tracing::error!("❌ Failed to detect local IP - services may not work correctly");
                return Err(anyhow::anyhow!("Failed to detect local network IP address. Please check your network connection."));
            }
        };

        // Start discovery service
        tracing::info!("Starting peer discovery service...");
        let discovery = Arc::new(
            DiscoveryService::new(
                profile.clone(),
                NetworkAddress::new(local_ip, lan_chat_protocol::MESSAGING_PORT),
                self.peer_registry.clone(),
                self.event_tx.clone(),
            )
            .with_public_key(public_key),
        );

        tokio::spawn(async move {
            if let Err(e) = discovery.start().await {
                tracing::error!("Discovery service error: {}", e);
            }
        });

        // Start messaging server
        tracing::info!("Starting messaging server on port {}...", lan_chat_protocol::MESSAGING_PORT);
        let messaging = Arc::new(MessagingServer::new(
            profile.clone(),
            keypair,
            self.peer_registry.clone(),
            self.event_tx.clone(),
        ));

        tokio::spawn(async move {
            if let Err(e) = messaging.start().await {
                tracing::error!("Messaging server error: {}", e);
            }
        });

        // Start transfer service
        let download_dir = dirs::download_dir().unwrap_or_else(|| PathBuf::from("."));
        tracing::info!("Starting file transfer service (downloads: {:?})...", download_dir);
        let transfer = Arc::new(TransferService::new(
            profile.user_id,
            self.peer_registry.clone(),
            self.event_tx.clone(),
            download_dir,
        ));

        tokio::spawn(async move {
            if let Err(e) = transfer.start().await {
                tracing::error!("Transfer service error: {}", e);
            }
        });

        tracing::info!("All services started successfully!");
        Ok(())
    }

    pub async fn listen_events(&self, window: Window) {
        let mut rx = self.event_rx.write().await;

        while let Some(event) = rx.recv().await {
            // Emit event to frontend
            match &event {
                ChatEvent::PeerDiscovered(peer) => {
                    let _ = window.emit("peer-discovered", peer);
                }
                ChatEvent::PeerConnected(peer) => {
                    let _ = window.emit("peer-connected", peer);
                }
                ChatEvent::PeerDisconnected(user_id) => {
                    let _ = window.emit("peer-disconnected", user_id);
                }
                ChatEvent::MessageReceived(msg) => {
                    // Store message
                    let mut messages = self.messages.write().await;
                    messages.push(msg.clone());
                    let _ = window.emit("message-received", msg);
                }
                ChatEvent::MessageSent(msg) => {
                    let _ = window.emit("message-sent", msg);
                }
                ChatEvent::FileTransferRequested { transfer_id, from, file_name, file_size } => {
                    let _ = window.emit("file-transfer-requested", serde_json::json!({
                        "transfer_id": transfer_id,
                        "from": from,
                        "file_name": file_name,
                        "file_size": file_size,
                    }));
                }
                ChatEvent::FileTransferProgress { transfer_id, bytes_transferred, total_bytes } => {
                    let _ = window.emit("file-transfer-progress", serde_json::json!({
                        "transfer_id": transfer_id,
                        "bytes_transferred": bytes_transferred,
                        "total_bytes": total_bytes,
                    }));
                }
                ChatEvent::FileTransferCompleted { transfer_id } => {
                    let _ = window.emit("file-transfer-completed", serde_json::json!({
                        "transfer_id": transfer_id,
                    }));
                }
                _ => {
                    // Fallback for other events
                    let _ = window.emit("chat-event", &event);
                }
            }
        }
    }
}

fn get_local_ip() -> Option<IpAddr> {
    use local_ip_address::local_ip;

    // Try to get local IP, filtering out loopback and WSL interfaces
    if let Ok(ip) = local_ip() {
        if !ip.is_loopback() {
            tracing::info!("Detected local IP: {}", ip);
            return Some(ip);
        }
    }

    // Fallback: connect to external address to determine routing IP
    use std::net::UdpSocket;
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                let ip = addr.ip();
                if !ip.is_loopback() {
                    tracing::info!("Fallback: detected local IP via routing: {}", ip);
                    return Some(ip);
                }
            }
        }
    }

    // Try alternative external addresses
    let fallback_addresses = ["1.1.1.1:80", "208.67.222.222:80"];
    for addr in fallback_addresses.iter() {
        if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
            if socket.connect(addr).is_ok() {
                if let Ok(local_addr) = socket.local_addr() {
                    let ip = local_addr.ip();
                    if !ip.is_loopback() {
                        tracing::info!("Fallback: detected local IP via routing ({}): {}", addr, ip);
                        return Some(ip);
                    }
                }
            }
        }
    }

    // CRITICAL: Do NOT fall back to loopback - it won't work for cross-device communication
    tracing::error!("❌ CRITICAL: Failed to detect local network IP address!");
    tracing::error!("This means the device cannot be discovered by other devices on the network.");
    tracing::error!("Please check your network connection and firewall settings.");
    None
}
