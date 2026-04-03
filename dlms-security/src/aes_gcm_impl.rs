//! AES-128-GCM encryption/decryption (requires `aes` feature)

use crate::SecurityError;

/// Encrypt with AES-128-GCM
pub fn aes128_gcm_encrypt(
    key: &[u8; 16],
    nonce: &[u8; 12],
    plaintext: &[u8],
    aad: &[u8],
) -> Result<(Vec<u8>, [u8; 16]), SecurityError> {
    use aes_gcm::aead::{Aead, KeyInit};
    use aes_gcm::{Aes128Gcm, Nonce};

    let cipher = Aes128Gcm::new_from_slice(key).map_err(|_| SecurityError::InvalidKey)?;
    let nonce = Nonce::from_slice(nonce);

    let payload = aes_gcm::aead::Payload { msg: plaintext, aad };
    let ciphertext = cipher.encrypt(nonce, payload).map_err(|_| SecurityError::EncryptionFailed)?;

    if ciphertext.len() < 16 {
        return Err(SecurityError::EncryptionFailed);
    }

    let tag_start = ciphertext.len() - 16;
    let mut tag = [0u8; 16];
    tag.copy_from_slice(&ciphertext[tag_start..]);

    Ok((ciphertext[..tag_start].to_vec(), tag))
}

/// Decrypt with AES-128-GCM
pub fn aes128_gcm_decrypt(
    key: &[u8; 16],
    nonce: &[u8; 12],
    ciphertext: &[u8],
    tag: &[u8; 16],
    aad: &[u8],
) -> Result<Vec<u8>, SecurityError> {
    use aes_gcm::aead::{Aead, KeyInit};
    use aes_gcm::{Aes128Gcm, Nonce};

    let cipher = Aes128Gcm::new_from_slice(key).map_err(|_| SecurityError::InvalidKey)?;
    let nonce = Nonce::from_slice(nonce);

    let mut full_ciphertext = ciphertext.to_vec();
    full_ciphertext.extend_from_slice(tag);

    let payload = aes_gcm::aead::Payload { msg: &full_ciphertext, aad };
    let plaintext = cipher.decrypt(nonce, payload).map_err(|_| SecurityError::DecryptionFailed)?;

    Ok(plaintext)
}

/// Compute AES-128-GMAC (authentication only)
pub fn aes128_gmac(
    key: &[u8; 16],
    nonce: &[u8; 12],
    _data: &[u8],
    aad: &[u8],
) -> Result<[u8; 16], SecurityError> {
    let (_, tag) = aes128_gcm_encrypt(key, nonce, &[], aad)?;
    Ok(tag)
}
