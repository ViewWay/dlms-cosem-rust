//! IC115 Token Gateway
//! Blue Book Ed16: class_id=115, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Token Gateway - processes payment tokens
pub struct TokenGateway {
    logical_name: ObisCode,
    token: Vec<u8>,
    token_time: DlmsData,
    token_status: u8,
}

impl TokenGateway {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            token: vec![],
            token_time: DlmsData::OctetString(vec![0xFF; 12]),
            token_status: 0,
        }
    }
    pub fn token_status(&self) -> u8 {
        self.token_status
    }
    pub fn set_token(&mut self, token: Vec<u8>) {
        self.token = token;
    }
}

impl CosemObject for TokenGateway {
    fn class_id(&self) -> u16 {
        115
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
                let n = self.logical_name.to_bytes();
                Some(vec![0x09, 0x06, n[0], n[1], n[2], n[3], n[4], n[5]])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.token.clone(),
            ))),
            3 => Some(dlms_axdr::encode(&self.token_time)),
            4 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.token_status))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(b) = d.as_octet_string() {
                    self.token = b.to_vec();
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
    fn test_token_gateway_new() {
        let t = TokenGateway::new(ObisCode::CLOCK);
        assert_eq!(t.class_id(), 115);
    }
    #[test]
    fn test_token_gateway_set() {
        let mut t = TokenGateway::new(ObisCode::CLOCK);
        t.set_token(vec![1, 2, 3]);
        assert_eq!(t.token.len(), 3);
    }
    #[test]
    fn test_token_gateway_roundtrip() {
        let mut t = TokenGateway::new(ObisCode::CLOCK);
        let b = dlms_axdr::encode(&DlmsData::OctetString(vec![0xDE, 0xAD]));
        t.attribute_from_bytes(2, &b).unwrap();
        assert_eq!(t.token, &[0xDE, 0xAD]);
    }
}
