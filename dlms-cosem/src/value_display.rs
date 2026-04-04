//! IC030 Value Display
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: value_to_display (any)
//! 3: status (enum)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct ValueDisplay {
    logical_name: ObisCode,
    value: DlmsData,
    status: u8,
}

impl ValueDisplay {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            value: DlmsData::None,
            status: 0,
        }
    }

    pub fn value(&self) -> &DlmsData {
        &self.value
    }
    pub fn set_value(&mut self, v: DlmsData) {
        self.value = v;
    }
}

impl CosemObject for ValueDisplay {
    fn class_id(&self) -> u16 {
        30
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        3
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
            2 => Some(dlms_axdr::encode(&self.value)),
            3 => Some(dlms_axdr::encode(&DlmsData::Enum(self.status))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                self.value = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                Ok(())
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_display_class_id() {
        let vd = ValueDisplay::new(ObisCode::new(0, 0, 30, 0, 0, 255));
        assert_eq!(vd.class_id(), 30);
    }

    #[test]
    fn test_value_display_roundtrip() {
        let mut vd = ValueDisplay::new(ObisCode::new(0, 0, 30, 0, 0, 255));
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(42));
        vd.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(vd.value().as_i32(), Some(42));
    }
}
