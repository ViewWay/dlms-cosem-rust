# DLMS/COSEM Protocol Stack in Rust

A complete DLMS/COSEM protocol stack implementation in pure Rust, targeting smart meter communication per IEC 62056 standards.

## Features

- **Pure Rust** implementation with minimal dependencies
- **Complete protocol stack**: HDLC framing, AXDR encoding, ASN.1 BER, COSEM interface classes
- **Security**: HLS-ISM, AES-128-GCM (optional), SM4-GCM (pure Rust, Chinese national standard)
- **COSEM Objects**: 15+ interface classes (Clock, Register, Profile Generic, Security Setup, etc.)
- **Client & Server**: Ready-to-use DLMS client and server implementations
- **Transport**: TCP, UDP support with trait-based abstraction

## Architecture

```
dlms-core       # Core types (OBIS, DlmsData, CosemObject trait)
dlms-hdlc       # HDLC frame format, CRC-16, byte stuffing, I/S/U frames
dlms-axdr       # AXDR encoding/decoding for all DLMS data types
dlms-asn1       # ASN.1 BER for AARQ/AARE/RLRQ/RLRE
dlms-security   # Security suites, HLS-ISM, SM4 (pure Rust), AES-128-GCM
dlms-cosem      # COSEM interface classes (IC001-IC107, Profile Generic)
dlms-transport  # Transport trait with TCP/UDP implementations
dlms-client     # DLMS client (associate, get, set, action)
dlms-server     # DLMS server (object registration, request routing)
```

## Supported COSEM Interface Classes

| Class ID | Name | Description |
|----------|------|-------------|
| 1 | Data | Generic data container |
| 3 | Register | Measured value |
| 4 | Extended Register | Register with status and capture time |
| 5 | Demand Register | Demand (power) register |
| 7 | Profile Generic | Load profile / interval data |
| 8 | Clock | Real-time clock |
| 70 | Security Setup | Security configuration |
| 100 | LP Setup | Load profile setup |
| 101 | RS485 Setup | RS485 port configuration |
| 102 | Infrared Setup | Infrared port configuration |
| 106 | NB-IoT Setup | NB-IoT configuration |
| 107 | LoRaWAN Setup | LoRaWAN configuration |

## SM4 Implementation

This crate includes a **pure Rust SM4** block cipher implementation (GB/T 32907-2016), verified against the standard test vector. No external cryptographic library is required for SM4 operations.

## Usage

### Client Example

```rust
use dlms_client::{DlmsClient, ClientConfig};
use dlms_transport::TcpTransport;
use dlms_core::ObisCode;

let transport = TcpTransport::new("192.168.1.100:4059");
let config = ClientConfig::default();
let mut client = DlmsClient::new(transport, config);

// Connect and associate
client.connect()?;
client.associate_hdlc()?;

// Read clock
let datetime = client.get(8, ObisCode::CLOCK, 2)?;

// Disconnect
client.disconnect()?;
```

### Server Example

```rust
use dlms_server::{DlmsServer, ServerConfig};
use dlms_cosem::{Clock, Register};
use dlms_core::{ObisCode, DlmsData};

let mut server = DlmsServer::new(ServerConfig::default());
server.register_object(Box::new(Clock::new(ObisCode::CLOCK)));
server.register_object(Box::new(Register::new(
    ObisCode::ACTIVE_POWER_L1,
    DlmsData::DoubleLong(0),
)));
```

## Testing

```bash
cargo test --lib
```

## License

MIT
