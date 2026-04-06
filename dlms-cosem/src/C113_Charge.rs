//! IC113 Charge
//! Blue Book Ed16: class_id=113, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Charge - charge management
pub struct Charge {
    logical_name: ObisCode,
    total_amount_paid: i32,
    charge_type: u8,
    priority: u8,
    unit_charge: i32,
    status: u8,
}

impl Charge {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            total_amount_paid: 0,
            charge_type: 0,
            priority: 0,
            unit_charge: 0,
            status: 0,
        }
    }

    pub fn total_amount_paid(&self) -> i32 {
        self.total_amount_paid
    }
    pub fn set_total_amount_paid(&mut self, amount: i32) {
        self.total_amount_paid = amount;
    }
    pub fn unit_charge(&self) -> i32 {
        self.unit_charge
    }
    pub fn set_unit_charge(&mut self, charge: i32) {
        self.unit_charge = charge;
    }
}

impl CosemObject for Charge {
    fn class_id(&self) -> u16 {
        113
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        8
    }
    fn method_count(&self) -> u8 {
        1
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLong(
                self.total_amount_paid,
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.charge_type))),
            4 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.priority))),
            5 => Some(dlms_axdr::encode(&DlmsData::DoubleLong(self.unit_charge))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = d.as_i32() {
                    self.total_amount_paid = v;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }

    fn execute_action(&mut self, method_id: u8, _data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        if method_id == 1 {
            Ok(vec![])
        } else {
            Err(CosemObjectError::MethodNotSupported(method_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charge_new() {
        let c = Charge::new(ObisCode::CLOCK);
        assert_eq!(c.class_id(), 113);
    }

    #[test]
    fn test_charge_amount() {
        let mut c = Charge::new(ObisCode::CLOCK);
        c.set_total_amount_paid(1000);
        assert_eq!(c.total_amount_paid(), 1000);
    }

    #[test]
    fn test_charge_roundtrip() {
        let mut c = Charge::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(500));
        c.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(c.total_amount_paid(), 500);
    }
}
