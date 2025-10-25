use crate::messages::{MessageFrame, ProtocolMessage};
use lan_chat_core::{ChatError, Result, UserId};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{debug, error};

/// A connection to a peer
pub struct PeerConnection {
    stream: TcpStream,
    peer_id: Option<UserId>,
}

impl PeerConnection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            peer_id: None,
        }
    }

    pub fn peer_id(&self) -> Option<UserId> {
        self.peer_id
    }

    pub fn set_peer_id(&mut self, peer_id: UserId) {
        self.peer_id = Some(peer_id);
    }

    /// Send a protocol message
    pub async fn send_message(&mut self, message: &ProtocolMessage) -> Result<()> {
        let data = message
            .to_bytes()
            .map_err(|e| ChatError::Protocol(e.to_string()))?;

        let frame = MessageFrame::new(data);
        let frame_bytes = frame.to_bytes();

        self.stream
            .write_all(&frame_bytes)
            .await
            .map_err(|e| ChatError::Network(e.to_string()))?;

        self.stream
            .flush()
            .await
            .map_err(|e| ChatError::Network(e.to_string()))?;

        Ok(())
    }

    /// Receive a protocol message
    pub async fn receive_message(&mut self) -> Result<ProtocolMessage> {
        // Read message length (4 bytes, big-endian)
        let length = self
            .stream
            .read_u32()
            .await
            .map_err(|e| ChatError::Network(e.to_string()))?;

        // Validate length
        const MAX_MESSAGE_SIZE: u32 = 10 * 1024 * 1024; // 10 MB
        if length > MAX_MESSAGE_SIZE {
            return Err(ChatError::Protocol(format!(
                "Message too large: {} bytes",
                length
            )));
        }

        // Read message data
        let mut data = vec![0u8; length as usize];
        self.stream
            .read_exact(&mut data)
            .await
            .map_err(|e| ChatError::Network(e.to_string()))?;

        // Parse message
        ProtocolMessage::from_bytes(&data)
            .map_err(|e| ChatError::Protocol(e.to_string()))
    }

    /// Close the connection
    pub async fn close(mut self) -> Result<()> {
        self.stream
            .shutdown()
            .await
            .map_err(|e| ChatError::Network(e.to_string()))
    }
}
