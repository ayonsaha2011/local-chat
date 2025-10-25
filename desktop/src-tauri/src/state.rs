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

        // Get local IP
        let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".parse().unwrap());

        // Start discovery service
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
                eprintln!("Discovery service error: {}", e);
            }
        });

        // Start messaging server
        let messaging = Arc::new(MessagingServer::new(
            profile.clone(),
            keypair,
            self.peer_registry.clone(),
            self.event_tx.clone(),
        ));

        tokio::spawn(async move {
            if let Err(e) = messaging.start().await {
                eprintln!("Messaging server error: {}", e);
            }
        });

        // Start transfer service
        let download_dir = dirs::download_dir().unwrap_or_else(|| PathBuf::from("."));
        let transfer = Arc::new(TransferService::new(
            profile.user_id,
            self.peer_registry.clone(),
            self.event_tx.clone(),
            download_dir,
        ));

        tokio::spawn(async move {
            if let Err(e) = transfer.start().await {
                eprintln!("Transfer service error: {}", e);
            }
        });

        Ok(())
    }

    pub async fn listen_events(&self, window: Window) {
        let mut rx = self.event_rx.write().await;

        while let Some(event) = rx.recv().await {
            // Emit event to frontend
            let event_name = match &event {
                ChatEvent::PeerDiscovered(_) => "peer-discovered",
                ChatEvent::PeerConnected(_) => "peer-connected",
                ChatEvent::PeerDisconnected(_) => "peer-disconnected",
                ChatEvent::MessageReceived(msg) => {
                    // Store message
                    let mut messages = self.messages.write().await;
                    messages.push(msg.clone());
                    "message-received"
                }
                ChatEvent::MessageSent(_) => "message-sent",
                ChatEvent::FileTransferRequested { .. } => "file-transfer-requested",
                ChatEvent::FileTransferProgress { .. } => "file-transfer-progress",
                ChatEvent::FileTransferCompleted { .. } => "file-transfer-completed",
                _ => "chat-event",
            };

            let _ = window.emit(event_name, &event);
        }
    }
}

fn get_local_ip() -> Option<IpAddr> {
    use std::net::UdpSocket;

    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    socket.local_addr().ok().map(|addr| addr.ip())
}
