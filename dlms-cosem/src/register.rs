//! IC003 Register

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct Register {
    logical_name: ObisCode,
    value: DlmsData,
    unit: u8,
    scaler: i8,
}

impl Register {
    pub fn new(logical_name: ObisCode, value: DlmsData) -> Self {
        Self {
            logical_name,
            value,
            unit: 0,
            scaler: 0,
        }
    }

    pub fn with_unit(logical_name: ObisCode, value: DlmsData, unit: u8, scaler: i8) -> Self {
        Self {
            logical_name,
            value,
            unit,
            scaler,
        }
    }

    pub fn value(&self) -> &DlmsData {
        &self.value
    }
    pub fn set_value(&mut self, value: DlmsData) {
        self.value = value;
    }
    pub fn unit(&self) -> u8 {
        self.unit
    }
    pub fn scaler(&self) -> i8 {
        self.scaler
    }
}

impl CosemObject for Register {
    fn class_id(&self) -> u16 {
        3
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
            3 => Some(dlms_axdr::encode(&DlmsData::Structure(vec![
                DlmsData::Enum(self.scaler as u8),
                DlmsData::Enum(self.unit),
            ]))),
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
    fn test_register_new() {
        let r = Register::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(1000));
        assert_eq!(r.class_id(), 3);
    }

    #[test]
    fn test_register_value() {
        let r = Register::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(42));
        assert_eq!(r.value().as_i32(), Some(42));
    }

    #[test]
    fn test_register_roundtrip() {
        let mut r = Register::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(12345));
        r.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(r.value().as_i32(), Some(12345));
    }

    #[test]
    fn test_register_unit_scaler() {
        let r = Register::with_unit(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0), 27, -1);
        assert_eq!(r.unit(), 27);
        assert_eq!(r.scaler(), -1);
    }

    #[test]
    fn test_register_attr1_logical_name() {
        let r = Register::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        let bytes = r.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_register_attr3_unit_scaler() {
        let r = Register::with_unit(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0), 27, -1);
        let bytes = r.attribute_to_bytes(3).unwrap();
        assert!(!bytes.is_empty());
    }
}
