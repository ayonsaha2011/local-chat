use crate::{CryptoError, Result};
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey,
};
use serde::{Deserialize, Serialize};

const KEY_SIZE: usize = 2048;

/// RSA key pair for asymmetric encryption
#[derive(Clone)]
pub struct KeyPair {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl KeyPair {
    /// Generate a new RSA key pair
    pub fn generate() -> Result<Self> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, KEY_SIZE)
            .map_err(|e| CryptoError::KeyGenerationFailed(e.to_string()))?;
        let public_key = RsaPublicKey::from(&private_key);

        Ok(Self {
            private_key,
            public_key,
        })
    }

    /// Get the public key
    pub fn public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }

    /// Get the private key
    pub fn private_key(&self) -> &RsaPrivateKey {
        &self.private_key
    }

    /// Export public key as PEM
    pub fn export_public_key_pem(&self) -> Result<String> {
        self.public_key
            .to_public_key_pem(LineEnding::LF)
            .map_err(|e| CryptoError::InvalidKey(e.to_string()))
    }

    /// Export private key as PEM
    pub fn export_private_key_pem(&self) -> Result<String> {
        self.private_key
            .to_pkcs8_pem(LineEnding::LF)
            .map_err(|e| CryptoError::InvalidKey(e.to_string()))
            .map(|s| s.to_string())
    }

    /// Import public key from PEM
    pub fn import_public_key_pem(pem: &str) -> Result<RsaPublicKey> {
        RsaPublicKey::from_public_key_pem(pem)
            .map_err(|e| CryptoError::InvalidKey(e.to_string()))
    }

    /// Import private key from PEM
    pub fn import_private_key_pem(pem: &str) -> Result<RsaPrivateKey> {
        RsaPrivateKey::from_pkcs8_pem(pem)
            .map_err(|e| CryptoError::InvalidKey(e.to_string()))
    }

    /// Export public key as bytes
    pub fn export_public_key_bytes(&self) -> Result<Vec<u8>> {
        let pem = self.export_public_key_pem()?;
        Ok(pem.into_bytes())
    }

    /// Import public key from bytes
    pub fn import_public_key_bytes(bytes: &[u8]) -> Result<RsaPublicKey> {
        let pem = String::from_utf8(bytes.to_vec())
            .map_err(|e| CryptoError::InvalidKey(e.to_string()))?;
        Self::import_public_key_pem(&pem)
    }
}

/// Serializable public key for network transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyData {
    pub key_pem: String,
}

impl PublicKeyData {
    pub fn from_keypair(keypair: &KeyPair) -> Result<Self> {
        Ok(Self {
            key_pem: keypair.export_public_key_pem()?,
        })
    }

    pub fn to_public_key(&self) -> Result<RsaPublicKey> {
        KeyPair::import_public_key_pem(&self.key_pem)
    }
}
