pub mod keypair;
pub mod encryption;
pub mod signature;

pub use keypair::*;
pub use encryption::*;
pub use signature::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Key generation failed: {0}")]
    KeyGenerationFailed(String),

    #[error("Invalid key: {0}")]
    InvalidKey(String),

    #[error("Signature verification failed")]
    SignatureVerificationFailed,

    #[error("RSA error: {0}")]
    RsaError(#[from] rsa::Error),
}

pub type Result<T> = std::result::Result<T, CryptoError>;
