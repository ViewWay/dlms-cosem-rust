//! IC64 Security Setup
//! Blue Book Ed16: class_id=64, version=1
//! Security policy, suite, keys, and certificates

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Security Setup - manages security parameters
pub struct SecuritySetupBluebook {
    logical_name: ObisCode,
    security_policy: u8,
    security_suite: u8,
    server_system_title: [u8; 8],
    certificates: Vec<u8>,
    global_unicast_encryption_key: Option<[u8; 16]>,
    global_broadcast_encryption_key: Option<[u8; 16]>,
    authentication_key: Option<[u8; 16]>,
}

impl SecuritySetupBluebook {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            security_policy: 0,
            security_suite: 0,
            server_system_title: [0u8; 8],
            certificates: vec![],
            global_unicast_encryption_key: None,
            global_broadcast_encryption_key: None,
            authentication_key: None,
        }
    }

    pub fn security_policy(&self) -> u8 {
        self.security_policy
    }
    pub fn set_security_policy(&mut self, policy: u8) {
        self.security_policy = policy;
    }
    pub fn security_suite(&self) -> u8 {
        self.security_suite
    }
    pub fn set_security_suite(&mut self, suite: u8) {
        self.security_suite = suite;
    }
    pub fn server_system_title(&self) -> &[u8; 8] {
        &self.server_system_title
    }
    pub fn set_server_system_title(&mut self, title: [u8; 8]) {
        self.server_system_title = title;
    }

    pub fn set_global_unicast_key(&mut self, key: [u8; 16]) {
        self.global_unicast_encryption_key = Some(key);
    }
    pub fn set_global_broadcast_key(&mut self, key: [u8; 16]) {
        self.global_broadcast_encryption_key = Some(key);
    }
    pub fn set_authentication_key(&mut self, key: [u8; 16]) {
        self.authentication_key = Some(key);
    }

    pub fn global_unicast_key(&self) -> Option<&[u8; 16]> {
        self.global_unicast_encryption_key.as_ref()
    }
    pub fn global_broadcast_key(&self) -> Option<&[u8; 16]> {
        self.global_broadcast_encryption_key.as_ref()
    }
    pub fn authentication_key(&self) -> Option<&[u8; 16]> {
        self.authentication_key.as_ref()
    }
}

impl CosemObject for SecuritySetupBluebook {
    fn class_id(&self) -> u16 {
        64
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
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.security_policy))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.security_suite))),
            4 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.server_system_title.to_vec(),
            ))),
            5 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.certificates.clone(),
            ))),
            7 => self
                .global_unicast_encryption_key
                .map(|k| dlms_axdr::encode(&DlmsData::OctetString(k.to_vec()))),
            8 => self
                .global_broadcast_encryption_key
                .map(|k| dlms_axdr::encode(&DlmsData::OctetString(k.to_vec()))),
            9 => self
                .authentication_key
                .map(|k| dlms_axdr::encode(&DlmsData::OctetString(k.to_vec()))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = d.as_u8() {
                    self.security_policy = v;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = d.as_u8() {
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
    fn test_security_setup_new() {
        let ss = SecuritySetupBluebook::new(ObisCode::CLOCK);
        assert_eq!(ss.class_id(), 64);
    }

    #[test]
    fn test_security_setup_policy() {
        let mut ss = SecuritySetupBluebook::new(ObisCode::CLOCK);
        ss.set_security_policy(5);
        assert_eq!(ss.security_policy(), 5);
    }

    #[test]
    fn test_security_setup_keys() {
        let mut ss = SecuritySetupBluebook::new(ObisCode::CLOCK);
        let key = [42u8; 16];
        ss.set_authentication_key(key);
        assert!(ss.authentication_key().is_some());
        assert_eq!(ss.authentication_key().unwrap()[0], 42);
    }

    #[test]
    fn test_security_setup_roundtrip() {
        let mut ss = SecuritySetupBluebook::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Unsigned(3));
        ss.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(ss.security_policy(), 3);
    }

    #[test]
    fn test_security_setup_system_title() {
        let mut ss = SecuritySetupBluebook::new(ObisCode::CLOCK);
        ss.set_server_system_title([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(ss.server_system_title()[0], 1);
    }
}
