//! IC28 ClockControl

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct ClockControl {
    logical_name: ObisCode,
    version: u8,
}

impl ClockControl {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            version: 0,
        }
    }

    pub fn version(&self) -> u8 {
        self.version
    }
}

impl CosemObject for ClockControl {
    fn class_id(&self) -> u16 {
        28
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        6
    }
    fn method_count(&self) -> u8 {
        1
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
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
    fn test_clock_control_class_id() {
        let obj = ClockControl::new(ObisCode::new(0, 0, 0, 0, 0, 255));
        assert_eq!(obj.class_id(), 28);
    }
}
