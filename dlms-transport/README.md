# dlms-transport

Transport layer abstraction for DLMS/COSEM.

## Overview

Provides a trait-based transport abstraction with implementations:
- **TcpTransport** — TCP/IP (default)
- **UdpTransport** — UDP (default)
- **TlsTransport** — TLS (behind `tls` feature)
- **SerialTransport** — Serial port (behind `serial` feature)

## Usage

```rust
use dlms_transport::{Transport, TcpTransport};

let mut transport = TcpTransport::new("127.0.0.1:4059");
transport.connect()?;
transport.send(&data)?;
let n = transport.recv(&mut buf)?;
transport.close()?;
```

## Dependencies

- `dlms-core`
- `rustls`, `webpki-roots` (optional, behind `tls`)
- `serialport` (optional, behind `serial`)
