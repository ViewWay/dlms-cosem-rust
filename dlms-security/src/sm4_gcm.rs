//! SM4-GCM and SM4-GMAC - Pure Rust implementation
//!
//! Implements GCM (Galois/Counter Mode) and GMAC (authentication-only) using SM4 as the block cipher.
//! Follows NIST SP 800-38D specification with SM4 substituted for AES.

use crate::sm4::{sm4_decrypt, sm4_encrypt, Sm4Block, Sm4Key};

/// SM4-GCM encryption with authentication.
/// Returns (ciphertext, authentication_tag).
pub fn sm4_gcm_encrypt(
    key: &Sm4Key,
    nonce: &[u8; 12],
    plaintext: &[u8],
    aad: &[u8],
) -> Result<(Vec<u8>, [u8; 16]), crate::SecurityError> {
    // H = SM4_K(0^128)
    let zero_block = Sm4Block::from([0u8; 16]);
    let h = sm4_encrypt(key, &zero_block);

    // Compute GHASH over AAD and ciphertext (using pre-encryption counter mode)
    let mut ghash = Ghash::new(h);

    // Process AAD
    if !aad.is_empty() {
        ghash.update_aad(aad);
    }

    // Counter mode encryption: J0 = IV || 0^31 || 1, J1 = inc32(J0), ...
    let j0 = build_j0(nonce);
    let j1 = inc32(j0);

    // Encrypt plaintext using CTR mode starting from J1
    let mut ciphertext = Vec::with_capacity(plaintext.len());
    let mut counter = j1;
    for chunk in plaintext.chunks(16) {
        let keystream = sm4_encrypt(key, &Sm4Block::from(counter));
        for (i, &byte) in chunk.iter().enumerate() {
            ciphertext.push(byte ^ keystream.0[i]);
        }
        counter = inc32(counter);
    }

    // GHASH ciphertext
    if !ciphertext.is_empty() {
        ghash.update_ciphertext(&ciphertext);
    }

    // Finalize: GHASH with lengths block, then XOR with GCTR(J0, H)
    let len_block = build_lengths_block(aad.len(), ciphertext.len());
    ghash.update_final(&len_block);

    let mut tag = ghash.digest();
    let gctr_j0 = sm4_encrypt(key, &Sm4Block::from(j0));
    for i in 0..16 {
        tag[i] ^= gctr_j0.0[i];
    }

    Ok((ciphertext, tag))
}

/// SM4-GCM decryption with authentication verification.
/// Returns decrypted plaintext on success, or SecurityError on auth failure.
pub fn sm4_gcm_decrypt(
    key: &Sm4Key,
    nonce: &[u8; 12],
    ciphertext: &[u8],
    tag: &[u8; 16],
    aad: &[u8],
) -> Result<Vec<u8>, crate::SecurityError> {
    // H = SM4_K(0^128)
    let zero_block = Sm4Block::from([0u8; 16]);
    let h = sm4_encrypt(key, &zero_block);

    // Recompute GHASH
    let mut ghash = Ghash::new(h);

    if !aad.is_empty() {
        ghash.update_aad(aad);
    }
    if !ciphertext.is_empty() {
        ghash.update_ciphertext(ciphertext);
    }

    let len_block = build_lengths_block(aad.len(), ciphertext.len());
    ghash.update_final(&len_block);

    let mut computed_tag = ghash.digest();
    let j0 = build_j0(nonce);
    let gctr_j0 = sm4_encrypt(key, &Sm4Block::from(j0));
    for i in 0..16 {
        computed_tag[i] ^= gctr_j0.0[i];
    }

    // Constant-time tag comparison
    if !constant_time_eq(&computed_tag, tag) {
        return Err(crate::SecurityError::AuthenticationFailed);
    }

    // CTR mode decryption
    let j1 = inc32(j0);
    let mut plaintext = Vec::with_capacity(ciphertext.len());
    let mut counter = j1;
    for chunk in ciphertext.chunks(16) {
        let keystream = sm4_encrypt(key, &Sm4Block::from(counter));
        for (i, &byte) in chunk.iter().enumerate() {
            plaintext.push(byte ^ keystream.0[i]);
        }
        counter = inc32(counter);
    }

    Ok(plaintext)
}

/// SM4-GMAC: authentication-only mode (GCM with empty plaintext).
/// Returns 16-byte authentication tag.
pub fn sm4_gmac(
    key: &Sm4Key,
    nonce: &[u8; 12],
    aad: &[u8],
) -> Result<[u8; 16], crate::SecurityError> {
    let (ciphertext, tag) = sm4_gcm_encrypt(key, nonce, &[], aad)?;
    debug_assert!(ciphertext.is_empty());
    Ok(tag)
}

/// SM4-GMAC tag verification. Returns Ok(()) if tag is valid.
pub fn sm4_gmac_verify(
    key: &Sm4Key,
    nonce: &[u8; 12],
    aad: &[u8],
    expected_tag: &[u8; 16],
) -> Result<(), crate::SecurityError> {
    let computed = sm4_gmac(key, nonce, aad)?;
    if constant_time_eq(&computed, expected_tag) {
        Ok(())
    } else {
        Err(crate::SecurityError::AuthenticationFailed)
    }
}

// --- Internal helpers ---

fn build_j0(nonce: &[u8; 12]) -> [u8; 16] {
    let mut j0 = [0u8; 16];
    j0[0..12].copy_from_slice(nonce);
    j0[15] = 1;
    j0
}

fn inc32(block: [u8; 16]) -> [u8; 16] {
    let mut r = block;
    let val = u32::from_be_bytes([r[12], r[13], r[14], r[15]]);
    let inc = val.wrapping_add(1);
    r[12..16].copy_from_slice(&inc.to_be_bytes());
    r
}

fn build_lengths_block(aad_len: usize, ct_len: usize) -> [u8; 16] {
    let mut block = [0u8; 16];
    let aad_bits = (aad_len as u64) * 8;
    let ct_bits = (ct_len as u64) * 8;
    block[0..8].copy_from_slice(&aad_bits.to_be_bytes());
    block[8..16].copy_from_slice(&ct_bits.to_be_bytes());
    block
}

fn constant_time_eq(a: &[u8; 16], b: &[u8; 16]) -> bool {
    let mut diff: u8 = 0;
    for i in 0..16 {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

/// GF(2^128) multiplication for GHASH.
/// Uses the GCM standard polynomial: x^128 + x^7 + x^2 + x + 1
struct Ghash {
    state: [u8; 16],
    h: Sm4Block,
}

impl Ghash {
    fn new(h: Sm4Block) -> Self {
        Self {
            state: [0u8; 16],
            h,
        }
    }

    fn update_block(&mut self, block: &[u8; 16]) {
        // XOR block into state
        for i in 0..16 {
            self.state[i] ^= block[i];
        }
        // Multiply state by H in GF(2^128)
        self.state = gf128_mul(&self.state, &self.h.0);
    }

    fn update_aad(&mut self, aad: &[u8]) {
        for chunk in aad.chunks(16) {
            let mut block = [0u8; 16];
            block[..chunk.len()].copy_from_slice(chunk);
            self.update_block(&block);
        }
    }

    fn update_ciphertext(&mut self, ct: &[u8]) {
        for chunk in ct.chunks(16) {
            let mut block = [0u8; 16];
            block[..chunk.len()].copy_from_slice(chunk);
            self.update_block(&block);
        }
    }

    fn update_final(&mut self, len_block: &[u8; 16]) {
        self.update_block(len_block);
    }

    fn digest(&self) -> [u8; 16] {
        self.state
    }
}

/// GF(2^128) multiplication using the bit-by-bit algorithm.
/// The irreducible polynomial for GCM is x^128 + x^7 + x^2 + x + 1.
fn gf128_mul(x: &[u8; 16], y: &[u8; 16]) -> [u8; 16] {
    // Use the algorithm from NIST SP 800-38D
    // Represent x as bits, multiply by y in GF(2^128)
    let mut z = [0u8; 16];
    let mut v = *y;

    for byte_idx in 0..16 {
        for bit_idx in 0..8 {
            // Check if bit (byte_idx * 8 + (7 - bit_idx)) of x is set
            if x[byte_idx] & (1 << (7 - bit_idx)) != 0 {
                for i in 0..16 {
                    z[i] ^= v[i];
                }
            }
            // Multiply v by x (right shift in GF)
            let lsb = v[15] & 1;
            // Right shift by 1
            for i in (1..16).rev() {
                v[i] = (v[i] >> 1) | (v[i - 1] << 7);
            }
            v[0] >>= 1;
            // If LSB was set, XOR with R = 0xE1000000000000000000000000000000
            if lsb != 0 {
                v[0] ^= 0xE1;
            }
        }
    }

    z
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> Sm4Key {
        Sm4Key::from([0x01; 16])
    }

    fn test_nonce() -> [u8; 12] {
        [0x42; 12]
    }

    #[test]
    fn test_sm4_gcm_encrypt_decrypt_roundtrip() {
        let key = test_key();
        let nonce = test_nonce();
        let plaintext = b"Hello, DLMS/COSEM SM4-GCM!";
        let aad = b"associated data";

        let (ct, tag) = sm4_gcm_encrypt(&key, &nonce, plaintext, aad).unwrap();
        let pt = sm4_gcm_decrypt(&key, &nonce, &ct, &tag, aad).unwrap();
        assert_eq!(pt, plaintext);
    }

    #[test]
    fn test_sm4_gcm_empty_plaintext() {
        let key = test_key();
        let nonce = test_nonce();
        let (ct, tag) = sm4_gcm_encrypt(&key, &nonce, &[], b"aad").unwrap();
        assert!(ct.is_empty());
        let pt = sm4_gcm_decrypt(&key, &nonce, &ct, &tag, b"aad").unwrap();
        assert!(pt.is_empty());
    }

    #[test]
    fn test_sm4_gcm_empty_aad() {
        let key = test_key();
        let nonce = test_nonce();
        let plaintext = b"test message";
        let (ct, tag) = sm4_gcm_encrypt(&key, &nonce, plaintext, &[]).unwrap();
        let pt = sm4_gcm_decrypt(&key, &nonce, &ct, &tag, &[]).unwrap();
        assert_eq!(pt, plaintext);
    }

    #[test]
    fn test_sm4_gcm_both_empty() {
        let key = test_key();
        let nonce = test_nonce();
        let (ct, tag) = sm4_gcm_encrypt(&key, &nonce, &[], &[]).unwrap();
        assert!(ct.is_empty());
        let pt = sm4_gcm_decrypt(&key, &nonce, &ct, &tag, &[]).unwrap();
        assert!(pt.is_empty());
    }

    #[test]
    fn test_sm4_gcm_wrong_tag() {
        let key = test_key();
        let nonce = test_nonce();
        let plaintext = b"secret";
        let (ct, _tag) = sm4_gcm_encrypt(&key, &nonce, plaintext, b"aad").unwrap();
        let bad_tag = [0xFF; 16];
        let result = sm4_gcm_decrypt(&key, &nonce, &ct, &bad_tag, b"aad");
        assert!(result.is_err());
    }

    #[test]
    fn test_sm4_gcm_wrong_key() {
        let key1 = Sm4Key::from([0x01; 16]);
        let key2 = Sm4Key::from([0x02; 16]);
        let nonce = test_nonce();
        let (ct, tag) = sm4_gcm_encrypt(&key1, &nonce, b"test", b"aad").unwrap();
        let result = sm4_gcm_decrypt(&key2, &nonce, &ct, &tag, b"aad");
        assert!(result.is_err());
    }

    #[test]
    fn test_sm4_gcm_wrong_nonce() {
        let key = test_key();
        let nonce1 = [0x01; 12];
        let nonce2 = [0x02; 12];
        let (ct, tag) = sm4_gcm_encrypt(&key, &nonce1, b"test", b"").unwrap();
        let result = sm4_gcm_decrypt(&key, &nonce2, &ct, &tag, b"");
        assert!(result.is_err());
    }

    #[test]
    fn test_sm4_gcm_wrong_aad() {
        let key = test_key();
        let nonce = test_nonce();
        let (ct, tag) = sm4_gcm_encrypt(&key, &nonce, b"test", b"correct").unwrap();
        let result = sm4_gcm_decrypt(&key, &nonce, &ct, &tag, b"wrong");
        assert!(result.is_err());
    }

    #[test]
    fn test_sm4_gcm_long_message() {
        let key = test_key();
        let nonce = test_nonce();
        let plaintext = vec![0xAB; 256]; // 16 blocks
        let (ct, tag) = sm4_gcm_encrypt(&key, &nonce, &plaintext, b"").unwrap();
        let pt = sm4_gcm_decrypt(&key, &nonce, &ct, &tag, b"").unwrap();
        assert_eq!(pt, plaintext);
    }

    #[test]
    fn test_sm4_gcm_non_aligned() {
        let key = test_key();
        let nonce = test_nonce();
        let plaintext = b"12345"; // 5 bytes, not block-aligned
        let (ct, tag) = sm4_gcm_encrypt(&key, &nonce, plaintext, b"").unwrap();
        assert_eq!(ct.len(), 5);
        let pt = sm4_gcm_decrypt(&key, &nonce, &ct, &tag, b"").unwrap();
        assert_eq!(pt.as_slice(), plaintext);
    }

    // GMAC tests
    #[test]
    fn test_sm4_gmac_basic() {
        let key = test_key();
        let nonce = test_nonce();
        let tag = sm4_gmac(&key, &nonce, b"message to authenticate").unwrap();
        assert_eq!(tag.len(), 16);
    }

    #[test]
    fn test_sm4_gmac_verify_success() {
        let key = test_key();
        let nonce = test_nonce();
        let tag = sm4_gmac(&key, &nonce, b"test message").unwrap();
        sm4_gmac_verify(&key, &nonce, b"test message", &tag).unwrap();
    }

    #[test]
    fn test_sm4_gmac_verify_failure() {
        let key = test_key();
        let nonce = test_nonce();
        let tag = sm4_gmac(&key, &nonce, b"original").unwrap();
        let result = sm4_gmac_verify(&key, &nonce, b"tampered", &tag);
        assert!(result.is_err());
    }

    #[test]
    fn test_sm4_gmac_empty() {
        let key = test_key();
        let nonce = test_nonce();
        let tag = sm4_gmac(&key, &nonce, &[]).unwrap();
        sm4_gmac_verify(&key, &nonce, &[], &tag).unwrap();
    }

    #[test]
    fn test_sm4_gmac_deterministic() {
        let key = test_key();
        let nonce = test_nonce();
        let t1 = sm4_gmac(&key, &nonce, b"msg").unwrap();
        let t2 = sm4_gmac(&key, &nonce, b"msg").unwrap();
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_sm4_gmac_different_keys() {
        let key1 = Sm4Key::from([0x01; 16]);
        let key2 = Sm4Key::from([0x02; 16]);
        let nonce = test_nonce();
        let t1 = sm4_gmac(&key1, &nonce, b"msg").unwrap();
        let t2 = sm4_gmac(&key2, &nonce, b"msg").unwrap();
        assert_ne!(t1, t2);
    }

    // GF(2^128) tests
    #[test]
    fn test_gf128_mul_identity() {
        let zero = [0u8; 16];
        let result = gf128_mul(&zero, &[0x01; 16]);
        assert_eq!(result, zero);
    }

    #[test]
    fn test_inc32() {
        let block = [0u8; 16];
        let inc = inc32(block);
        assert_eq!(inc[0..12], [0u8; 12]);
        assert_eq!(inc[12..16], [0, 0, 0, 1]);
    }

    #[test]
    fn test_inc32_overflow() {
        let mut block = [0u8; 16];
        block[15] = 0xFF;
        let inc = inc32(block);
        assert_eq!(inc[14], 1);
        assert_eq!(inc[15], 0);
    }

    #[test]
    fn test_build_j0() {
        let nonce = [0x01; 12];
        let j0 = build_j0(&nonce);
        assert_eq!(&j0[0..12], &nonce);
        assert_eq!(j0[15], 1);
    }
}
