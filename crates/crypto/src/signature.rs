use crate::{CryptoError, Result};
use ring::signature::{self, KeyPair as RingKeyPair};
use serde::{Deserialize, Serialize};

/// Digital signature for message authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub signature: Vec<u8>,
}

/// Ed25519 signature utilities
pub struct MessageSigner {
    key_pair: signature::Ed25519KeyPair,
}

impl MessageSigner {
    /// Generate a new signing key pair
    pub fn generate() -> Result<Self> {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|_| CryptoError::KeyGenerationFailed("Ed25519 generation failed".into()))?;

        let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|_| CryptoError::KeyGenerationFailed("Key parsing failed".into()))?;

        Ok(Self { key_pair })
    }

    /// Sign a message
    pub fn sign(&self, message: &[u8]) -> Signature {
        let sig = self.key_pair.sign(message);
        Signature {
            signature: sig.as_ref().to_vec(),
        }
    }

    /// Get the public key
    pub fn public_key(&self) -> Vec<u8> {
        self.key_pair.public_key().as_ref().to_vec()
    }

    /// Verify a signature
    pub fn verify(public_key: &[u8], message: &[u8], signature: &Signature) -> Result<()> {
        let public_key =
            signature::UnparsedPublicKey::new(&signature::ED25519, public_key);

        public_key
            .verify(message, &signature.signature)
            .map_err(|_| CryptoError::SignatureVerificationFailed)
    }
}
