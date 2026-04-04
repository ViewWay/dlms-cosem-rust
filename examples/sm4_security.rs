//! Example: SM4-GCM encryption and SM4-GMAC authentication
//!
//! Demonstrates using the pure Rust SM4 cryptographic primitives.

use dlms_security::{Sm4Key, sm4_encrypt, sm4_decrypt, Sm4Block, sm4_gcm_encrypt, sm4_gcm_decrypt, sm4_gmac, sm4_gmac_verify, kdf};

fn main() {
    // --- SM4 block cipher ---
    println!("=== SM4 Block Cipher ===");
    let key = Sm4Key::from([
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
        0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10,
    ]);
    let plaintext = Sm4Block::from([
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
        0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10,
    ]);
    let ciphertext = sm4_encrypt(&key, &plaintext);
    let decrypted = sm4_decrypt(&key, &ciphertext);
    println!("Plaintext:  {:?}", plaintext);
    println!("Ciphertext: {:?}", ciphertext);
    println!("Decrypted:  {:?}", decrypted);
    println!("Match: {}", plaintext == decrypted);

    // --- SM4-GCM ---
    println!("\n=== SM4-GCM Encryption ===");
    let key2 = Sm4Key::from([0x42; 16]);
    let nonce: [u8; 12] = [0x01; 12];
    let message = b"Hello, DLMS/COSEM!";
    let aad = b"authentication data";

    let (ct, tag) = sm4_gcm_encrypt(&key2, &nonce, message, aad).unwrap();
    println!("Plaintext: {}", String::from_utf8_lossy(message));
    println!("Ciphertext: {:02X?}", ct);
    println!("Auth tag:   {:02X?}", tag);

    let pt = sm4_gcm_decrypt(&key2, &nonce, &ct, &tag, aad).unwrap();
    println!("Decrypted:  {}", String::from_utf8_lossy(&pt));

    // --- SM4-GMAC ---
    println!("\n=== SM4-GMAC Authentication ===");
    let gmac_tag = sm4_gmac(&key2, &nonce, b"message to sign").unwrap();
    println!("GMAC tag: {:02X?}", gmac_tag);

    let verified = sm4_gmac_verify(&key2, &nonce, b"message to sign", &gmac_tag);
    println!("Verify (correct): {}", verified.is_ok());

    let tampered = sm4_gmac_verify(&key2, &nonce, b"tampered message", &gmac_tag);
    println!("Verify (tampered): {}", tampered.is_err());

    // --- KDF ---
    println!("\n=== Key Derivation ===");
    let derived = kdf(&[0x01; 16], b"system-title", 32);
    println!("KDF derived key (32 bytes): {:02X?}", derived);

    println!("\nAll security operations completed successfully!");
}
