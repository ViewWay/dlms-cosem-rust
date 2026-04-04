//! IC070 Disconnect Control
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: disconnect_control_state (enum)
//! 3: control_state (enum)
//! 4: output_state (enum)
//!
//! Methods:
//! 1: disconnect
//! 2: reconnect
//! 3: arm

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisconnectState {
    Disconnected = 0,
    Connected = 1,
    ReadyForDisconnect = 2,
    ReadyForReconnect = 3,
    Armed = 4,
}

pub struct DisconnectControl {
    logical_name: ObisCode,
    control_state: DisconnectState,
    output_state: DisconnectState,
}

impl DisconnectControl {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            control_state: DisconnectState::Connected,
            output_state: DisconnectState::Connected,
        }
    }

    pub fn control_state(&self) -> DisconnectState {
        self.control_state
    }
    pub fn set_control_state(&mut self, state: DisconnectState) {
        self.control_state = state;
    }
    pub fn output_state(&self) -> DisconnectState {
        self.output_state
    }
}

impl CosemObject for DisconnectControl {
    fn class_id(&self) -> u16 {
        70
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        4
    }
    fn method_count(&self) -> u8 {
        3
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::Enum(self.control_state as u8))),
            3 => Some(dlms_axdr::encode(&DlmsData::Enum(self.control_state as u8))),
            4 => Some(dlms_axdr::encode(&DlmsData::Enum(self.output_state as u8))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 | 3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(v) = decoded {
                    self.control_state = match v {
                        0 => DisconnectState::Disconnected,
                        1 => DisconnectState::Connected,
                        2 => DisconnectState::ReadyForDisconnect,
                        3 => DisconnectState::ReadyForReconnect,
                        4 => DisconnectState::Armed,
                        _ => return Err(CosemObjectError::InvalidData),
                    };
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }

    fn execute_action(&mut self, method_id: u8, _data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        match method_id {
            1 => {
                // Disconnect
                self.control_state = DisconnectState::Disconnected;
                self.output_state = DisconnectState::Disconnected;
                Ok(vec![0x00, 0x00])
            }
            2 => {
                // Reconnect
                self.control_state = DisconnectState::Connected;
                self.output_state = DisconnectState::Connected;
                Ok(vec![0x00, 0x00])
            }
            3 => {
                // Arm
                self.control_state = DisconnectState::Armed;
                Ok(vec![0x00, 0x00])
            }
            _ => Err(CosemObjectError::MethodNotSupported(method_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disconnect_control_class_id() {
        let dc = DisconnectControl::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        assert_eq!(dc.class_id(), 70);
    }

    #[test]
    fn test_disconnect_control_method_count() {
        let dc = DisconnectControl::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        assert_eq!(dc.method_count(), 3);
    }

    #[test]
    fn test_disconnect_control_state() {
        let dc = DisconnectControl::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        assert_eq!(dc.control_state(), DisconnectState::Connected);
    }

    #[test]
    fn test_disconnect_control_roundtrip() {
        let mut dc = DisconnectControl::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        let bytes = dlms_axdr::encode(&DlmsData::Enum(0)); // Disconnected
        dc.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(dc.control_state(), DisconnectState::Disconnected);
    }

    #[test]
    fn test_disconnect_control_action_disconnect() {
        let mut dc = DisconnectControl::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        let result = dc.execute_action(1, &[]).unwrap();
        assert_eq!(dc.control_state(), DisconnectState::Disconnected);
        assert_eq!(dc.output_state(), DisconnectState::Disconnected);
    }

    #[test]
    fn test_disconnect_control_action_reconnect() {
        let mut dc = DisconnectControl::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        dc.execute_action(1, &[]).unwrap(); // disconnect first
        dc.execute_action(2, &[]).unwrap(); // reconnect
        assert_eq!(dc.control_state(), DisconnectState::Connected);
    }
}
