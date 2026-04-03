//! IC001 Data - Generic data container

use dlms_core::{CosemObject, ObisCode, DlmsData, CosemObjectError};

/// IC001 Data - Generic COSEM data object
pub struct Data {
    logical_name: ObisCode,
    value: DlmsData,
}

impl Data {
    pub fn new(logical_name: ObisCode, value: DlmsData) -> Self {
        Self { logical_name, value }
    }

    pub fn value(&self) -> &DlmsData {
        &self.value
    }

    pub fn set_value(&mut self, value: DlmsData) {
        self.value = value;
    }
}

impl CosemObject for Data {
    fn class_id(&self) -> u16 { 1 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 2 }
    fn method_count(&self) -> u8 { 0 }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                // logical_name
                let name = self.logical_name.to_bytes();
                let mut bytes = vec![0x09, 0x06];
                bytes.extend_from_slice(&name);
                Some(bytes)
            }
            2 => Some(dlms_axdr::encode(&self.value)),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        if attr == 2 {
            self.value = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
            Ok(())
        } else {
            Err(CosemObjectError::AttributeNotSupported(attr))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_new() {
        let d = Data::new(ObisCode::DATA, DlmsData::None);
        assert_eq!(d.class_id(), 1);
    }

    #[test]
    fn test_data_value() {
        let d = Data::new(ObisCode::DATA, DlmsData::Boolean(true));
        assert_eq!(d.value().as_bool(), Some(true));
    }

    #[test]
    fn test_data_set_value() {
        let mut d = Data::new(ObisCode::DATA, DlmsData::None);
        d.set_value(DlmsData::Unsigned(42));
        assert_eq!(d.value().as_u8(), Some(42));
    }

    #[test]
    fn test_data_attribute_to_bytes() {
        let d = Data::new(ObisCode::DATA, DlmsData::Unsigned(42));
        let bytes = d.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_data_attribute_from_bytes() {
        let mut d = Data::new(ObisCode::DATA, DlmsData::None);
        let encoded = dlms_axdr::encode(&DlmsData::Unsigned(99));
        d.attribute_from_bytes(2, &encoded).unwrap();
        assert_eq!(d.value().as_u8(), Some(99));
    }

    #[test]
    fn test_data_unsupported_attr() {
        let d = Data::new(ObisCode::DATA, DlmsData::None);
        assert!(d.attribute_to_bytes(99).is_none());
    }
}
