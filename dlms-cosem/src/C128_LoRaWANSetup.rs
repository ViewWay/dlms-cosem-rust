//! IC107 LoRaWAN Setup

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct LorawanSetup {
    dev_eui: [u8; 8],
    app_eui: [u8; 8],
    #[allow(dead_code)]
    app_key: [u8; 16],
}

impl LorawanSetup {
    pub fn new() -> Self {
        Self {
            dev_eui: [0; 8],
            app_eui: [0; 8],
            app_key: [0; 16],
        }
    }
}

impl CosemObject for LorawanSetup {
    fn class_id(&self) -> u16 {
        128
    }
    fn logical_name(&self) -> ObisCode {
        ObisCode::new(0, 0, 110, 0, 0, 255)
    }
    fn attribute_count(&self) -> u8 {
        10
    }
    fn method_count(&self) -> u8 {
        0
    }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.dev_eui.to_vec(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.app_eui.to_vec(),
            ))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        Err(CosemObjectError::AttributeNotSupported(attr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lorawan_class_id() {
        assert_eq!(LorawanSetup::new().class_id(), 128);
    }
}
