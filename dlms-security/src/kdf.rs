//! KDF (Key Derivation Function) for DLMS/COSEM

use crate::sm4::{sm4_encrypt, Sm4Block, Sm4Key};

/// Simple KDF based on SM4 (for environments without external crypto)
/// Uses a counter-mode construction with SM4 as PRF
pub fn kdf(key: &[u8; 16], id: &[u8], length: usize) -> Vec<u8> {
    let mut result = Vec::with_capacity(length);
    let mut counter: u32 = 1;

    while result.len() < length {
        // counter(4) || id(variable) padded to 16 bytes
        let mut block = [0u8; 16];
        block[0..4].copy_from_slice(&counter.to_be_bytes());
        let id_len = id.len().min(12);
        block[4..4 + id_len].copy_from_slice(&id[..id_len]);

        let key = Sm4Key::from(*key);
        let input = Sm4Block::from(block);
        let output = sm4_encrypt(&key, &input);
        result.extend_from_slice(&output.0);

        counter += 1;
    }

    result.truncate(length);
    result
}

/// KDF for GMAC key derivation
pub fn kdf_gmac(key: &[u8; 16], system_title: &[u8], length: usize) -> Vec<u8> {
    kdf(key, system_title, length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kdf_basic() {
        let key = [0x01; 16];
        let result = kdf(&key, b"test", 16);
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn test_kdf_32_bytes() {
        let key = [0x01; 16];
        let result = kdf(&key, b"test", 32);
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_kdf_deterministic() {
        let key = [0x42; 16];
        let r1 = kdf(&key, b"id", 16);
        let r2 = kdf(&key, b"id", 16);
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_kdf_different_keys() {
        let r1 = kdf(&[0x01; 16], b"id", 16);
        let r2 = kdf(&[0x02; 16], b"id", 16);
        assert_ne!(r1, r2);
    }

    #[test]
    fn test_kdf_gmac() {
        let key = [0x01; 16];
        let st = b"SYS_TITLE";
        let result = kdf_gmac(&key, st, 16);
        assert_eq!(result.len(), 16);
    }
}
