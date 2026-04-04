//! IC034 Maximum Demand (Register)
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: value (any)
//! 3: scaler_unit (structure)
//! 4: status (double-long-unsigned)
//! 5: start_time_current (date-time)
//! 6: capture_time (date-time)
//! 7: period (double-long-unsigned)
//! 8: number_of_periods (unsigned)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct MaximumDemand {
    logical_name: ObisCode,
    value: DlmsData,
    unit: u8,
    scaler: i8,
    status: u32,
    start_time_current: DlmsData,
    capture_time: DlmsData,
    period: u32,
    number_of_periods: u8,
}

impl MaximumDemand {
    pub fn new(logical_name: ObisCode, value: DlmsData) -> Self {
        Self {
            logical_name,
            value,
            unit: 0,
            scaler: 0,
            status: 0,
            start_time_current: DlmsData::DateTime([0u8; 12]),
            capture_time: DlmsData::DateTime([0u8; 12]),
            period: 60,
            number_of_periods: 1,
        }
    }

    pub fn value(&self) -> &DlmsData {
        &self.value
    }
    pub fn set_value(&mut self, value: DlmsData) {
        self.value = value;
    }
}

impl CosemObject for MaximumDemand {
    fn class_id(&self) -> u16 {
        34
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        8
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
            5 => Some(dlms_axdr::encode(&self.start_time_current)),
            6 => Some(dlms_axdr::encode(&self.capture_time)),
            7 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.period,
            ))),
            8 => Some(dlms_axdr::encode(&DlmsData::Unsigned(
                self.number_of_periods,
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
    fn test_max_demand_class_id() {
        let m = MaximumDemand::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        assert_eq!(m.class_id(), 34);
    }

    #[test]
    fn test_max_demand_roundtrip() {
        let mut m = MaximumDemand::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(9999));
        m.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(m.value().as_i32(), Some(9999));
    }

    #[test]
    fn test_max_demand_attr_count() {
        let m = MaximumDemand::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        assert_eq!(m.attribute_count(), 8);
    }
}
