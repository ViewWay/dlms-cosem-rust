//! IC112 Credit
//! Blue Book Ed16: class_id=112, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Credit - credit management for prepayment
pub struct Credit {
    logical_name: ObisCode,
    current_credit_amount: i32,
    credit_type: u8,
    priority: u8,
    warning_threshold: i32,
    limit: i32,
    status: u8,
    available_credit: i32,
}

impl Credit {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            current_credit_amount: 0,
            credit_type: 0,
            priority: 0,
            warning_threshold: 100,
            limit: 0,
            status: 0,
            available_credit: 0,
        }
    }

    pub fn current_credit_amount(&self) -> i32 {
        self.current_credit_amount
    }
    pub fn set_current_credit_amount(&mut self, amount: i32) {
        self.current_credit_amount = amount;
    }
    pub fn available_credit(&self) -> i32 {
        self.available_credit
    }
    pub fn set_available_credit(&mut self, credit: i32) {
        self.available_credit = credit;
    }
}

impl CosemObject for Credit {
    fn class_id(&self) -> u16 {
        112
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        12
    }
    fn method_count(&self) -> u8 {
        2
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
                self.current_credit_amount,
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.credit_type))),
            5 => Some(dlms_axdr::encode(&DlmsData::DoubleLong(
                self.warning_threshold,
            ))),
            8 => Some(dlms_axdr::encode(&DlmsData::DoubleLong(
                self.available_credit,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = d.as_i32() {
                    self.current_credit_amount = v;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }

    fn execute_action(&mut self, method_id: u8, _data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        match method_id {
            1 | 2 => Ok(vec![]),
            _ => Err(CosemObjectError::MethodNotSupported(method_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_new() {
        let c = Credit::new(ObisCode::CLOCK);
        assert_eq!(c.class_id(), 112);
    }

    #[test]
    fn test_credit_amount() {
        let mut c = Credit::new(ObisCode::CLOCK);
        c.set_current_credit_amount(5000);
        assert_eq!(c.current_credit_amount(), 5000);
    }

    #[test]
    fn test_credit_roundtrip() {
        let mut c = Credit::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(9999));
        c.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(c.current_credit_amount(), 9999);
    }
}
