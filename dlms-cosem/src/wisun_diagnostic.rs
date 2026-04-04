//! IC96 Wi-SUN Diagnostic
//! Blue Book Ed16: class_id=96, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Wi-SUN Diagnostic - network diagnostic counters
pub struct WiSunDiagnostic {
    logical_name: ObisCode,
    messages_sent: u32,
    messages_received: u32,
    messages_failed: u32,
}

impl WiSunDiagnostic {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            messages_sent: 0,
            messages_received: 0,
            messages_failed: 0,
        }
    }
    pub fn messages_sent(&self) -> u32 {
        self.messages_sent
    }
    pub fn messages_received(&self) -> u32 {
        self.messages_received
    }
    pub fn messages_failed(&self) -> u32 {
        self.messages_failed
    }
    pub fn increment_sent(&mut self) {
        self.messages_sent += 1;
    }
}

impl CosemObject for WiSunDiagnostic {
    fn class_id(&self) -> u16 {
        96
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        13
    }
    fn method_count(&self) -> u8 {
        0
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let n = self.logical_name.to_bytes();
                Some(vec![0x09, 0x06, n[0], n[1], n[2], n[3], n[4], n[5]])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.messages_sent,
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.messages_received,
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.messages_failed,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, _attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        Err(CosemObjectError::AttributeNotSupported(_attr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wisun_diag_new() {
        let d = WiSunDiagnostic::new(ObisCode::CLOCK);
        assert_eq!(d.class_id(), 96);
    }
    #[test]
    fn test_wisun_diag_increment() {
        let mut d = WiSunDiagnostic::new(ObisCode::CLOCK);
        d.increment_sent();
        assert_eq!(d.messages_sent(), 1);
    }
}
