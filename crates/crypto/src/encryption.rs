use crate::{CryptoError, Result};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use rand::RngCore;
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

const AES_KEY_SIZE: usize = 32; // 256 bits
const NONCE_SIZE: usize = 12; // 96 bits for GCM

/// Encrypted data with nonce
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}

/// Encrypted session key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedSessionKey {
    pub encrypted_key: Vec<u8>,
}

/// AES-GCM encryption engine
pub struct AesEncryption {
    key: [u8; AES_KEY_SIZE],
}

impl AesEncryption {
    /// Create a new AES encryption engine with a random key
    pub fn new() -> Self {
        let mut key = [0u8; AES_KEY_SIZE];
        rand::thread_rng().fill_bytes(&mut key);
        Self { key }
    }

    /// Create from an existing key
    pub fn from_key(key: [u8; AES_KEY_SIZE]) -> Self {
        Self { key }
    }

    /// Get the encryption key
    pub fn key(&self) -> &[u8; AES_KEY_SIZE] {
        &self.key
    }

    /// Encrypt data using AES-256-GCM
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<EncryptedData> {
        let key = Key::<Aes256Gcm>::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);

        // Generate random nonce
        let mut nonce_bytes = [0u8; NONCE_SIZE];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

        Ok(EncryptedData {
            ciphertext,
            nonce: nonce_bytes.to_vec(),
        })
    }

    /// Decrypt data using AES-256-GCM
    pub fn decrypt(&self, encrypted: &EncryptedData) -> Result<Vec<u8>> {
        let key = Key::<Aes256Gcm>::from_slice(&self.key);
        let cipher = Aes256Gcm::new(key);

        if encrypted.nonce.len() != NONCE_SIZE {
            return Err(CryptoError::DecryptionFailed("Invalid nonce size".into()));
        }

        let nonce = Nonce::from_slice(&encrypted.nonce);

        cipher
            .decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
    }
}

impl Default for AesEncryption {
    fn default() -> Self {
        Self::new()
    }
}

/// RSA encryption utilities
pub struct RsaEncryption;

impl RsaEncryption {
    /// Encrypt a session key with RSA public key
    pub fn encrypt_session_key(
        public_key: &RsaPublicKey,
        session_key: &[u8; AES_KEY_SIZE],
    ) -> Result<EncryptedSessionKey> {
        let mut rng = rand::thread_rng();
        let padding = Oaep::new::<Sha256>();

        let encrypted_key = public_key
            .encrypt(&mut rng, padding, session_key)
            .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))?;

        Ok(EncryptedSessionKey { encrypted_key })
    }

    /// Decrypt a session key with RSA private key
    pub fn decrypt_session_key(
        private_key: &RsaPrivateKey,
        encrypted: &EncryptedSessionKey,
    ) -> Result<[u8; AES_KEY_SIZE]> {
        let padding = Oaep::new::<Sha256>();

        let decrypted = private_key
            .decrypt(padding, &encrypted.encrypted_key)
            .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))?;

        if decrypted.len() != AES_KEY_SIZE {
            return Err(CryptoError::DecryptionFailed(
                "Invalid key size after decryption".into(),
            ));
        }

        let mut key = [0u8; AES_KEY_SIZE];
        key.copy_from_slice(&decrypted);
        Ok(key)
    }
}

/// Hybrid encryption: RSA for key exchange, AES for data
pub struct HybridEncryption;

impl HybridEncryption {
    /// Encrypt data with a recipient's public key
    pub fn encrypt(
        recipient_public_key: &RsaPublicKey,
        plaintext: &[u8],
    ) -> Result<(EncryptedSessionKey, EncryptedData)> {
        // Generate random session key
        let aes = AesEncryption::new();

        // Encrypt data with AES
        let encrypted_data = aes.encrypt(plaintext)?;

        // Encrypt session key with RSA
        let encrypted_key = RsaEncryption::encrypt_session_key(recipient_public_key, aes.key())?;

        Ok((encrypted_key, encrypted_data))
    }

    /// Decrypt data with private key
    pub fn decrypt(
        private_key: &RsaPrivateKey,
        encrypted_key: &EncryptedSessionKey,
        encrypted_data: &EncryptedData,
    ) -> Result<Vec<u8>> {
        // Decrypt session key with RSA
        let session_key = RsaEncryption::decrypt_session_key(private_key, encrypted_key)?;

        // Decrypt data with AES
        let aes = AesEncryption::from_key(session_key);
        aes.decrypt(encrypted_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::KeyPair;

    #[test]
    fn test_aes_encryption() {
        let aes = AesEncryption::new();
        let plaintext = b"Hello, World!";

        let encrypted = aes.encrypt(plaintext).unwrap();
        let decrypted = aes.decrypt(&encrypted).unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }

    #[test]
    fn test_hybrid_encryption() {
        let keypair = KeyPair::generate().unwrap();
        let plaintext = b"Secret message";

        let (encrypted_key, encrypted_data) =
            HybridEncryption::encrypt(keypair.public_key(), plaintext).unwrap();

        let decrypted =
            HybridEncryption::decrypt(keypair.private_key(), &encrypted_key, &encrypted_data)
                .unwrap();

        assert_eq!(plaintext, &decrypted[..]);
    }
}
