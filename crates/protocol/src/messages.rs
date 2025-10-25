use lan_chat_core::{Message, ReadReceipt, SessionId, TypingIndicator, UserId};
use lan_chat_crypto::{EncryptedData, EncryptedSessionKey};
use serde::{Deserialize, Serialize};

/// Protocol version
pub const PROTOCOL_VERSION: u32 = 1;

/// Protocol messages exchanged between peers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProtocolMessage {
    /// Handshake to establish connection
    Handshake {
        version: u32,
        user_id: UserId,
        public_key: Vec<u8>,
    },

    /// Handshake acknowledgment
    HandshakeAck {
        user_id: UserId,
        public_key: Vec<u8>,
    },

    /// Text/media message (encrypted)
    Message {
        message: Message,
        encrypted_key: Option<EncryptedSessionKey>,
        encrypted_data: Option<EncryptedData>,
    },

    /// Message acknowledgment
    MessageAck {
        message_id: uuid::Uuid,
    },

    /// Message delivery confirmation
    MessageDelivered {
        message_id: uuid::Uuid,
    },

    /// Message read receipt
    MessageRead {
        receipt: ReadReceipt,
    },

    /// Typing indicator
    Typing {
        indicator: TypingIndicator,
    },

    /// Request message history
    HistoryRequest {
        session_id: SessionId,
        before: Option<chrono::DateTime<chrono::Utc>>,
        limit: usize,
    },

    /// Response with message history
    HistoryResponse {
        session_id: SessionId,
        messages: Vec<Message>,
    },

    /// Ping for keep-alive
    Ping,

    /// Pong response
    Pong,

    /// Error message
    Error {
        code: u32,
        message: String,
    },
}

impl ProtocolMessage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(bytes)
    }
}

/// Message frame with length prefix
#[derive(Debug)]
pub struct MessageFrame {
    pub length: u32,
    pub data: Vec<u8>,
}

impl MessageFrame {
    pub fn new(data: Vec<u8>) -> Self {
        let length = data.len() as u32;
        Self { length, data }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(4 + self.data.len());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.data);
        bytes
    }
}
