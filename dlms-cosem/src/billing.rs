//! IC017 Billing (IC17) - Billing / payment metering
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: charging_date_time (octet-string)
//! 3: billing_period (unsigned)
//! 4: billing_cycle (long-unsigned)
//! 5: last_billing_date_time (octet-string)
//! 6: amount_prescribed (double-long)
//! 7: amount_to_be_paid (double-long)
//! 8: debt_amount (double-long)
//! 9: credit_amount (double-long)
//! 10: charge_type (enum)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct Billing {
    logical_name: ObisCode,
    charging_datetime: DlmsData,
    billing_period: u8,
    billing_cycle: u16,
    last_billing_datetime: DlmsData,
    amount_prescribed: i32,
    amount_to_be_paid: i32,
    debt_amount: i32,
    credit_amount: i32,
    charge_type: u8,
}

impl Billing {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            charging_datetime: DlmsData::DateTime([0u8; 12]),
            billing_period: 1,
            billing_cycle: 30,
            last_billing_datetime: DlmsData::DateTime([0u8; 12]),
            amount_prescribed: 0,
            amount_to_be_paid: 0,
            debt_amount: 0,
            credit_amount: 0,
            charge_type: 0,
        }
    }

    pub fn amount_to_be_paid(&self) -> i32 {
        self.amount_to_be_paid
    }
    pub fn set_amount_to_be_paid(&mut self, val: i32) {
        self.amount_to_be_paid = val;
    }
}

impl CosemObject for Billing {
    fn class_id(&self) -> u16 {
        17
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        10
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
            2 => Some(dlms_axdr::encode(&self.charging_datetime)),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.billing_period))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.billing_cycle,
            ))),
            5 => Some(dlms_axdr::encode(&self.last_billing_datetime)),
            6 => Some(dlms_axdr::encode(&DlmsData::DoubleLong(
                self.amount_prescribed,
            ))),
            7 => Some(dlms_axdr::encode(&DlmsData::DoubleLong(
                self.amount_to_be_paid,
            ))),
            8 => Some(dlms_axdr::encode(&DlmsData::DoubleLong(self.debt_amount))),
            9 => Some(dlms_axdr::encode(&DlmsData::DoubleLong(self.credit_amount))),
            10 => Some(dlms_axdr::encode(&DlmsData::Enum(self.charge_type))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                self.charging_datetime =
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
    fn test_billing_class_id() {
        let b = Billing::new(ObisCode::new(0, 0, 17, 0, 0, 255));
        assert_eq!(b.class_id(), 17);
    }

    #[test]
    fn test_billing_attr_count() {
        let b = Billing::new(ObisCode::new(0, 0, 17, 0, 0, 255));
        assert_eq!(b.attribute_count(), 10);
    }

    #[test]
    fn test_billing_amount() {
        let mut b = Billing::new(ObisCode::new(0, 0, 17, 0, 0, 255));
        b.set_amount_to_be_paid(1000);
        assert_eq!(b.amount_to_be_paid(), 1000);
    }

    #[test]
    fn test_billing_attr7_encode() {
        let mut b = Billing::new(ObisCode::new(0, 0, 17, 0, 0, 255));
        b.set_amount_to_be_paid(500);
        let bytes = b.attribute_to_bytes(7).unwrap();
        assert!(!bytes.is_empty());
    }
}
