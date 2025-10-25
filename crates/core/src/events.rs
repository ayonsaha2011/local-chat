use crate::{Message, Peer, ReadReceipt, TypingIndicator, UserId, UserStatus};
use serde::{Deserialize, Serialize};

/// Events that can occur in the chat system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatEvent {
    // Peer events
    PeerDiscovered(Peer),
    PeerConnected(Peer),
    PeerDisconnected(UserId),
    PeerStatusChanged { user_id: UserId, status: UserStatus },

    // Message events
    MessageReceived(Message),
    MessageSent(Message),
    MessageDelivered { message_id: uuid::Uuid },
    MessageRead(ReadReceipt),

    // Typing events
    TypingIndicator(TypingIndicator),

    // File transfer events
    FileTransferRequested {
        transfer_id: uuid::Uuid,
        from: UserId,
        file_name: String,
        file_size: u64,
    },
    FileTransferAccepted {
        transfer_id: uuid::Uuid,
    },
    FileTransferProgress {
        transfer_id: uuid::Uuid,
        bytes_transferred: u64,
        total_bytes: u64,
    },
    FileTransferCompleted {
        transfer_id: uuid::Uuid,
    },
    FileTransferFailed {
        transfer_id: uuid::Uuid,
        error: String,
    },

    // System events
    Error(String),
    NetworkStatusChanged { connected: bool },
}
