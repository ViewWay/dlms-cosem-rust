//! IC101 RS485 Setup

use dlms_core::{CosemObject, ObisCode, DlmsData, CosemObjectError};

pub struct Rs485Setup { baudrate: u32 }

impl Rs485Setup {
    pub fn new() -> Self { Self { baudrate: 9600 } }
    pub fn baudrate(&self) -> u32 { self.baudrate }
}

impl CosemObject for Rs485Setup {
    fn class_id(&self) -> u16 { 101 }
    fn logical_name(&self) -> ObisCode { ObisCode::new(0, 0, 44, 0, 0, 255) }
    fn attribute_count(&self) -> u8 { 9 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.baudrate))),
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
    fn test_rs485_class_id() { assert_eq!(Rs485Setup::new().class_id(), 101); }
    #[test]
    fn test_rs485_baudrate() { assert_eq!(Rs485Setup::new().baudrate(), 9600); }
}
