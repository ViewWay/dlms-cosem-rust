//! dlms-security: Security for DLMS/COSEM
//!
//! - Security Suite 0~5
//! - HLS-ISM (High Level Security) authentication
//! - AES-128-GCM encryption/decryption (optional, behind `aes` feature)
//! - SM4-GCM / SM4-GMAC (pure Rust implementation)
//! - KDF key derivation
//! - Key management

#![cfg_attr(not(feature = "std"), no_std)]

// no_std support: feature gate for no_std

use core::fmt;

mod hls;
mod kdf;
mod key_management;
mod sm4;
mod sm4_gcm;
mod suite;

pub use hls::{HlsAuthResult, HlsContext, HlsStep};
pub use kdf::{kdf, kdf_gmac};
pub use key_management::KeyManagement;
pub use sm4::{sm4_decrypt, sm4_encrypt, Sm4Block, Sm4Key};
pub use sm4_gcm::{sm4_gcm_decrypt, sm4_gcm_encrypt, sm4_gmac, sm4_gmac_verify};
pub use suite::SecuritySuite;

#[cfg(feature = "aes")]
mod aes_gcm_impl;

#[cfg(feature = "aes")]
pub use aes_gcm_impl::{aes128_gcm_decrypt, aes128_gcm_encrypt, aes128_gmac};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityError {
    EncryptionFailed,
    DecryptionFailed,
    AuthenticationFailed,
    InvalidKey,
    InvalidData,
    InvalidNonce,
    InvalidTag,
    UnsupportedSuite(u8),
    KdfError,
}

#[cfg(feature = "std")]
impl std::error::Error for SecurityError {}

impl fmt::Display for SecurityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityError::EncryptionFailed => write!(f, "Encryption failed"),
            SecurityError::DecryptionFailed => write!(f, "Decryption failed"),
            SecurityError::AuthenticationFailed => write!(f, "Authentication failed"),
            SecurityError::InvalidKey => write!(f, "Invalid key"),
            SecurityError::InvalidData => write!(f, "Invalid data"),
            SecurityError::InvalidNonce => write!(f, "Invalid nonce"),
            SecurityError::InvalidTag => write!(f, "Invalid authentication tag"),
            SecurityError::UnsupportedSuite(s) => write!(f, "Unsupported security suite: {s}"),
            SecurityError::KdfError => write!(f, "Key derivation error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // SM4 tests (pure Rust, always available)
    #[test]
    fn test_sm4_encrypt_decrypt() {
        let key = Sm4Key::from([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54,
            0x32, 0x10,
        ]);
        let plaintext = Sm4Block::from([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54,
            0x32, 0x10,
        ]);
        let ciphertext = sm4_encrypt(&key, &plaintext);
        let decrypted = sm4_decrypt(&key, &ciphertext);
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_sm4_all_zeros() {
        let key = Sm4Key::from([0u8; 16]);
        let plaintext = Sm4Block::from([0u8; 16]);
        let ciphertext = sm4_encrypt(&key, &plaintext);
        let decrypted = sm4_decrypt(&key, &ciphertext);
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_sm4_all_ones() {
        let key = Sm4Key::from([0xFF; 16]);
        let plaintext = Sm4Block::from([0xFF; 16]);
        let ciphertext = sm4_encrypt(&key, &plaintext);
        let decrypted = sm4_decrypt(&key, &ciphertext);
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_sm4_deterministic() {
        let key = Sm4Key::from([0x42; 16]);
        let plaintext = Sm4Block::from([0x13; 16]);
        let c1 = sm4_encrypt(&key, &plaintext);
        let c2 = sm4_encrypt(&key, &plaintext);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_sm4_different_keys() {
        let key1 = Sm4Key::from([0x01; 16]);
        let key2 = Sm4Key::from([0x02; 16]);
        let plaintext = Sm4Block::from([0xAA; 16]);
        let c1 = sm4_encrypt(&key1, &plaintext);
        let c2 = sm4_encrypt(&key2, &plaintext);
        assert_ne!(c1, c2);
    }

    #[test]
    fn test_sm4_different_plaintexts() {
        let key = Sm4Key::from([0x01; 16]);
        let p1 = Sm4Block::from([0x01; 16]);
        let p2 = Sm4Block::from([0x02; 16]);
        let c1 = sm4_encrypt(&key, &p1);
        let c2 = sm4_encrypt(&key, &p2);
        assert_ne!(c1, c2);
    }

    #[test]
    fn test_sm4_key_from() {
        let key = Sm4Key::from([0u8; 16]);
        assert_eq!(key.0.len(), 16);
    }

    #[test]
    fn test_sm4_block_from() {
        let block = Sm4Block::from([0u8; 16]);
        assert_eq!(block.0.len(), 16);
    }

    #[test]
    fn test_sm4_known_vector() {
        // SM4 test vector from GB/T 32907-2016
        let key = Sm4Key::from([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54,
            0x32, 0x10,
        ]);
        let plaintext = Sm4Block::from([
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54,
            0x32, 0x10,
        ]);
        let expected = Sm4Block::from([
            0x68, 0x1E, 0xDF, 0x34, 0xD2, 0x06, 0x96, 0x5E, 0x86, 0xB3, 0xE9, 0x4F, 0x53, 0x6E,
            0x42, 0x46,
        ]);
        let ciphertext = sm4_encrypt(&key, &plaintext);
        assert_eq!(ciphertext, expected);
    }

    // KDF tests
    #[test]
    fn test_kdf_basic() {
        let key = [0x01; 16];
        let result = kdf(&key, b"test-id", 16);
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn test_kdf_deterministic() {
        let key = [0x42; 16];
        let r1 = kdf(&key, b"session", 16);
        let r2 = kdf(&key, b"session", 16);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_kdf_different_ids() {
        let key = [0x01; 16];
        let r1 = kdf(&key, b"id1", 16);
        let r2 = kdf(&key, b"id2", 16);
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_kdf_different_lengths() {
        let key = [0x01; 16];
        let r16 = kdf(&key, b"test", 16);
        let r32 = kdf(&key, b"test", 32);
        assert_eq!(r16.len(), 16);
        assert_eq!(r32.len(), 32);
        assert_eq!(r16, &r32[..16]);
    }

    // Security suite tests
    #[test]
    fn test_security_suite_from_id() {
        assert_eq!(SecuritySuite::from_id(0), Some(SecuritySuite::Suite0));
        assert_eq!(SecuritySuite::from_id(1), Some(SecuritySuite::Suite1));
        assert_eq!(SecuritySuite::from_id(2), Some(SecuritySuite::Suite2));
        assert_eq!(SecuritySuite::from_id(5), Some(SecuritySuite::Suite5));
        assert_eq!(SecuritySuite::from_id(99), None);
    }

    #[test]
    fn test_security_suite_needs_authentication() {
        assert!(!SecuritySuite::Suite0.needs_authentication());
        assert!(SecuritySuite::Suite1.needs_authentication());
        assert!(SecuritySuite::Suite5.needs_authentication());
    }

    #[test]
    fn test_security_suite_needs_encryption() {
        assert!(!SecuritySuite::Suite0.needs_encryption());
        assert!(!SecuritySuite::Suite1.needs_encryption());
        assert!(SecuritySuite::Suite2.needs_encryption());
        assert!(SecuritySuite::Suite5.needs_encryption());
    }

    // HLS tests
    #[test]
    fn test_hls_context_new() {
        let ctx = HlsContext::new(SecuritySuite::Suite0);
        assert_eq!(ctx.suite(), SecuritySuite::Suite0);
    }

    #[test]
    fn test_hls_step_initial() {
        let ctx = HlsContext::new(SecuritySuite::Suite0);
        assert_eq!(ctx.step(), HlsStep::Idle);
    }

    #[test]
    fn test_hls_context_with_key() {
        let mut ctx = HlsContext::new(SecuritySuite::Suite1);
        ctx.set_key([0x01; 16]);
        assert!(ctx.has_key());
    }

    // Key management tests
    #[test]
    fn test_key_management_new() {
        let km = KeyManagement::new();
        assert!(km.get_key(0).is_none());
    }

    #[test]
    fn test_key_management_set_get() {
        let mut km = KeyManagement::new();
        km.set_key(1, [0x01; 16]);
        let key = km.get_key(1).unwrap();
        assert_eq!(key, &[0x01; 16]);
    }

    #[test]
    fn test_key_management_remove() {
        let mut km = KeyManagement::new();
        km.set_key(1, [0x01; 16]);
        km.remove_key(1);
        assert!(km.get_key(1).is_none());
    }

    #[test]
    fn test_key_management_multiple_keys() {
        let mut km = KeyManagement::new();
        km.set_key(0, [0x00; 16]);
        km.set_key(1, [0x01; 16]);
        km.set_key(2, [0x02; 16]);
        assert_eq!(km.key_count(), 3);
    }

    // Error tests
    #[test]
    fn test_security_error_display() {
        let err = SecurityError::EncryptionFailed;
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_security_error_unsupported_suite() {
        let err = SecurityError::UnsupportedSuite(99);
        let s = format!("{err}");
        assert!(s.contains("99"));
    }

    // AES-GCM tests (only when feature is enabled)
    #[cfg(feature = "aes")]
    #[test]
    fn test_aes128_gcm_encrypt_decrypt() {
        let key = [0x42; 16];
        let nonce = [0x01; 12];
        let plaintext = b"Hello, DLMS!";
        let (ciphertext, tag) = aes128_gcm_encrypt(&key, &nonce, plaintext, b"").unwrap();
        let decrypted = aes128_gcm_decrypt(&key, &nonce, &ciphertext, &tag, b"").unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[cfg(feature = "aes")]
    #[test]
    fn test_aes128_gmac() {
        let key = [0x42; 16];
        let nonce = [0x01; 12];
        let tag = aes128_gmac(&key, &nonce, b"test message", b"").unwrap();
        assert_eq!(tag.len(), 16);
    }

    #[cfg(feature = "aes")]
    #[test]
    fn test_aes128_gcm_with_aad() {
        let key = [0x01; 16];
        let nonce = [0x02; 12];
        let aad = b"associated data";
        let (ct, tag) = aes128_gcm_encrypt(&key, &nonce, b"secret", aad).unwrap();
        let pt = aes128_gcm_decrypt(&key, &nonce, &ct, &tag, aad).unwrap();
        assert_eq!(pt, b"secret");
    }

    #[cfg(feature = "aes")]
    #[test]
    fn test_aes128_gcm_wrong_key() {
        let key1 = [0x01; 16];
        let key2 = [0x02; 16];
        let nonce = [0x01; 12];
        let (ct, tag) = aes128_gcm_encrypt(&key1, &nonce, b"test", b"").unwrap();
        let result = aes128_gcm_decrypt(&key2, &nonce, &ct, &tag, b"");
        assert!(result.is_err());
    }

    #[cfg(feature = "aes")]
    #[test]
    fn test_aes128_gcm_empty_plaintext() {
        let key = [0x01; 16];
        let nonce = [0x01; 12];
        let (ct, tag) = aes128_gcm_encrypt(&key, &nonce, b"", b"").unwrap();
        assert!(ct.is_empty());
        let pt = aes128_gcm_decrypt(&key, &nonce, &ct, &tag, b"").unwrap();
        assert!(pt.is_empty());
    }
}
