use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use std::env;

lazy_static::lazy_static! {
    static ref COOKIE_SECRET: [u8; 32] = {
        let secret = env::var("COOKIE_SECRET")
            .expect("COOKIE_SECRET environment variable must be set (32 hex characters)");
        
        if secret.len() != 64 {
            panic!("COOKIE_SECRET must be exactly 64 hex characters (32 bytes)");
        }
        
        let mut key = [0u8; 32];
        hex::decode_to_slice(&secret, &mut key)
            .expect("COOKIE_SECRET must be valid hex");
        key
    };
}

pub fn encrypt_cookie_value(plaintext: &str) -> Result<String, Box<dyn std::error::Error>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&COOKIE_SECRET[..]));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| format!("Encryption failed: {:?}", e))?;
    
    // Combine nonce + ciphertext and base64 encode
    let mut combined = Vec::new();
    combined.extend_from_slice(&nonce);
    combined.extend_from_slice(&ciphertext);
    
    Ok(general_purpose::STANDARD.encode(combined))
}

pub fn decrypt_cookie_value(encrypted: &str) -> Result<String, Box<dyn std::error::Error>> {
    let combined = general_purpose::STANDARD.decode(encrypted)?;
    
    if combined.len() < 12 {
        return Err("Invalid encrypted cookie data".into());
    }
    
    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&COOKIE_SECRET[..]));
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {:?}", e))?;
    
    Ok(String::from_utf8(plaintext)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        // Set test secret
        env::set_var("COOKIE_SECRET", "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef");
        
        let original = r#"{"id":123,"login":"test","avatar_url":"https://example.com","email":null}"#;
        
        let encrypted = encrypt_cookie_value(original).unwrap();
        let decrypted = decrypt_cookie_value(&encrypted).unwrap();
        
        assert_eq!(original, decrypted);
    }
}