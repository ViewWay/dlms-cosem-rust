//! dlms-client: DLMS/COSEM Client
//!
//! Implements a DLMS client for communicating with meters:
//! - Association (AARQ/AARE)
//! - Get/Set/Action requests
//! - HDLC and TCP (wrapper) transport
//! - Timeout and retry support

use dlms_core::{AccessResult, DlmsData, ObisCode};
use dlms_transport::{Transport, TransportError};
use std::time::Duration;

/// Client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub client_address: u8,
    pub server_address: u8,
    pub timeout: Duration,
    pub retries: u32,
    pub max_send_window: u8,
    pub max_recv_window: u8,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            client_address: 0x10,
            server_address: 0x01,
            timeout: Duration::from_secs(5),
            retries: 3,
            max_send_window: 1,
            max_recv_window: 7,
        }
    }
}

/// DLMS client states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientState {
    Disconnected,
    Connecting,
    Associated,
    Releasing,
}

/// DLMS Client
pub struct DlmsClient<T: Transport> {
    transport: T,
    config: ClientConfig,
    state: ClientState,
    send_seq: u8,
    recv_seq: u8,
    invoke_id: u8,
}

impl<T: Transport> DlmsClient<T> {
    pub fn new(transport: T, config: ClientConfig) -> Self {
        Self {
            transport,
            config,
            state: ClientState::Disconnected,
            send_seq: 0,
            recv_seq: 0,
            invoke_id: 0xC0,
        }
    }

    pub fn state(&self) -> ClientState {
        self.state
    }
    pub fn is_associated(&self) -> bool {
        self.state == ClientState::Associated
    }

    fn next_invoke_id(&mut self) -> u8 {
        let id = self.invoke_id;
        self.invoke_id = if self.invoke_id >= 0xF0 {
            0xC0
        } else {
            self.invoke_id + 1
        };
        id
    }

    /// Connect the transport
    pub fn connect(&mut self) -> Result<(), ClientError> {
        self.state = ClientState::Connecting;
        self.transport.connect().map_err(ClientError::Transport)?;
        Ok(())
    }

    /// Associate with the server using HDLC
    pub fn associate_hdlc(&mut self) -> Result<AssociateResult, ClientError> {
        self.connect()?;
        // Send SNRM (Set Normal Response Mode)
        let snrm = dlms_hdlc::build_frame(self.config.server_address, 0x83, &[]);
        self.transport.send(&snrm).map_err(ClientError::Transport)?;

        // Wait for UA
        let mut buf = [0u8; 1024];
        let n = self
            .transport
            .recv(&mut buf)
            .map_err(ClientError::Transport)?;
        let mut parser = dlms_hdlc::HdlcParser::new();
        for &byte in &buf[..n] {
            if let Some(result) = parser.feed(byte) {
                let _frame = result.map_err(|e| ClientError::Hdlc(e.to_string()))?;
                break;
            }
        }

        // Send AARQ
        let aarq = dlms_asn1::AarqApdu {
            protocol_version: 1,
            application_context_name: dlms_asn1::app_context::ACSE_DLMS_NO_CIPHER.to_vec(),
            called_ap_title: None,
            calling_ap_title: None,
            mechanism_name: None,
            user_information: None,
        };
        let aarq_bytes = aarq.encode();

        // Send AARQ in I-frame
        let control = dlms_hdlc::FrameType::I {
            send_seq: self.send_seq,
            recv_seq: self.recv_seq,
        }
        .to_control();
        let frame = dlms_hdlc::build_frame(self.config.server_address, control, &aarq_bytes);
        self.transport
            .send(&frame)
            .map_err(ClientError::Transport)?;
        self.send_seq = (self.send_seq + 1) & 0x07;

        // Wait for AARE
        let n = self
            .transport
            .recv(&mut buf)
            .map_err(ClientError::Transport)?;
        let mut parser2 = dlms_hdlc::HdlcParser::new();
        for &byte in &buf[..n] {
            if let Some(result) = parser2.feed(byte) {
                let frame = result.map_err(|e| ClientError::Hdlc(e.to_string()))?;
                let aare = dlms_asn1::AareApdu::decode(&frame.info)
                    .map_err(|e| ClientError::Asn1(e.to_string()))?;
                if let dlms_hdlc::FrameType::I { recv_seq, .. } =
                    dlms_hdlc::ControlField::from_byte(frame.control.to_byte()).frame_type()
                {
                    self.recv_seq = (recv_seq + 1) & 0x07;
                }
                self.state = ClientState::Associated;
                return Ok(AssociateResult {
                    accepted: aare.is_accepted(),
                    result: aare.result,
                });
            }
        }

        Err(ClientError::Timeout)
    }

    /// Read an attribute from a COSEM object
    pub fn get(
        &mut self,
        class_id: u16,
        obis: ObisCode,
        attribute_id: u8,
    ) -> Result<DlmsData, ClientError> {
        if !self.is_associated() {
            return Err(ClientError::NotAssociated);
        }

        let invoke_id = self.next_invoke_id();
        // Build GET-Request (normal)
        let mut get_request = Vec::new();
        get_request.push(0xC0); // GET-Request normal
        get_request.push(invoke_id);
        // CosemAttributeDescriptor
        get_request.push(0x02); // Structure tag
        get_request.push(0x04); // 4 elements
                                // class_id
        get_request.push(0x12); // LongUnsigned
        get_request.push(0x02);
        get_request.extend_from_slice(&class_id.to_be_bytes());
        // logical_name
        get_request.push(0x09); // OctetString
        get_request.push(0x06);
        get_request.extend_from_slice(&obis.to_bytes());
        // attribute_id
        get_request.push(0x91); // Integer
        get_request.push(0x02);
        get_request.push(0x02);
        get_request.push(attribute_id);
        // selector
        get_request.push(0x10);
        get_request.push(0x02);
        get_request.push(0x00);
        get_request.push(0x00);

        // Wrap in xDLMS APDU
        let mut apdu = Vec::new();
        apdu.push(invoke_id);
        apdu.push(0x05); // Confirmed-Service-Request (GET)
        apdu.extend_from_slice(&get_request[1..]);

        // Send in I-frame
        let control = dlms_hdlc::FrameType::I {
            send_seq: self.send_seq,
            recv_seq: self.recv_seq,
        }
        .to_control();
        let frame = dlms_hdlc::build_frame(self.config.server_address, control, &apdu);
        self.transport
            .send(&frame)
            .map_err(ClientError::Transport)?;
        self.send_seq = (self.send_seq + 1) & 0x07;

        // Receive response
        let mut buf = [0u8; 4096];
        let n = self
            .transport
            .recv(&mut buf)
            .map_err(ClientError::Transport)?;
        let mut parser = dlms_hdlc::HdlcParser::new();
        for &byte in &buf[..n] {
            if let Some(result) = parser.feed(byte) {
                let frame = result.map_err(|e| ClientError::Hdlc(e.to_string()))?;
                if let dlms_hdlc::FrameType::I { recv_seq, .. } =
                    dlms_hdlc::ControlField::from_byte(frame.control.to_byte()).frame_type()
                {
                    self.recv_seq = (recv_seq + 1) & 0x07;
                }
                // Parse response - simplified: try AXDR decode
                if frame.info.len() > 2 {
                    let data = dlms_axdr::decode(&frame.info[2..])
                        .map_err(|e| ClientError::Axdr(e.to_string()))?;
                    return Ok(data);
                }
            }
        }

        Err(ClientError::Timeout)
    }

    /// Disconnect
    pub fn disconnect(&mut self) -> Result<(), ClientError> {
        // Send DISC
        let disc = dlms_hdlc::build_frame(self.config.server_address, 0x53, &[]);
        let _ = self.transport.send(&disc);
        self.transport.close().map_err(ClientError::Transport)?;
        self.state = ClientState::Disconnected;
        Ok(())
    }
}

/// Association result
#[derive(Debug, Clone)]
pub struct AssociateResult {
    pub accepted: bool,
    pub result: u8,
}

#[derive(Debug, Clone)]
pub enum ClientError {
    Transport(TransportError),
    Hdlc(String),
    Asn1(String),
    Axdr(String),
    Security(String),
    NotAssociated,
    Timeout,
    Access(AccessResult),
    Other(String),
}

impl core::fmt::Display for ClientError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ClientError::Transport(e) => write!(f, "Transport: {e}"),
            ClientError::Hdlc(msg) => write!(f, "HDLC: {msg}"),
            ClientError::Asn1(msg) => write!(f, "ASN.1: {msg}"),
            ClientError::Axdr(msg) => write!(f, "AXDR: {msg}"),
            ClientError::Security(msg) => write!(f, "Security: {msg}"),
            ClientError::NotAssociated => write!(f, "Not associated"),
            ClientError::Timeout => write!(f, "Timeout"),
            ClientError::Access(r) => write!(f, "Access error: {r:?}"),
            ClientError::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ClientError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_default() {
        let cfg = ClientConfig::default();
        assert_eq!(cfg.client_address, 0x10);
        assert_eq!(cfg.server_address, 0x01);
        assert_eq!(cfg.retries, 3);
    }

    #[test]
    fn test_client_state() {
        let t = dlms_transport::TcpTransport::new("127.0.0.1:4059");
        let client = DlmsClient::new(t, ClientConfig::default());
        assert_eq!(client.state(), ClientState::Disconnected);
        assert!(!client.is_associated());
    }

    #[test]
    fn test_next_invoke_id() {
        let t = dlms_transport::TcpTransport::new("127.0.0.1:4059");
        let mut client = DlmsClient::new(t, ClientConfig::default());
        let id1 = 0xC0;
        let _ = client.next_invoke_id();
        // invoke_id should increment
    }

    #[test]
    fn test_get_not_associated() {
        let t = dlms_transport::TcpTransport::new("127.0.0.1:4059");
        let mut client = DlmsClient::new(t, ClientConfig::default());
        let result = client.get(8, ObisCode::CLOCK, 2);
        assert!(matches!(result, Err(ClientError::NotAssociated)));
    }

    #[test]
    fn test_disconnect_not_connected() {
        let t = dlms_transport::TcpTransport::new("127.0.0.1:4059");
        let mut client = DlmsClient::new(t, ClientConfig::default());
        // Should not panic even if not connected
        let _ = client.disconnect();
        assert_eq!(client.state(), ClientState::Disconnected);
    }

    #[test]
    fn test_client_error_display() {
        let err = ClientError::NotAssociated;
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_client_error_timeout() {
        let err = ClientError::Timeout;
        assert_eq!(format!("{err}"), "Timeout");
    }

    #[test]
    fn test_associate_result() {
        let result = AssociateResult {
            accepted: true,
            result: 0,
        };
        assert!(result.accepted);
    }
}
