//! IC020 Generic Accumulator / Total
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: value (any)
//! 3: scaler_unit (structure)
//! 4: status (double-long-unsigned)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct Total {
    logical_name: ObisCode,
    value: DlmsData,
    unit: u8,
    scaler: i8,
    status: u32,
}

impl Total {
    pub fn new(logical_name: ObisCode, value: DlmsData) -> Self {
        Self {
            logical_name,
            value,
            unit: 0,
            scaler: 0,
            status: 0,
        }
    }

    pub fn value(&self) -> &DlmsData {
        &self.value
    }
    pub fn set_value(&mut self, value: DlmsData) {
        self.value = value;
    }
}

impl CosemObject for Total {
    fn class_id(&self) -> u16 {
        20
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
            2 => Some(dlms_axdr::encode(&self.value)),
            3 => Some(dlms_axdr::encode(&DlmsData::Structure(vec![
                DlmsData::Enum(self.scaler as u8),
                DlmsData::Enum(self.unit),
            ]))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.status,
            ))),
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
    fn test_total_class_id() {
        let t = Total::new(
            ObisCode::ACTIVE_ENERGY_IMPORT,
            DlmsData::DoubleLongUnsigned(0),
        );
        assert_eq!(t.class_id(), 20);
    }

    #[test]
    fn test_total_roundtrip() {
        let mut t = Total::new(
            ObisCode::ACTIVE_ENERGY_IMPORT,
            DlmsData::DoubleLongUnsigned(0),
        );
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(12345));
        t.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(t.value(), &DlmsData::DoubleLongUnsigned(12345));
    }
}
