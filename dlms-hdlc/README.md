# dlms-hdlc

HDLC framing for DLMS/COSEM (IEC 62056-53).

## Overview

Implements the HDLC frame format used in DLMS/COSEM:
- Frame building/parsing with CRC-16
- Byte stuffing (escape sequences)
- I-frames, S-frames, U-frames
- Sliding window mechanism
- Stream-based frame parser

## Usage

```rust
use dlms_hdlc::{build_frame, HdlcParser};

// Build an HDLC frame
let frame = build_frame(server_addr, control_byte, &data);

// Parse incoming stream
let mut parser = HdlcParser::new();
for byte in stream {
    if let Some(result) = parser.feed(byte) {
        // Process complete frame
    }
}
```

## Dependencies

- `dlms-core`
