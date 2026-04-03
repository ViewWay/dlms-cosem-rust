//! IC070 Security Setup

use dlms_core::{CosemObject, ObisCode, DlmsData, CosemObjectError};

pub struct SecuritySetup {
    logical_name: ObisCode,
    security_suite: u8,
    security_policy: u8,
    #[allow(dead_code)] authentication_level: u8,
    #[allow(dead_code)] encryption_level: u8,
}

impl SecuritySetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name, security_suite: 0, security_policy: 0,
            #[allow(dead_code)] authentication_level: 0, encryption_level: 0,
        }
    }

    pub fn security_suite(&self) -> u8 { self.security_suite }
    pub fn set_security_suite(&mut self, suite: u8) { self.security_suite = suite; }
}

impl CosemObject for SecuritySetup {
    fn class_id(&self) -> u16 { 70 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 8 }
    fn method_count(&self) -> u8 { 4 }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5]])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::Enum(self.security_suite))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.security_policy as u16))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(v) = decoded {
                    self.security_suite = v;
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
    fn test_security_setup_class_id() { assert_eq!(SecuritySetup::new(ObisCode::new(0,0,43,0,0,255)).class_id(), 70); }
    #[test]
    fn test_security_setup_suite() {
        let mut s = SecuritySetup::new(ObisCode::new(0,0,43,0,0,255));
        s.set_security_suite(5);
        assert_eq!(s.security_suite(), 5);
    }
}
