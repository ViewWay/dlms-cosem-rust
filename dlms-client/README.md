# dlms-client

DLMS/COSEM Client implementation.

## Overview

Implements a DLMS client for communicating with meters:
- HDLC connection management (SNRM/UA)
- Association (AARQ/AARE)
- GET/SET/ACTION requests
- Configurable timeouts, retries, and window sizes

## Usage

```rust
use dlms_client::{DlmsClient, ClientConfig};
use dlms_transport::TcpTransport;

let transport = TcpTransport::new("127.0.0.1:4059");
let config = ClientConfig::default();
let mut client = DlmsClient::new(transport, config);

client.connect()?;
client.associate_hdlc()?;
let value = client.get(8, ObisCode::CLOCK, 2)?; // Read clock time
client.disconnect()?;
```

## Dependencies

- `dlms-core`, `dlms-hdlc`, `dlms-axdr`, `dlms-asn1`, `dlms-security`, `dlms-cosem`, `dlms-transport`
