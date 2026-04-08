use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rand::RngCore;
use std::sync::OnceLock;

static ENCRYPTION_KEY: OnceLock<[u8; 32]> = OnceLock::new();

/// Load or create the encryption key from a file in the app data directory.
/// This avoids OS Keychain issues with unsigned apps (key would change each restart).
fn get_or_create_file_key(dir: &std::path::Path) -> Result<[u8; 32], String> {
    let key_file = dir.join(".key");

    if key_file.exists() {
        let key_b64 = std::fs::read_to_string(&key_file)
            .map_err(|e| format!("Failed to read key file: {}", e))?;
        let key_bytes = STANDARD.decode(key_b64.trim())
            .map_err(|e| format!("Failed to decode key: {}", e))?;
        if key_bytes.len() == 32 {
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&key_bytes);
            return Ok(arr);
        }
    }

    // Generate and persist a new key
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);
    let key_b64 = STANDARD.encode(&key);
    std::fs::write(&key_file, &key_b64)
        .map_err(|e| format!("Failed to save key file: {}", e))?;

    // Restrict to owner read/write only (Unix)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(&key_file) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o600);
            let _ = std::fs::set_permissions(&key_file, perms);
        }
    }

    Ok(key)
}

/// Initialize encryption key from the app data directory (call once at startup)
pub fn init_encryption(app_data_dir: &std::path::Path) -> Result<(), String> {
    let key = get_or_create_file_key(app_data_dir)?;
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
