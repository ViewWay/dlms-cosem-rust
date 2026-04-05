# dlms-core

Core types for the DLMS/COSEM protocol stack.

## Overview

Provides fundamental types shared across all DLMS/COSEM crates:

- **`ObisCode`** — 6-byte OBIS identification code
- **`CosemDateTime`** — DLMS date/time representation (12-byte)
- **`DlmsData`** — All DLMS data types (integer, unsigned, float, string, array, structure, etc.)
- **`CosemObject`** — Trait for COSEM interface class implementations
- **`CosemAttribute`** / **`CosemMethod`** — Attribute and method descriptors
- **`AccessResult`** — COSEM access result codes

## Usage

```rust
use dlms_core::{ObisCode, DlmsData, CosemDateTime};

// Create an OBIS code
let clock = ObisCode::CLOCK; // 0.0.1.0.0.255

// Work with DLMS data types
let value = DlmsData::DoubleLongUnsigned(12345);
let encoded = dlms_axdr::encode(&value);
```

## Dependencies

None (leaf crate).

## Used By

All other `dlms-*` crates depend on this crate.
