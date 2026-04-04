//! IC089 Transport

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Transport Mode
#[derive(Debug, Clone, PartialEq)]
pub enum TransportMode {
    Idle,
    Sending,
    Receiving,
}

/// Transport - data transport layer configuration
pub struct Transport {
    logical_name: ObisCode,
    mode: TransportMode,
    baud_rate: u32,
}

impl Transport {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            mode: TransportMode::Idle,
            baud_rate: 9600,
        }
    }

    pub fn mode(&self) -> &TransportMode {
        &self.mode
    }

    pub fn set_mode(&mut self, mode: TransportMode) {
        self.mode = mode;
    }

    pub fn baud_rate(&self) -> u32 {
        self.baud_rate
    }

    pub fn set_baud_rate(&mut self, rate: u32) {
        self.baud_rate = rate;
    }
}

impl CosemObject for Transport {
    fn class_id(&self) -> u16 {
        89
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        4
    }
    fn method_count(&self) -> u8 {
        0
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::Enum(match self.mode {
                TransportMode::Idle => 0,
                TransportMode::Sending => 1,
                TransportMode::Receiving => 2,
            }))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.baud_rate,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_u8() {
                    self.mode = match v {
                        0 => TransportMode::Idle,
                        1 => TransportMode::Sending,
                        _ => TransportMode::Receiving,
                    };
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_i32() {
                    self.baud_rate = v as u32;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transport_new() {
        let t = Transport::new(ObisCode::CLOCK);
        assert_eq!(t.class_id(), 89);
        assert_eq!(t.baud_rate(), 9600);
    }

    #[test]
    fn test_transport_mode() {
        let mut t = Transport::new(ObisCode::CLOCK);
        t.set_mode(TransportMode::Sending);
        assert_eq!(*t.mode(), TransportMode::Sending);
    }

    #[test]
    fn test_transport_roundtrip() {
        let mut t = Transport::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Unsigned(1));
        t.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(*t.mode(), TransportMode::Sending);
    }
}
