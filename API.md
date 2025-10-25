# API Reference

## Core Types

### UserProfile

Represents a user's profile information.

```rust
pub struct UserProfile {
    pub user_id: UserId,
    pub username: String,
    pub display_name: String,
    pub status: UserStatus,
    pub status_message: Option<String>,
    pub avatar_hash: Option<String>,
}
```

### Peer

Represents a discovered peer on the network.

```rust
pub struct Peer {
    pub profile: UserProfile,
    pub address: NetworkAddress,
    pub last_seen: DateTime<Utc>,
    pub public_key: Option<Vec<u8>>,
}
```

### Message

Represents a chat message.

```rust
pub struct Message {
    pub id: Uuid,
    pub session_id: SessionId,
    pub sender_id: UserId,
    pub recipient_id: UserId,
    pub message_type: MessageType,
    pub content: String,
    pub metadata: Option<MessageMetadata>,
    pub timestamp: DateTime<Utc>,
    pub status: MessageStatus,
    pub encrypted: bool,
}
```

## Services

### DiscoveryService

Handles peer discovery using UDP multicast.

```rust
impl DiscoveryService {
    pub fn new(
        profile: UserProfile,
        listen_address: NetworkAddress,
        peer_registry: PeerRegistry,
        event_tx: mpsc::UnboundedSender<ChatEvent>,
    ) -> Self;

    pub async fn start(self: Arc<Self>) -> Result<()>;
    pub async fn shutdown(&self, socket: &Socket) -> Result<()>;
}
```

**Events Emitted:**
- `PeerDiscovered(Peer)` - When a new peer is found
- `PeerDisconnected(UserId)` - When a peer goes offline

### MessagingServer

Handles secure messaging between peers.

```rust
impl MessagingServer {
    pub fn new(
        profile: UserProfile,
        keypair: KeyPair,
        peer_registry: PeerRegistry,
        event_tx: mpsc::UnboundedSender<ChatEvent>,
    ) -> Self;

    pub async fn start(self: Arc<Self>) -> Result<()>;
    pub async fn send_encrypted_message(
        &self,
        peer_id: &UserId,
        message: Message,
    ) -> Result<()>;
    pub async fn connect_to_peer(&self, peer_id: &UserId) -> Result<()>;
}
```

**Events Emitted:**
- `MessageReceived(Message)` - When a message is received
- `MessageSent(Message)` - When a message is sent
- `MessageDelivered { message_id }` - When delivery is confirmed
- `MessageRead(ReadReceipt)` - When message is read

### TransferService

Handles file transfers between peers.

```rust
impl TransferService {
    pub fn new(
        user_id: UserId,
        peer_registry: PeerRegistry,
        event_tx: mpsc::UnboundedSender<ChatEvent>,
        download_dir: PathBuf,
    ) -> Self;

    pub async fn start(self: Arc<Self>) -> Result<()>;
    
    pub async fn send_file(
        &self,
        recipient_id: UserId,
        file_path: &Path,
    ) -> Result<TransferId>;
    
    pub async fn accept_transfer(&self, transfer_id: TransferId) -> Result<()>;
    pub async fn reject_transfer(&self, transfer_id: TransferId, reason: String) -> Result<()>;
}
```

**Events Emitted:**
- `FileTransferRequested { transfer_id, from, file_name, file_size }`
- `FileTransferProgress { transfer_id, bytes_transferred, total_bytes }`
- `FileTransferCompleted { transfer_id }`
- `FileTransferFailed { transfer_id, error }`

## Cryptography

### KeyPair

RSA key pair for asymmetric encryption.

```rust
impl KeyPair {
    pub fn generate() -> Result<Self>;
    pub fn public_key(&self) -> &RsaPublicKey;
    pub fn export_public_key_bytes(&self) -> Result<Vec<u8>>;
}
```

### HybridEncryption

Combines RSA and AES for efficient encryption.

```rust
impl HybridEncryption {
    pub fn encrypt(
        recipient_public_key: &RsaPublicKey,
        plaintext: &[u8],
    ) -> Result<(EncryptedSessionKey, EncryptedData)>;

    pub fn decrypt(
        private_key: &RsaPrivateKey,
        encrypted_key: &EncryptedSessionKey,
        encrypted_data: &EncryptedData,
    ) -> Result<Vec<u8>>;
}
```

## Tauri Commands

### Frontend API

All commands are invoked from the frontend using `@tauri-apps/api`:

```typescript
// Initialize the application
await invoke('initialize_app', {
  request: { username, display_name }
});

// Get user profile
const profile = await invoke('get_user_profile');

// Get all peers
const peers = await invoke('get_peers');

// Send a message
await invoke('send_message', {
  request: { recipient_id, content }
});

// Send a file
const transferId = await invoke('send_file', {
  request: { recipient_id, file_path }
});

// Accept file transfer
await invoke('accept_file_transfer', { transferId });

// Reject file transfer
await invoke('reject_file_transfer', { transferId });
```

### Event Listeners

Listen to backend events:

```typescript
import { listen } from '@tauri-apps/api/event';

// Listen for peer discovery
await listen('peer-discovered', (event) => {
  console.log('New peer:', event.payload);
});

// Listen for messages
await listen('message-received', (event) => {
  console.log('Message:', event.payload);
});

// Listen for file transfers
await listen('file-transfer-requested', (event) => {
  console.log('Transfer request:', event.payload);
});

await listen('file-transfer-progress', (event) => {
  const { transfer_id, bytes_transferred, total_bytes } = event.payload;
  const progress = (bytes_transferred / total_bytes) * 100;
  console.log(`Transfer ${transfer_id}: ${progress}%`);
});
```

## Protocol Messages

### Discovery Protocol

```rust
pub enum DiscoveryMessage {
    Announce {
        profile: UserProfile,
        address: NetworkAddress,
        public_key: Option<Vec<u8>>,
    },
    DiscoveryRequest,
    DiscoveryResponse {
        profile: UserProfile,
        address: NetworkAddress,
        public_key: Option<Vec<u8>>,
    },
    Goodbye { user_id: UserId },
    Heartbeat { user_id: UserId, status: UserStatus },
}
```

### Messaging Protocol

```rust
pub enum ProtocolMessage {
    Handshake {
        version: u32,
        user_id: UserId,
        public_key: Vec<u8>,
    },
    HandshakeAck {
        user_id: UserId,
        public_key: Vec<u8>,
    },
    Message {
        message: Message,
        encrypted_key: Option<EncryptedSessionKey>,
        encrypted_data: Option<EncryptedData>,
    },
    MessageAck { message_id: Uuid },
    MessageDelivered { message_id: Uuid },
    MessageRead { receipt: ReadReceipt },
    Typing { indicator: TypingIndicator },
    Ping,
    Pong,
}
```

### Transfer Protocol

```rust
pub enum TransferMessage {
    TransferRequest {
        transfer_id: TransferId,
        sender_id: UserId,
        file_name: String,
        file_size: u64,
        file_hash: String,
    },
    TransferAccept { transfer_id: TransferId },
    TransferReject { transfer_id: TransferId, reason: String },
    StartTransfer { transfer_id: TransferId },
    DataChunk {
        transfer_id: TransferId,
        chunk_index: u64,
        data: Vec<u8>,
    },
    TransferComplete { transfer_id: TransferId },
    TransferFailed { transfer_id: TransferId, error: String },
    Pause { transfer_id: TransferId },
    Resume { transfer_id: TransferId, from_chunk: u64 },
    Cancel { transfer_id: TransferId },
}
```

## Error Handling

All operations return `Result<T, ChatError>`:

```rust
pub enum ChatError {
    Network(String),
    Encryption(String),
    Protocol(String),
    PeerNotFound(String),
    FileTransfer(String),
    Io(std::io::Error),
    Serialization(serde_json::Error),
    InvalidData(String),
}
```

## Examples

### Creating a Simple Chat Client

```rust
use lan_chat_core::*;
use lan_chat_crypto::KeyPair;
use lan_chat_discovery::DiscoveryService;
use lan_chat_protocol::MessagingServer;
use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create profile
    let profile = UserProfile::new("alice".into(), "Alice".into());
    
    // Generate encryption keys
    let keypair = KeyPair::generate()?;
    
    // Create peer registry
    let peer_registry = PeerRegistry::new();
    
    // Create event channel
    let (event_tx, mut event_rx) = mpsc::unbounded_channel();
    
    // Start discovery service
    let discovery = Arc::new(DiscoveryService::new(
        profile.clone(),
        NetworkAddress::new("192.168.1.100".parse()?, 37843),
        peer_registry.clone(),
        event_tx.clone(),
    ));
    
    tokio::spawn(async move {
        discovery.start().await.unwrap();
    });
    
    // Start messaging server
    let messaging = Arc::new(MessagingServer::new(
        profile,
        keypair,
        peer_registry,
        event_tx,
    ));
    
    tokio::spawn(async move {
        messaging.start().await.unwrap();
    });
    
    // Handle events
    while let Some(event) = event_rx.recv().await {
        match event {
            ChatEvent::PeerDiscovered(peer) => {
                println!("Discovered: {}", peer.profile.display_name);
            }
            ChatEvent::MessageReceived(msg) => {
                println!("Message: {}", msg.content);
            }
            _ => {}
        }
    }
    
    Ok(())
}
```
