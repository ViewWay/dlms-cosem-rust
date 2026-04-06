//! IC005 Demand Register

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct DemandRegister {
    logical_name: ObisCode,
    current_value: DlmsData,
    unit: u8,
    scaler: i8,
    period: u16,
    #[allow(dead_code)]
    number_of_periods: u8,
}

impl DemandRegister {
    pub fn new(logical_name: ObisCode, value: DlmsData) -> Self {
        Self {
            logical_name,
            current_value: value,
            unit: 0,
            scaler: 0,
            period: 60,
            number_of_periods: 1,
        }
    }

    pub fn value(&self) -> &DlmsData {
        &self.current_value
    }
    pub fn set_value(&mut self, value: DlmsData) {
        self.current_value = value;
    }
}

impl CosemObject for DemandRegister {
    fn class_id(&self) -> u16 {
        5
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        7
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
            2 => Some(dlms_axdr::encode(&self.current_value)),
            3 => Some(dlms_axdr::encode(&DlmsData::Structure(vec![
                DlmsData::Enum(self.scaler as u8),
                DlmsData::Enum(self.unit),
            ]))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.period as u32,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                self.current_value =
                    dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
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
    fn test_demand_register_class_id() {
        let r = DemandRegister::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        assert_eq!(r.class_id(), 5);
    }

    #[test]
    fn test_demand_register_roundtrip() {
        let mut r = DemandRegister::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(555));
        r.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(r.value().as_i32(), Some(555));
    }

    #[test]
    fn test_demand_register_attribute_count() {
        let r = DemandRegister::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        assert_eq!(r.attribute_count(), 7);
    }

    #[test]
    fn test_demand_register_attr1_logical_name() {
        let r = DemandRegister::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        let bytes = r.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_demand_register_attr2_value() {
        let r = DemandRegister::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(12345));
        let bytes = r.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_demand_register_attr3_unit_scaler() {
        let r = DemandRegister::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        let bytes = r.attribute_to_bytes(3).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_demand_register_set_value() {
        let mut r = DemandRegister::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        r.set_value(DlmsData::DoubleLong(9999));
        assert_eq!(r.value().as_i32(), Some(9999));
    }
}
