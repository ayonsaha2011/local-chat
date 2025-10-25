use lan_chat_core::{TransferId, UserId};
use serde::{Deserialize, Serialize};

/// File transfer protocol messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferMessage {
    /// Request to send a file
    TransferRequest {
        transfer_id: TransferId,
        sender_id: UserId,
        file_name: String,
        file_size: u64,
        file_hash: String,
    },

    /// Accept file transfer
    TransferAccept {
        transfer_id: TransferId,
    },

    /// Reject file transfer
    TransferReject {
        transfer_id: TransferId,
        reason: String,
    },

    /// Start sending file data
    StartTransfer {
        transfer_id: TransferId,
    },

    /// File data chunk
    DataChunk {
        transfer_id: TransferId,
        chunk_index: u64,
        data: Vec<u8>,
    },

    /// Transfer complete
    TransferComplete {
        transfer_id: TransferId,
    },

    /// Transfer failed
    TransferFailed {
        transfer_id: TransferId,
        error: String,
    },

    /// Request to pause transfer
    Pause {
        transfer_id: TransferId,
    },

    /// Resume transfer
    Resume {
        transfer_id: TransferId,
        from_chunk: u64,
    },

    /// Cancel transfer
    Cancel {
        transfer_id: TransferId,
    },
}

impl TransferMessage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

/// Transfer status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransferStatus {
    Pending,
    Accepted,
    InProgress,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// File transfer metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTransfer {
    pub transfer_id: TransferId,
    pub sender_id: UserId,
    pub recipient_id: UserId,
    pub file_name: String,
    pub file_size: u64,
    pub file_hash: String,
    pub bytes_transferred: u64,
    pub status: TransferStatus,
    pub error: Option<String>,
}

impl FileTransfer {
    pub fn new(
        sender_id: UserId,
        recipient_id: UserId,
        file_name: String,
        file_size: u64,
        file_hash: String,
    ) -> Self {
        Self {
            transfer_id: uuid::Uuid::new_v4(),
            sender_id,
            recipient_id,
            file_name,
            file_size,
            file_hash,
            bytes_transferred: 0,
            status: TransferStatus::Pending,
            error: None,
        }
    }

    pub fn progress_percentage(&self) -> f64 {
        if self.file_size == 0 {
            return 100.0;
        }
        (self.bytes_transferred as f64 / self.file_size as f64) * 100.0
    }
}
