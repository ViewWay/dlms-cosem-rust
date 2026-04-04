//! IC038 Single Phase Export

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Single Phase Export - measures exported energy on a single phase
pub struct SinglePhaseExport {
    logical_name: ObisCode,
    value: DlmsData,
    unit: u8,
    scaler: i8,
}

impl SinglePhaseExport {
    pub fn new(logical_name: ObisCode, value: DlmsData) -> Self {
        Self {
            logical_name,
            value,
            unit: 1, // Wh
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

impl CosemObject for SinglePhaseExport {
    fn class_id(&self) -> u16 {
        212
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
    fn test_single_phase_export_new() {
        let spe = SinglePhaseExport::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(1000));
        assert_eq!(spe.class_id(), 212);
    }

    #[test]
    fn test_single_phase_export_value() {
        let spe = SinglePhaseExport::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(42));
        assert_eq!(spe.value().as_i32(), Some(42));
    }

    #[test]
    fn test_single_phase_export_roundtrip() {
        let mut spe = SinglePhaseExport::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(55555));
        spe.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(spe.value().as_i32(), Some(55555));
    }
}
