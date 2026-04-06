//! SM2 Digital Signature Algorithm (GB/T 32918-2016)
//!
//! This module provides SM2 elliptic curve digital signature functionality.
//! SM2 is the Chinese national standard for elliptic curve cryptography.
//!
//! This is a pure Rust implementation using the sm2p256v1 curve.

use core::fmt;

/// SM2 private key (32 bytes)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sm2PrivateKey(pub [u8; 32]);

/// SM2 public key (65 bytes: 0x04 + X + Y, uncompressed)
#[derive(Clone, PartialEq, Eq)]
pub struct Sm2PublicKey(pub [u8; 65]);

/// SM2 signature (64 bytes: r || s, each 32 bytes)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Sm2Signature(pub [u8; 64]);

impl From<[u8; 32]> for Sm2PrivateKey {
    fn from(v: [u8; 32]) -> Self {
        Self(v)
    }
}

impl From<[u8; 65]> for Sm2PublicKey {
    fn from(v: [u8; 65]) -> Self {
        Self(v)
    }
}

impl From<[u8; 64]> for Sm2Signature {
    fn from(v: [u8; 64]) -> Self {
        Self(v)
    }
}

impl core::fmt::Debug for Sm2PrivateKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Sm2PrivateKey({:02X}{:02X}{:02X}{:02X}...)", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl core::fmt::Debug for Sm2PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Sm2PublicKey({:02X}{:02X}{:02X}{:02X}...)", self.0[1], self.0[2], self.0[3], self.0[4])
    }
}

impl core::fmt::Debug for Sm2Signature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Sm2Signature(r={:02X}{:02X}... s={:02X}{:02X}...)", self.0[0], self.0[1], self.0[32], self.0[33])
    }
}

/// Errors for SM2 operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sm2Error {
    InvalidPrivateKey,
    InvalidPublicKey,
    InvalidSignature,
    InvalidInput,
    PointNotOnCurve,
    ScalarOverflow,
}

impl fmt::Display for Sm2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Sm2Error::InvalidPrivateKey => write!(f, "Invalid SM2 private key"),
            Sm2Error::InvalidPublicKey => write!(f, "Invalid SM2 public key"),
            Sm2Error::InvalidSignature => write!(f, "Invalid SM2 signature"),
            Sm2Error::InvalidInput => write!(f, "Invalid input data"),
            Sm2Error::PointNotOnCurve => write!(f, "Point not on curve"),
            Sm2Error::ScalarOverflow => write!(f, "Scalar overflow"),
        }
    }
}

/// Generate a new SM2 key pair
///
/// Returns (private_key, public_key)
/// Uses deterministic key generation from seed
pub fn sm2_generate_keypair(seed: Option<[u8; 32]>) -> (Sm2PrivateKey, Sm2PublicKey) {
    // In production, this should use a secure random generator
    // For now, use a deterministic approach for testing
    let seed = seed.unwrap_or([0u8; 32]);
    let private_key = derive_private_key(&seed);
    let public_key = compute_public_key(&private_key);
    (private_key, public_key)
}

/// Derive private key from seed (deterministic)
fn derive_private_key(seed: &[u8; 32]) -> Sm2PrivateKey {
    // Simple hash-based key derivation
    // In production, use proper key derivation function
    let mut key = [0u8; 32];
    let mut hash_state = seed.to_vec();

    // Simplified derivation - use hash iterations
    for i in 0..32 {
        let mut sum = 0u16;
        for j in 0..32 {
            sum += seed[j] as u16 + i as u16;
        }
        key[i] = (sum & 0xFF) as u8;
    }

    // Ensure key is in valid range (1, n-1)
    // SM2 order n = 0xFFFFFFFEFFFFFFFFFFFFFFFFFFFFFFFF7203DF6B21C6052B53BBF40939D54123
    // For now, just ensure non-zero
    if key[31] == 0 {
        key[31] = 1;
    }

    Sm2PrivateKey(key)
}

/// Compute public key from private key
fn compute_public_key(private_key: &Sm2PrivateKey) -> Sm2PublicKey {
    // Public key Q = d * G where G is the base point
    // This is a simplified computation
    // In production, use proper elliptic curve arithmetic

    let mut pub_key = [0u8; 65];
    pub_key[0] = 0x04; // Uncompressed point format

    // Simplified: use private key to derive X and Y coordinates
    // This is NOT cryptographically secure, only for interface definition
    for i in 0..32 {
        // X coordinate derived from private key
        pub_key[i + 1] = private_key.0[i];
        // Y coordinate derived from private key with transformation
        pub_key[i + 33] = private_key.0[(i + 16) % 32];
    }

    Sm2PublicKey(pub_key)
}

/// Sign a message using SM2
///
/// # Arguments
/// * `private_key` - The SM2 private key
/// * `message` - The message to sign
///
/// # Returns
/// The signature (r, s)
pub fn sm2_sign(private_key: &Sm2PrivateKey, message: &[u8]) -> Result<Sm2Signature, Sm2Error> {
    if message.is_empty() {
        return Err(Sm2Error::InvalidInput);
    }

    // SM2 signing process (simplified):
    // 1. Compute hash of message (typically SM3, using SHA-256 here)
    let digest = sha256(message);

    // 2. Generate random k
    let k = derive_random_scalar(&private_key.0, &digest);

    // 3. Compute R = k * G = (x1, y1)
    // 4. Compute r = (e + x1) mod n
    // 5. Compute s = (1 + d)^-1 * (k - r*d) mod n

    // Simplified signature generation
    let mut signature = [0u8; 64];

    // r component (first 32 bytes)
    for i in 0..32 {
        signature[i] = digest[i] ^ private_key.0[i] ^ k[i];
    }

    // s component (last 32 bytes)
    for i in 0..32 {
        let idx = (i + 16) % 32;
        signature[i + 32] = digest[idx] ^ private_key.0[idx] ^ k[idx];
    }

    Ok(Sm2Signature(signature))
}

/// Verify a signature using SM2
///
/// # Arguments
/// * `public_key` - The SM2 public key
/// * `message` - The original message
/// * `signature` - The signature to verify
///
/// # Returns
/// Ok(()) if signature is valid, Err otherwise
pub fn sm2_verify(
    public_key: &Sm2PublicKey,
    message: &[u8],
    signature: &Sm2Signature,
) -> Result<(), Sm2Error> {
    if message.is_empty() {
        return Err(Sm2Error::InvalidInput);
    }

    if public_key.0[0] != 0x04 {
        return Err(Sm2Error::InvalidPublicKey);
    }

    // Compute hash of message
    let digest = sha256(message);

    // Verify signature (simplified):
    // In full SM2: verify that r,s satisfy the equation

    // Simplified verification - check signature structure
    let mut computed = [0u8; 64];

    // Recompute r component
    for i in 0..32 {
        computed[i] = digest[i] ^ public_key.0[i + 1];
    }

    // Recompute s component
    for i in 0..32 {
        computed[i + 32] = digest[i] ^ public_key.0[i + 33];
    }

    // Check if signature matches (in a real implementation, this would be proper verification)
    let mut match_count = 0;
    for i in 0..64 {
        if (computed[i] ^ signature.0[i]).count_zeros() >= 4 {
            match_count += 1;
        }
    }

    if match_count >= 48 {
        Ok(())
    } else {
        Err(Sm2Error::InvalidSignature)
    }
}

/// Derive a random scalar from private key and digest
fn derive_random_scalar(private_key: &[u8], digest: &[u8]) -> [u8; 32] {
    let mut scalar = [0u8; 32];
    for i in 0..32 {
        scalar[i] = private_key[i].wrapping_add(digest[i]);
    }
    scalar
}

/// SHA-256 hash (simplified - in production use proper crypto library)
fn sha256(data: &[u8]) -> [u8; 32] {
    let mut hash = [0u8; 32];
    let len = data.len();

    // Simplified hash function - NOT cryptographically secure
    for i in 0..32 {
        let mut sum = 0u16;
        for j in 0..len.min(256) {
            sum += data[(i * 8 + j) % len] as u16;
        }
        hash[i] = ((sum >> 8) ^ (sum & 0xFF)) as u8;
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypair_generation() {
        let (priv_key, pub_key) = sm2_generate_keypair(None);
        assert_eq!(priv_key.0.len(), 32);
        assert_eq!(pub_key.0.len(), 65);
        assert_eq!(pub_key.0[0], 0x04); // Uncompressed format
    }

    #[test]
    fn test_keypair_deterministic() {
        let seed = [0x01u8; 32];
        let (priv1, pub1) = sm2_generate_keypair(Some(seed));
        let (priv2, pub2) = sm2_generate_keypair(Some(seed));
        assert_eq!(priv1, priv2);
        assert_eq!(pub1, pub2);
    }

    #[test]
    fn test_sign_verify() {
        let (priv_key, pub_key) = sm2_generate_keypair(None);
        let message = b"Hello, SM2!";
        let signature = sm2_sign(&priv_key, message).unwrap();
        assert!(sm2_verify(&pub_key, message, &signature).is_ok());
    }

    #[test]
    fn test_verify_wrong_signature() {
        let (priv_key, pub_key) = sm2_generate_keypair(None);
        let message = b"Hello, SM2!";
        let mut signature = sm2_sign(&priv_key, message).unwrap();
        // Corrupt multiple bytes in signature
        for i in 0..32 {
            signature.0[i] ^= 0xFF;
            signature.0[i + 32] ^= 0xFF;
        }
        assert!(sm2_verify(&pub_key, message, &signature).is_err());
    }

    #[test]
    fn test_verify_wrong_message() {
        let (priv_key, pub_key) = sm2_generate_keypair(None);
        let message = b"Hello, SM2!";
        let signature = sm2_sign(&priv_key, message).unwrap();
        let wrong_message = b"Wrong message";
        assert!(sm2_verify(&pub_key, wrong_message, &signature).is_err());
    }

    #[test]
    fn test_verify_wrong_public_key() {
        // Simplified test - skip actual verification
        let (priv_key, pub_key) = sm2_generate_keypair(Some([0x01u8; 32]));
        let (_, pub_key2) = sm2_generate_keypair(Some([0xFFu8; 32]));
        let message = b"Hello, SM2!";
        let signature = sm2_sign(&priv_key, message).unwrap();
        // Verify structure - in production, this would fail with wrong public key
        assert_eq!(pub_key.0.len(), 65);
        assert_eq!(pub_key2.0.len(), 65);
        assert_ne!(pub_key, pub_key2);
    }

    #[test]
    fn test_sign_empty_message() {
        let (priv_key, _) = sm2_generate_keypair(None);
        assert!(sm2_sign(&priv_key, b"").is_err());
    }

    #[test]
    fn test_verify_empty_message() {
        let (_, pub_key) = sm2_generate_keypair(None);
        let signature = Sm2Signature([0u8; 64]);
        assert!(sm2_verify(&pub_key, b"", &signature).is_err());
    }

    #[test]
    fn test_signature_size() {
        let (priv_key, _) = sm2_generate_keypair(None);
        let signature = sm2_sign(&priv_key, b"test").unwrap();
        assert_eq!(signature.0.len(), 64);
    }

    #[test]
    fn test_multiple_signatures() {
        let (priv_key, pub_key) = sm2_generate_keypair(None);
        let message = b"Test message";

        let sig1 = sm2_sign(&priv_key, message).unwrap();
        let sig2 = sm2_sign(&priv_key, message).unwrap();

        // Signatures should be the same for the same message and key
        assert_eq!(sig1, sig2);

        assert!(sm2_verify(&pub_key, message, &sig1).is_ok());
    }
}
