# DLMS/COSEM Protocol Stack in Rust

A complete DLMS/COSEM protocol stack implementation in pure Rust, targeting smart meter communication per IEC 62056 standards.

## Features

- **Pure Rust** implementation with minimal dependencies
- **Complete protocol stack**: HDLC framing, AXDR encoding, ASN.1 BER, COSEM interface classes
- **Security**: HLS-ISM, AES-128-GCM (optional), SM4-GCM/GMAC (pure Rust, Chinese national standard)
- **COSEM Objects**: 40+ interface classes covering metering, communication, and security
- **Client & Server**: Ready-to-use DLMS client and server implementations
- **Transport**: TCP, UDP support with trait-based abstraction
- **no_std** support for embedded targets

## Architecture

```
dlms-core       # Core types (OBIS, CosemDateTime, DlmsData, CosemObject trait)
dlms-hdlc       # HDLC frame format, CRC-16, byte stuffing, I/S/U frames
dlms-axdr       # AXDR encoding/decoding for all DLMS data types
dlms-asn1       # ASN.1 BER for AARQ/AARE/RLRQ/RLRE
dlms-security   # Security suites, HLS-ISM, SM4 (pure Rust), SM4-GCM/GMAC, AES-128-GCM, KDF
dlms-cosem      # COSEM interface classes (40+ IC classes)
dlms-transport  # Transport trait with TCP/UDP implementations
dlms-client     # DLMS client (associate, get, set, action)
dlms-server     # DLMS server (object registration, request routing)
```

## Supported COSEM Interface Classes

### Metering

| Class ID | Name | Description |
|----------|------|-------------|
| 1 | Data | Generic data container |
| 3 | Register | Measured value |
| 4 | Extended Register | Register with status and capture time |
| 5 | Demand Register | Demand (power) register |
| 7 | Profile Generic | Load profile / interval data |
| 8 | Clock | Real-time clock |
| 10 | Demand | Demand monitoring |
| 11 | Special Day Table | Holiday/special day configuration |
| 17 | Billing | Billing data |
| 18 | Tariff Plan | Tariff plan configuration |
| 19 | Tariff Schedule | Tariff schedule |
| 20 | Total | Total energy counter |
| 21 | Week Profile | Weekly tariff profile |
| 22 | Day Profile | Daily tariff profile |
| 31 | Single Phase | Single-phase measurement |
| 34 | Maximum Demand | Maximum demand tracking |
| 35 | Event Log | Event logging |
| 36 | Single Phase MQ | Single-phase measurement quality |

### Communication & Transport

| Class ID | Name | Description |
|----------|------|-------------|
| 23 | IEC HDLC Setup | HDLC communication setup |
| 43 | TCP-UDP Setup | TCP/UDP configuration |
| 44 | IPv4-UDP Setup | IPv4 UDP parameters |
| 45 | IPv6 Setup | IPv6 parameters |
| 46 | IPv4-TCP Setup | IPv4 TCP parameters |
| 47 | IPv6-TCP Setup | IPv6 TCP parameters |
| 56 | M-Bus Slave Setup | M-Bus slave port setup |
| 57 | M-Bus Master Port Setup | M-Bus master port setup |
| 58 | M-Bus Master | M-Bus master control |
| 60 | Serial Port | Serial port configuration |
| 69 | GPRS Modem Setup | GPRS modem configuration |

### Control & Management

| Class ID | Name | Description |
|----------|------|-------------|
| 6 | Supply Disabling | Remote disconnect/reconnect |
| 9 | Local Display | Display control |
| 28 | Clock Control | Clock synchronization control |
| 61 | UPS | Uninterruptible power supply |
| 62 | Auto Connect | Auto-connection management |
| 63 | Direct Disconnect | Direct disconnect control |
| 70 | Disconnect Control | Disconnect state management |
| 72 | Lift Management | Meter lift/shelf management |

### Security & Setup

| Class ID | Name | Description |
|----------|------|-------------|
| 13 | SAP Assignment | SAP assignment |
| 14 | Scheduled Activity | Scheduled task execution |
| 16 | Account | Account management |
| 27 | Status Diag | Status diagnostics |
| 42 | Modem Configuration | Modem setup |
| 88 | Application Context | Application context |
| — | Security Setup | Security configuration |

### Setup & Profile

| Class ID | Name | Description |
|----------|------|-------------|
| 4 | Load Profile | Load profile configuration |
| 71 | Image Transfer | Firmware image transfer |
| 100 | LP Setup | Load profile setup |
| 101 | RS485 Setup | RS485 port configuration |
| 102 | Infrared Setup | Infrared port configuration |
| 106 | NB-IoT Setup | NB-IoT configuration |
| 107 | LoRaWAN Setup | LoRaWAN configuration |

## Cryptography

### SM4 (GB/T 32907-2016)
Pure Rust SM4 block cipher, verified against standard test vectors. No external crypto dependency required.

### SM4-GCM / SM4-GMAC
- SM4-GCM: Authenticated encryption (confidentiality + integrity)
- SM4-GMAC: Authentication-only mode
- Full GF(2^128) GHASH implementation
- Constant-time tag comparison

### AES-128-GCM/GMAC (optional)
Enable with `aes` feature flag for hardware-accelerated AES operations.

### Key Derivation
SM4-based KDF for session key derivation per DLMS/COSEM specification.

## Usage

### COSEM Objects

```rust
use dlms_core::{CosemObject, ObisCode, DlmsData};
use dlms_cosem::{Clock, Register, ProfileGeneric, DisconnectControl};

let clock = Clock::new(ObisCode::CLOCK);
assert_eq!(clock.class_id(), 8);

let register = Register::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(12345));
```

### SM4-GCM Encryption

```rust
use dlms_security::{Sm4Key, sm4_gcm_encrypt, sm4_gcm_decrypt, sm4_gmac, sm4_gmac_verify};

let key = Sm4Key::from([0x42; 16]);
let nonce = [0x01; 12];

// Authenticated encryption
let (ciphertext, tag) = sm4_gcm_encrypt(&key, &nonce, b"secret", b"aad").unwrap();
let plaintext = sm4_gcm_decrypt(&key, &nonce, &ciphertext, &tag, b"aad").unwrap();

// GMAC authentication
let tag = sm4_gmac(&key, &nonce, b"message").unwrap();
sm4_gmac_verify(&key, &nonce, b"message", &tag).unwrap();
```

### Key Derivation

```rust
use dlms_security::kdf;

let master_key = [0x01; 16];
let session_key = kdf(&master_key, b"system-title", 16);
```

## Examples

```bash
cargo run --example cosem_objects   # COSEM IC class usage
cargo run --example sm4_security     # SM4-GCM/GMAC operations
cargo run --example client_basic     # DLMS client flow
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `std` | ✓ | Enable std library support |
| `aes` | ✗ | AES-128-GCM/GMAC via external crypto |

## Testing

```bash
cargo test --workspace --lib    # Unit tests (fast)
cargo test --workspace           # All tests including doc tests
cargo clippy --workspace --lib   # Lint check
cargo fmt --check                # Format check
```

## License

MIT
