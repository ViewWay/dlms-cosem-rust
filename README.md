# dlms-cosem-rust

DLMS/COSEM protocol stack in pure Rust — Blue Book (IEC 62056-53) implementation.

## Overview

| Crate | Description |
|--------|-------------|
| [dlms-core](./dlms-core/) | Core types (OBIS, DateTime, AccessResult) |
| [dlms-hdlc](./dlms-hdlc/) | HDLC framing (IEC 62056-53) |
| [dlms-axdr](./dlms-axdr/) | AXDR encoding/decoding |
| [dlms-asn1](./dlms-asn1/) | ASN.1 BER encoding/decoding |
| [dlms-security](./dlms-security/) | SM4 encryption (ECB, GCM, GMAC) |
| [dlms-cosem](./dlms-cosem/) | COSEM interface classes (130 IC classes) |
| [dlms-client](./dlms-client/) | DLMS client (GET/SET/ACTION) |
| [dlms-server](./dlms-server/) | DLMS server (GET/SET/ACTION handlers) |
| [dlms-transport](./dlms-transport/) | Transport (TCP, UDP, serial) |

## Features

- **No-std support**: All crates support `no_std` (feature gate: `default = ["std"]`)
- **SM4 encryption**: Full SM4 implementation for Chinese DLMS
- **Extensive testing**: 967 tests + property tests + fuzz targets

## Quick Start

### Parse an HDLC frame and extract DLMS data

```rust
use dlms_hdlc::{HdlcParser, build_frame};

// Build a GET-REQUEST frame
let info = vec![0xE6, 0xE0, 0x00, 0x01, 0x00, 0xFF, 0xFF];
let frame = build_frame(0x03, 0x10, &info);

// Parse incoming data
let mut parser = HdlcParser::new();
for byte in incoming_data {
    if let Some(result) = parser.feed(byte) {
        let hdlc_frame = result.unwrap();
        // info field contains DLMS APDU starting with 0xE6 (request) or 0xE7 (response)
    }
}
```

### Encode/Decode DLMS data types

```rust
use dlms_axdr::{encode, decode};
use dlms_core::DlmsData;

let value = 12345u32;
let encoded = encode(&DlmsData::DoubleLongUnsigned(value));
let decoded = decode(&encoded).unwrap();
```

## Building

### Requirements

- Rust 1.70+ or stable
- For `no_std` builds: `--target thumbv6m-none-eabi` (Cortex-M0+)

```bash
cargo build --workspace
cargo build --workspace --no-default-features
```

## Testing

### Run all tests

```bash
cargo test --workspace
```

### Run specific crate tests

```bash
cargo test -p dlms-hdlc
cargo test -p dlms-axdr
```

### Property-based testing

Property tests use `proptest` to verify invariants over random inputs:

```bash
cargo test -p dlms-hdlc --test property_tests
cargo test -p dlms-axdr --test property_tests
```

### Fuzzing

```bash
cd fuzz
cargo fuzz run hdlc_parser -- -max_total_time=300
```

## Statistics

- **Total tests**: 967
- **Coverage**: 
  - dlms-hdlc: 99 tests (44 unit + 45 integration + 10 property)
  - dlms-cosem: 580 tests
  - dlms-axdr: 54 tests
  - dlms-client: 48 tests
  - dlms-hdlc integration: 45 tests
  - dlms-asn1: 38 tests
  - dlms-security: 26 tests
  - dlms-core: 10 tests
  - dlms-axdr property: 10 tests
  - dlms-transport: 11 tests
  - dlms-server: 11 tests
