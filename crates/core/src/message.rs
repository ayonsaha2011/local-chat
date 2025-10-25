use crate::{SessionId, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Message types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    Text,
    Image,
    File,
    Audio,
    Video,
    System,
}

/// Message status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageStatus {
    Sending,
    Sent,
    Delivered,
    Read,
    Failed,
}

/// A chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Message {
    pub fn new_text(
        session_id: SessionId,
        sender_id: UserId,
        recipient_id: UserId,
        content: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            session_id,
            sender_id,
            recipient_id,
            message_type: MessageType::Text,
            content,
            metadata: None,
            timestamp: Utc::now(),
            status: MessageStatus::Sending,
            encrypted: false,
        }
    }

    pub fn new_file(
        session_id: SessionId,
        sender_id: UserId,
        recipient_id: UserId,
        file_name: String,
        file_size: u64,
        file_hash: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            session_id,
            sender_id,
            recipient_id,
            message_type: MessageType::File,
            content: file_name.clone(),
            metadata: Some(MessageMetadata::File {
                file_name,
                file_size,
                file_hash,
            }),
            timestamp: Utc::now(),
            status: MessageStatus::Sending,
            encrypted: false,
        }
    }
}

/// Additional message metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageMetadata {
    File {
        file_name: String,
        file_size: u64,
        file_hash: String,
    },
    Image {
        width: u32,
        height: u32,
        thumbnail: Option<String>,
    },
    Audio {
        duration: u32,
    },
    Video {
        duration: u32,
        thumbnail: Option<String>,
    },
}

/// Typing indicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingIndicator {
    pub user_id: UserId,
    pub session_id: SessionId,
    pub is_typing: bool,
}

/// Read receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadReceipt {
    pub message_id: Uuid,
    pub user_id: UserId,
    pub timestamp: DateTime<Utc>,
}
