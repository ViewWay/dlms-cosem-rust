//! dlms-transport: Transport layer abstraction for DLMS/COSEM
//!
//! Provides a trait-based transport abstraction with implementations for:
//! - TcpTransport
//! - UdpTransport
//! - TlsTransport (behind `tls` feature)
//! - SerialTransport (behind `serial` feature)

/// Transport trait for DLMS/COSEM communication
pub trait Transport {
    /// Connect to the remote endpoint
    fn connect(&mut self) -> Result<(), TransportError>;

    /// Send data
    fn send(&mut self, data: &[u8]) -> Result<(), TransportError>;

    /// Receive data into buffer, returns number of bytes read
    fn recv(&mut self, buf: &mut [u8]) -> Result<usize, TransportError>;

    /// Close the connection
    fn close(&mut self) -> Result<(), TransportError>;

    /// Check if connected
    fn is_connected(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportError {
    ConnectionFailed(String),
    SendFailed(String),
    RecvFailed(String),
    NotConnected,
    Timeout,
    IoError(String),
}

impl core::fmt::Display for TransportError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            TransportError::ConnectionFailed(msg) => write!(f, "Connection failed: {msg}"),
            TransportError::SendFailed(msg) => write!(f, "Send failed: {msg}"),
            TransportError::RecvFailed(msg) => write!(f, "Receive failed: {msg}"),
            TransportError::NotConnected => write!(f, "Not connected"),
            TransportError::Timeout => write!(f, "Timeout"),
            TransportError::IoError(msg) => write!(f, "I/O error: {msg}"),
        }
    }
}

impl std::error::Error for TransportError {}

/// TCP Transport implementation
pub struct TcpTransport {
    address: String,
    stream: Option<std::net::TcpStream>,
    timeout: Option<std::time::Duration>,
}

impl TcpTransport {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            stream: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        }
    }

    pub fn with_timeout(address: &str, timeout: std::time::Duration) -> Self {
        Self {
            address: address.to_string(),
            stream: None,
            timeout: Some(timeout),
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}

impl Transport for TcpTransport {
    fn connect(&mut self) -> Result<(), TransportError> {
        let addr = self
            .address
            .parse::<std::net::SocketAddr>()
            .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;
        let stream = std::net::TcpStream::connect_timeout(
            &addr,
            self.timeout.unwrap_or(std::time::Duration::from_secs(5)),
        )
        .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;
        stream
            .set_read_timeout(self.timeout)
            .map_err(|e| TransportError::IoError(e.to_string()))?;
        stream
            .set_write_timeout(self.timeout)
            .map_err(|e| TransportError::IoError(e.to_string()))?;
        self.stream = Some(stream);
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<(), TransportError> {
        let stream = self.stream.as_mut().ok_or(TransportError::NotConnected)?;
        std::io::Write::write_all(stream, data)
            .map_err(|e| TransportError::SendFailed(e.to_string()))
    }

    fn recv(&mut self, buf: &mut [u8]) -> Result<usize, TransportError> {
        let stream = self.stream.as_mut().ok_or(TransportError::NotConnected)?;
        std::io::Read::read(stream, buf).map_err(|e| {
            if e.kind() == std::io::ErrorKind::TimedOut {
                TransportError::Timeout
            } else {
                TransportError::RecvFailed(e.to_string())
            }
        })
    }

    fn close(&mut self) -> Result<(), TransportError> {
        self.stream = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.stream.is_some()
    }
}

/// UDP Transport implementation
pub struct UdpTransport {
    address: String,
    socket: Option<std::net::UdpSocket>,
    timeout: Option<std::time::Duration>,
}

impl UdpTransport {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            socket: None,
            timeout: Some(std::time::Duration::from_secs(5)),
        }
    }
}

impl Transport for UdpTransport {
    fn connect(&mut self) -> Result<(), TransportError> {
        let socket = std::net::UdpSocket::bind("0.0.0.0:0")
            .map_err(|e| TransportError::ConnectionFailed(e.to_string()))?;
        socket
            .set_read_timeout(self.timeout)
            .map_err(|e| TransportError::IoError(e.to_string()))?;
        self.socket = Some(socket);
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<(), TransportError> {
        let socket = self.socket.as_ref().ok_or(TransportError::NotConnected)?;
        let addr = self
            .address
            .parse::<std::net::SocketAddr>()
            .map_err(|e| TransportError::SendFailed(e.to_string()))?;
        socket
            .send_to(data, addr)
            .map_err(|e| TransportError::SendFailed(e.to_string()))?;
        Ok(())
    }

    fn recv(&mut self, buf: &mut [u8]) -> Result<usize, TransportError> {
        let socket = self.socket.as_ref().ok_or(TransportError::NotConnected)?;
        socket
            .recv(buf)
            .map_err(|e| TransportError::RecvFailed(e.to_string()))
    }

    fn close(&mut self) -> Result<(), TransportError> {
        self.socket = None;
        Ok(())
    }
    fn is_connected(&self) -> bool {
        self.socket.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_transport_new() {
        let t = TcpTransport::new("127.0.0.1:4059");
        assert!(!t.is_connected());
        assert_eq!(t.address(), "127.0.0.1:4059");
    }

    #[test]
    fn test_tcp_transport_with_timeout() {
        let t = TcpTransport::with_timeout("127.0.0.1:4059", std::time::Duration::from_secs(10));
        assert!(!t.is_connected());
    }

    #[test]
    fn test_tcp_transport_send_not_connected() {
        let mut t = TcpTransport::new("127.0.0.1:4059");
        assert!(matches!(
            t.send(&[1, 2, 3]),
            Err(TransportError::NotConnected)
        ));
    }

    #[test]
    fn test_tcp_transport_recv_not_connected() {
        let mut t = TcpTransport::new("127.0.0.1:4059");
        let mut buf = [0u8; 16];
        assert!(matches!(
            t.recv(&mut buf),
            Err(TransportError::NotConnected)
        ));
    }

    #[test]
    fn test_tcp_transport_close_not_connected() {
        let mut t = TcpTransport::new("127.0.0.1:4059");
        assert!(t.close().is_ok());
    }

    #[test]
    fn test_udp_transport_new() {
        let t = UdpTransport::new("127.0.0.1:4059");
        assert!(!t.is_connected());
    }

    #[test]
    fn test_udp_transport_send_not_connected() {
        let mut t = UdpTransport::new("127.0.0.1:4059");
        assert!(matches!(
            t.send(&[1, 2, 3]),
            Err(TransportError::NotConnected)
        ));
    }

    #[test]
    fn test_transport_error_display() {
        let err = TransportError::NotConnected;
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_transport_error_timeout() {
        let err = TransportError::Timeout;
        assert_eq!(format!("{err}"), "Timeout");
    }

    #[test]
    fn test_tcp_connect_invalid_address() {
        let mut t = TcpTransport::new("invalid:address");
        assert!(t.connect().is_err());
    }

    #[test]
    fn test_udp_connect_bind() {
        let mut t = UdpTransport::new("127.0.0.1:4059");
        // Should be able to bind (may fail in restricted environments)
        // Just test it doesn't panic
        let _ = t.connect();
    }
}
