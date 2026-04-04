//! IC106 NB-IoT Setup

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct NbiotSetup {
    apn: String,
    #[allow(dead_code)]
    plmn: String,
}

impl NbiotSetup {
    pub fn new() -> Self {
        Self {
            apn: String::new(),
            plmn: String::new(),
        }
    }
}

impl CosemObject for NbiotSetup {
    fn class_id(&self) -> u16 {
        106
    }
    fn logical_name(&self) -> ObisCode {
        ObisCode::new(0, 0, 109, 0, 0, 255)
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
                self.apn.as_bytes().to_vec(),
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
    fn test_nbiot_class_id() {
        assert_eq!(NbiotSetup::new().class_id(), 106);
    }
}
