//! IC100 LP Setup

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct LpSetup {
    period: u16,
}

impl LpSetup {
    pub fn new() -> Self {
        Self { period: 60 }
    }
    pub fn period(&self) -> u16 {
        self.period
    }
}

impl CosemObject for LpSetup {
    fn class_id(&self) -> u16 {
        100
    }
    fn logical_name(&self) -> ObisCode {
        ObisCode::new(0, 0, 43, 1, 0, 255)
    }
    fn attribute_count(&self) -> u8 {
        6
    }
    fn method_count(&self) -> u8 {
        2
    }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            2 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.period))),
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
    fn test_lp_setup_class_id() {
        assert_eq!(LpSetup::new().class_id(), 100);
    }
    #[test]
    fn test_lp_setup_period() {
        assert_eq!(LpSetup::new().period(), 60);
    }
}
