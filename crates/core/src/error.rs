use thiserror::Error;

#[derive(Error, Debug)]
pub enum ChatError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Peer not found: {0}")]
    PeerNotFound(String),

    #[error("File transfer error: {0}")]
    FileTransfer(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("Crypto error: {0}")]
    Crypto(String),
}

pub type Result<T> = std::result::Result<T, ChatError>;
