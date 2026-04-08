use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use keyring::Entry;
use rand::RngCore;
use std::sync::OnceLock;

const KEYRING_SERVICE: &str = "com.boussole.app";
const KEYRING_ENCRYPTION_KEY: &str = "encryption_key";

static ENCRYPTION_KEY: OnceLock<[u8; 32]> = OnceLock::new();

/// Initialize or retrieve the encryption key from OS keyring
fn get_or_create_encryption_key() -> Result<[u8; 32], String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_ENCRYPTION_KEY)
        .map_err(|e| format!("Failed to access keyring: {}", e))?;

    // Try to get existing key
    if let Ok(key_b64) = entry.get_password() {
        let key = STANDARD.decode(&key_b64)
            .map_err(|e| format!("Failed to decode key: {}", e))?;
        if key.len() == 32 {
            let mut key_array = [0u8; 32];
            key_array.copy_from_slice(&key);
            return Ok(key_array);
        }
    }

    // Generate new key if none exists
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    
    // Store in keyring
    let key_b64 = STANDARD.encode(&key);
    entry.set_password(&key_b64)
        .map_err(|e| format!("Failed to store key: {}", e))?;

    Ok(key)
}

/// Initialize encryption key (call once at startup)
pub fn init_encryption() -> Result<(), String> {
    let key = get_or_create_encryption_key()?;
    ENCRYPTION_KEY.set(key)
        .map_err(|_| "Encryption key already initialized".to_string())
}

/// Encrypt plaintext data
pub fn encrypt(plaintext: &str) -> Result<String, String> {
    let key = ENCRYPTION_KEY.get()
        .ok_or("Encryption not initialized")?;

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| format!("Failed to create cipher: {}", e))?;

    // Generate random nonce
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt
    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    // Combine nonce + ciphertext and encode
    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(STANDARD.encode(&result))
}

/// Decrypt ciphertext data
pub fn decrypt(ciphertext_b64: &str) -> Result<String, String> {
    let key = ENCRYPTION_KEY.get()
        .ok_or("Encryption not initialized")?;

    let data = STANDARD.decode(ciphertext_b64)
        .map_err(|e| format!("Failed to decode: {}", e))?;

    if data.len() < 12 {
        return Err("Invalid ciphertext: too short".to_string());
    }

    let nonce = Nonce::from_slice(&data[..12]);
    let ciphertext = &data[12..];

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|e| format!("Failed to create cipher: {}", e))?;

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    String::from_utf8(plaintext)
        .map_err(|e| format!("Invalid UTF-8: {}", e))
}

/// Encrypt a JSON-serializable value
pub fn encrypt_json<T: serde::Serialize>(value: &T) -> Result<String, String> {
    let json = serde_json::to_string(value)
        .map_err(|e| format!("JSON serialization failed: {}", e))?;
    encrypt(&json)
}

/// Decrypt to a JSON-deserializable value
pub fn decrypt_json<T: serde::de::DeserializeOwned>(ciphertext: &str) -> Result<T, String> {
    let json = decrypt(ciphertext)?;
    serde_json::from_str(&json)
        .map_err(|e| format!("JSON deserialization failed: {}", e))
}
