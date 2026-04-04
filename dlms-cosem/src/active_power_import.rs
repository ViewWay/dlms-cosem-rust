//! IC029 Active Power Import
//! (Alias for IC031 Single Phase with import direction)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Active Power Import - measures imported active power
pub struct ActivePowerImport {
    logical_name: ObisCode,
    value: DlmsData,
    unit: u8,
    scaler: i8,
}

impl ActivePowerImport {
    pub fn new(logical_name: ObisCode, value: DlmsData) -> Self {
        Self {
            logical_name,
            value,
            unit: 27, // W
            scaler: 0,
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

impl CosemObject for ActivePowerImport {
    fn class_id(&self) -> u16 {
        29
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
    fn test_active_power_import_new() {
        let api = ActivePowerImport::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(1000));
        assert_eq!(api.class_id(), 29);
        assert_eq!(api.unit(), 27);
    }

    #[test]
    fn test_active_power_import_value() {
        let api = ActivePowerImport::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(42));
        assert_eq!(api.value().as_i32(), Some(42));
    }

    #[test]
    fn test_active_power_import_roundtrip() {
        let mut api = ActivePowerImport::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(12345));
        api.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(api.value().as_i32(), Some(12345));
    }
}
