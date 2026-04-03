//! IC004 Extended Register

use dlms_core::{CosemObject, ObisCode, DlmsData, CosemObjectError};

pub struct ExtendedRegister {
    logical_name: ObisCode,
    value: DlmsData,
    unit: u8,
    scaler: i8,
    status: DlmsData,
    capture_time: DlmsData,
}

impl ExtendedRegister {
    pub fn new(logical_name: ObisCode, value: DlmsData) -> Self {
        Self {
            logical_name, value, unit: 0, scaler: 0,
            status: DlmsData::DoubleLongUnsigned(0),
            capture_time: DlmsData::DateTime([0u8; 12]),
        }
    }

    pub fn value(&self) -> &DlmsData { &self.value }
    pub fn set_value(&mut self, value: DlmsData) { self.value = value; }
}

impl CosemObject for ExtendedRegister {
    fn class_id(&self) -> u16 { 4 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 5 }
    fn method_count(&self) -> u8 { 0 }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5]])
            }
            2 => Some(dlms_axdr::encode(&self.value)),
            3 => Some(dlms_axdr::encode(&DlmsData::Structure(vec![
                DlmsData::Enum(self.scaler as u8),
                DlmsData::Enum(self.unit),
            ]))),
            4 => Some(dlms_axdr::encode(&self.status)),
            5 => Some(dlms_axdr::encode(&self.capture_time)),
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
    fn test_extended_register_class_id() {
        let r = ExtendedRegister::new(ObisCode::ACTIVE_ENERGY_IMPORT, DlmsData::DoubleLong(0));
        assert_eq!(r.class_id(), 4);
    }

    #[test]
    fn test_extended_register_attr_count() {
        let r = ExtendedRegister::new(ObisCode::ACTIVE_ENERGY_IMPORT, DlmsData::DoubleLong(0));
        assert_eq!(r.attribute_count(), 5);
    }

    #[test]
    fn test_extended_register_roundtrip() {
        let mut r = ExtendedRegister::new(ObisCode::ACTIVE_ENERGY_IMPORT, DlmsData::DoubleLong(0));
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(999));
        r.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(r.value().as_i32(), Some(999));
    }
}
