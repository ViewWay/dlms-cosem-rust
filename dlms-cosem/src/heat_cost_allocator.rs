//! IC083 Heat Cost Allocator

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Heat Cost Allocator - heat cost allocation measurement
pub struct HeatCostAllocator {
    logical_name: ObisCode,
    value: DlmsData,
    unit: u8,
    scaler: i8,
    integration_period: u32,
}

impl HeatCostAllocator {
    pub fn new(logical_name: ObisCode, value: DlmsData) -> Self {
        Self {
            logical_name,
            value,
            unit: 0,
            scaler: 0,
            integration_period: 60,
        }
    }

    pub fn value(&self) -> &DlmsData {
        &self.value
    }

    pub fn set_value(&mut self, value: DlmsData) {
        self.value = value;
    }

    pub fn integration_period(&self) -> u32 {
        self.integration_period
    }

    pub fn set_integration_period(&mut self, period: u32) {
        self.integration_period = period;
    }
}

impl CosemObject for HeatCostAllocator {
    fn class_id(&self) -> u16 {
        208
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
                self.integration_period,
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
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_i32() {
                    self.integration_period = v as u32;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heat_cost_allocator_new() {
        let hca = HeatCostAllocator::new(ObisCode::CLOCK, DlmsData::DoubleLong(100));
        assert_eq!(hca.class_id(), 208);
    }

    #[test]
    fn test_heat_cost_allocator_value() {
        let hca = HeatCostAllocator::new(ObisCode::CLOCK, DlmsData::DoubleLong(42));
        assert_eq!(hca.value().as_i32(), Some(42));
    }

    #[test]
    fn test_heat_cost_allocator_roundtrip() {
        let mut hca = HeatCostAllocator::new(ObisCode::CLOCK, DlmsData::DoubleLong(0));
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(777));
        hca.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(hca.value().as_i32(), Some(777));
    }

    #[test]
    fn test_heat_cost_allocator_period() {
        let mut hca = HeatCostAllocator::new(ObisCode::CLOCK, DlmsData::DoubleLong(0));
        hca.set_integration_period(300);
        assert_eq!(hca.integration_period(), 300);
    }
}
