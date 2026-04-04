//! IC010 Demand Register (IC10)
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: current_value (any)
//! 3: scaler_unit (structure: scaler enum, unit enum)
//! 4: status (double-long-unsigned)
//! 5: capture_time (octet-string / date-time)
//! 6: period (double-long-unsigned)
//! 7: number_of_periods (unsigned)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct Demand {
    logical_name: ObisCode,
    current_value: DlmsData,
    unit: u8,
    scaler: i8,
    status: u32,
    capture_time: DlmsData,
    period: u32,
    number_of_periods: u8,
}

impl Demand {
    pub fn new(logical_name: ObisCode, value: DlmsData) -> Self {
        Self {
            logical_name,
            current_value: value,
            unit: 0,
            scaler: 0,
            status: 0,
            capture_time: DlmsData::DateTime([0u8; 12]),
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
    pub fn period(&self) -> u32 {
        self.period
    }
}

impl CosemObject for Demand {
    fn class_id(&self) -> u16 {
        10
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
                self.status,
            ))),
            5 => Some(dlms_axdr::encode(&self.capture_time)),
            6 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.period,
            ))),
            7 => Some(dlms_axdr::encode(&DlmsData::Unsigned(
                self.number_of_periods,
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
    fn test_demand_class_id() {
        let d = Demand::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(100));
        assert_eq!(d.class_id(), 10);
    }

    #[test]
    fn test_demand_attr_count() {
        let d = Demand::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        assert_eq!(d.attribute_count(), 7);
    }

    #[test]
    fn test_demand_roundtrip() {
        let mut d = Demand::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(777));
        d.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(d.value().as_i32(), Some(777));
    }

    #[test]
    fn test_demand_period() {
        let d = Demand::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        assert_eq!(d.period(), 60);
    }

    #[test]
    fn test_demand_attr6_encode() {
        let d = Demand::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        let bytes = d.attribute_to_bytes(6).unwrap();
        assert!(!bytes.is_empty());
    }
}
