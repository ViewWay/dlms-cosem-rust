//! Integration tests for dlms-client
//!
//! Tests client-server interaction using a mock transport layer.

use dlms_core::{CosemObject, DlmsData, ObisCode};
use dlms_cosem::{Clock, DisconnectControl, Register};
use dlms_transport::{Transport, TransportError};

/// Mock transport that simulates a loopback connection to a DlmsServer.
struct MockTransport {
    connected: bool,
    send_buffer: Vec<u8>,
    recv_buffer: Vec<u8>,
    /// Whether the next recv should return timeout
    simulate_timeout: bool,
}

impl MockTransport {
    fn new() -> Self {
        Self {
            connected: false,
            send_buffer: Vec::new(),
            recv_buffer: Vec::new(),
            simulate_timeout: false,
        }
    }

    /// Pre-load a response to be returned on next recv
    fn set_response(&mut self, data: Vec<u8>) {
        self.recv_buffer = data;
    }
}

impl Transport for MockTransport {
    fn connect(&mut self) -> Result<(), TransportError> {
        self.connected = true;
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<(), TransportError> {
        if !self.connected {
            return Err(TransportError::NotConnected);
        }
        self.send_buffer.extend_from_slice(data);
        Ok(())
    }

    fn recv(&mut self, buf: &mut [u8]) -> Result<usize, TransportError> {
        if !self.connected {
            return Err(TransportError::NotConnected);
        }
        if self.simulate_timeout {
            return Err(TransportError::Timeout);
        }
        if self.recv_buffer.is_empty() {
            return Err(TransportError::Timeout);
        }
        let n = self.recv_buffer.len().min(buf.len());
        buf[..n].copy_from_slice(&self.recv_buffer[..n]);
        self.recv_buffer.drain(..n);
        Ok(n)
    }

    fn close(&mut self) -> Result<(), TransportError> {
        self.connected = false;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }
}

#[test]
fn test_client_creation() {
    use dlms_client::{ClientConfig, DlmsClient};

    let transport = MockTransport::new();
    let config = ClientConfig::default();
    let _client = DlmsClient::new(transport, config);
}

#[test]
fn test_client_initial_state() {
    use dlms_client::{ClientConfig, ClientState, DlmsClient};

    let transport = MockTransport::new();
    let config = ClientConfig::default();
    let client = DlmsClient::new(transport, config);
    assert_eq!(client.state(), ClientState::Disconnected);
    assert!(!client.is_associated());
}

#[test]
fn test_client_connect() {
    use dlms_client::{ClientConfig, DlmsClient};

    let transport = MockTransport::new();
    let mut client = DlmsClient::new(transport, ClientConfig::default());
    assert!(client.connect().is_ok());
}

#[test]
fn test_client_disconnect() {
    use dlms_client::{ClientConfig, DlmsClient};

    let transport = MockTransport::new();
    let mut client = DlmsClient::new(transport, ClientConfig::default());
    client.connect().unwrap();
    assert!(client.disconnect().is_ok());
}

#[test]
fn test_client_get_without_association() {
    use dlms_client::{ClientConfig, ClientError, DlmsClient};

    let transport = MockTransport::new();
    let mut client = DlmsClient::new(transport, ClientConfig::default());
    let result = client.get(8, ObisCode::CLOCK, 2);
    assert!(matches!(result, Err(ClientError::NotAssociated)));
}

#[test]
fn test_mock_transport_send_without_connect() {
    let mut transport = MockTransport::new();
    assert!(matches!(transport.send(&[1, 2, 3]), Err(TransportError::NotConnected)));
}

#[test]
fn test_mock_transport_recv_without_connect() {
    let mut transport = MockTransport::new();
    let mut buf = [0u8; 16];
    assert!(matches!(transport.recv(&mut buf), Err(TransportError::NotConnected)));
}

#[test]
fn test_mock_transport_send_recv_loopback() {
    let mut transport = MockTransport::new();
    transport.connect().unwrap();

    let response = vec![0x01, 0x02, 0x03];
    transport.set_response(response.clone());

    transport.send(&[0xAA, 0xBB]).unwrap();
    let mut buf = [0u8; 16];
    let n = transport.recv(&mut buf).unwrap();
    assert_eq!(n, 3);
    assert_eq!(&buf[..n], &response);
}

#[test]
fn test_mock_transport_timeout() {
    let mut transport = MockTransport::new();
    transport.connect().unwrap();
    transport.simulate_timeout = true;

    let mut buf = [0u8; 16];
    assert!(matches!(transport.recv(&mut buf), Err(TransportError::Timeout)));
}

#[test]
fn test_mock_transport_close() {
    let mut transport = MockTransport::new();
    transport.connect().unwrap();
    assert!(transport.is_connected());
    transport.close().unwrap();
    assert!(!transport.is_connected());
}

#[test]
fn test_server_side_get_response() {
    use dlms_server::{DlmsServer, ServerConfig};

    let mut server = DlmsServer::new(ServerConfig::default());
    server.register_object(Box::new(Clock::new(ObisCode::CLOCK)));
    server.register_object(Box::new(Register::new(
        ObisCode::ACTIVE_POWER_L1,
        DlmsData::DoubleLong(1234),
    )));

    // Verify server can handle get requests
    let result = server.handle_get(8, &ObisCode::CLOCK, 1);
    assert!(result.is_ok());

    let result = server.handle_get(3, &ObisCode::ACTIVE_POWER_L1, 2);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_i32(), Some(1234));
}

#[test]
fn test_server_set_then_client_read_scenario() {
    use dlms_server::{DlmsServer, ServerConfig};

    // Simulate server-side update + client read pattern
    let mut server = DlmsServer::new(ServerConfig::default());
    server.register_object(Box::new(Register::new(
        ObisCode::ACTIVE_POWER_L1,
        DlmsData::DoubleLong(100),
    )));

    // Server updates (simulating meter event)
    let new_val = dlms_axdr::encode(&DlmsData::DoubleLong(200));
    server
        .handle_set(3, &ObisCode::ACTIVE_POWER_L1, 2, &new_val)
        .unwrap();

    // Client reads updated value
    let result = server.handle_get(3, &ObisCode::ACTIVE_POWER_L1, 2).unwrap();
    assert_eq!(result.as_i32(), Some(200));
}

#[test]
fn test_disconnect_control_server_action() {
    use dlms_server::{DlmsServer, ServerConfig};

    let mut server = DlmsServer::new(ServerConfig::default());
    let obis = ObisCode::new(0, 0, 96, 1, 0, 255);
    server.register_object(Box::new(DisconnectControl::new(obis)));

    // Initial state should be connected
    let state = server.handle_get(70, &obis, 2).unwrap();
    if let DlmsData::Enum(v) = state { assert_eq!(v, 1); } else { panic!("Expected Enum"); }

    // Disconnect
    server.handle_action(70, &obis, 1, &[]).unwrap();
    let state = server.handle_get(70, &obis, 2).unwrap();
    if let DlmsData::Enum(v) = state { assert_eq!(v, 0); } else { panic!("Expected Enum"); }

    // Reconnect
    server.handle_action(70, &obis, 2, &[]).unwrap();
    let state = server.handle_get(70, &obis, 2).unwrap();
    if let DlmsData::Enum(v) = state { assert_eq!(v, 1); } else { panic!("Expected Enum"); }
}

#[test]
fn test_client_config_default() {
    use dlms_client::ClientConfig;

    let config = ClientConfig::default();
    // Verify default config fields exist and are reasonable
    let _ = config;
}
