//! IC102 Infrared Setup

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct InfraredSetup {
    baudrate: u32,
}

impl InfraredSetup {
    pub fn new() -> Self {
        Self { baudrate: 9600 }
    }
}

impl CosemObject for InfraredSetup {
    fn class_id(&self) -> u16 {
        102
    }
    fn logical_name(&self) -> ObisCode {
        ObisCode::new(0, 0, 45, 0, 0, 255)
    }
    fn attribute_count(&self) -> u8 {
        5
    }
    fn method_count(&self) -> u8 {
        0
    }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.baudrate,
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
    fn test_infrared_class_id() {
        assert_eq!(InfraredSetup::new().class_id(), 102);
    }
}
