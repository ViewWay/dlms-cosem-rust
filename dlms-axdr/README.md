# dlms-axdr

AXDR (ASN.1 XER Defined Rules) encoding/decoding for DLMS/COSEM.

## Overview

Implements AXDR encoding for all DLMS data types as specified in IEC 62056-53:
- Integer types (i8, i16, i32, i64)
- Unsigned types (u8, u16, u32, u64)
- Float/Double
- OctetString, VisibleString, Utf8String
- Array, Structure
- Boolean, Enum, None
- DateTime, Date, Time
- BitString, Bcd, CompactArray

## Usage

```rust
use dlms_axdr::{encode, decode};
use dlms_core::DlmsData;

let data = DlmsData::DoubleLongUnsigned(42);
let encoded = encode(&data);
let decoded = decode(&encoded).unwrap();
assert_eq!(decoded, data);
```

## Dependencies

- `dlms-core`

## Dev Dependencies

- `proptest` (property-based tests)
