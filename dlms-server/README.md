# dlms-server

DLMS/COSEM Server (meter simulator).

## Overview

Implements a DLMS server for testing and simulation:
- COSEM object registration
- Request routing (GET/SET/ACTION)
- Raw frame processing
- Full xDLMS APDU handling

## Usage

```rust
use dlms_server::{DlmsServer, ServerConfig};
use dlms_cosem::{Clock, Register};
use dlms_core::{ObisCode, DlmsData, CosemObject};

let mut server = DlmsServer::new(ServerConfig::default());
server.register_object(Box::new(Clock::new(ObisCode::CLOCK)));
server.register_object(Box::new(Register::new(
    ObisCode::ACTIVE_POWER_L1,
    DlmsData::DoubleLong(1234),
)));

// Handle requests
let result = server.handle_get(8, &ObisCode::CLOCK, 2)?;
```

## Dependencies

- `dlms-core`, `dlms-axdr`, `dlms-cosem`
