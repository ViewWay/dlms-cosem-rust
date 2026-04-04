//! IC111 Account
//! Blue Book Ed16: class_id=111, version=0
//! Payment metering account management

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Account - payment metering account
pub struct AccountBluebook {
    logical_name: ObisCode,
    payer_name: String,
    status: u8,
    current_credit_in_use: i32,
    current_credit_status: u8,
    available_credit: i32,
    amount_type: u8,
    amount: i32,
}

impl AccountBluebook {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            payer_name: String::new(),
            status: 0,
            current_credit_in_use: 0,
            current_credit_status: 0,
            available_credit: 0,
            amount_type: 0,
            amount: 0,
        }
    }

    pub fn payer_name(&self) -> &str {
        &self.payer_name
    }
    pub fn set_payer_name(&mut self, name: String) {
        self.payer_name = name;
    }
    pub fn status(&self) -> u8 {
        self.status
    }
    pub fn available_credit(&self) -> i32 {
        self.available_credit
    }
    pub fn set_available_credit(&mut self, credit: i32) {
        self.available_credit = credit;
    }
}

impl CosemObject for AccountBluebook {
    fn class_id(&self) -> u16 {
        111
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        14
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
            2 => Some(dlms_axdr::encode(&DlmsData::VisibleString(
                self.payer_name.clone(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.status))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLong(
                self.current_credit_in_use,
            ))),
            6 => Some(dlms_axdr::encode(&DlmsData::DoubleLong(
                self.available_credit,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            3 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = d.as_u8() {
                    self.status = v;
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
    fn test_account_new() {
        let a = AccountBluebook::new(ObisCode::CLOCK);
        assert_eq!(a.class_id(), 111);
    }

    #[test]
    fn test_account_payer() {
        let mut a = AccountBluebook::new(ObisCode::CLOCK);
        a.set_payer_name("John Doe".to_string());
        assert_eq!(a.payer_name(), "John Doe");
    }

    #[test]
    fn test_account_credit() {
        let mut a = AccountBluebook::new(ObisCode::CLOCK);
        a.set_available_credit(10000);
        assert_eq!(a.available_credit(), 10000);
    }

    #[test]
    fn test_account_roundtrip() {
        let mut a = AccountBluebook::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Unsigned(5));
        a.attribute_from_bytes(3, &bytes).unwrap();
        assert_eq!(a.status(), 5);
    }
}
