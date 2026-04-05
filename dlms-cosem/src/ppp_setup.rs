//! PPP Setup - Point-to-Point Protocol Configuration

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// PPP Setup - Point-to-Point Protocol Configuration Object
/// 
/// This class manages PPP connection configuration.
/// Used in dial-up and serial communication systems.
#[derive(Debug, Clone)]
pub struct PppSetup {
    logical_name: ObisCode,
    username: String,
    password: String,
    phone_number: String,
    authentication_protocol: PppAuthProtocol,
    enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PppAuthProtocol {
    None = 0,
    PAP = 1,
    CHAP = 2,
}

impl PppSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            username: String::new(),
            password: String::new(),
            phone_number: String::new(),
            authentication_protocol: PppAuthProtocol::None,
            enabled: false,
        }
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn phone_number(&self) -> &str {
        &self.phone_number
    }

    pub fn set_phone_number(&mut self, number: String) {
        self.phone_number = number;
    }

    pub fn authentication_protocol(&self) -> PppAuthProtocol {
        self.authentication_protocol
    }

    pub fn set_authentication_protocol(&mut self, protocol: PppAuthProtocol) {
        self.authentication_protocol = protocol;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl CosemObject for PppSetup {
    fn class_id(&self) -> u16 {
        44
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        6
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
            2 => {
                // Username
                let mut bytes = vec![0x09];
                bytes.push(self.username.len() as u8);
                bytes.extend_from_slice(self.username.as_bytes());
                Some(bytes)
            }
            3 => {
                // Password
                let mut bytes = vec![0x09];
                bytes.push(self.password.len() as u8);
                bytes.extend_from_slice(self.password.as_bytes());
                Some(bytes)
            }
            4 => {
                // Phone number
                let mut bytes = vec![0x09];
                bytes.push(self.phone_number.len() as u8);
                bytes.extend_from_slice(self.phone_number.as_bytes());
                Some(bytes)
            }
            5 => {
                // Authentication protocol
                Some(vec![0x0F, self.authentication_protocol as u8])
            }
            6 => {
                // Enabled
                Some(vec![0x0F, if self.enabled { 1 } else { 0 }])
            }
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 | 3 | 4 => {
                // Parse octet string
                if data.len() > 2 {
                    let len = data[1] as usize;
                    if data.len() >= 2 + len {
                        let value = String::from_utf8_lossy(&data[2..2 + len]).to_string();
                        match attr {
                            2 => self.username = value,
                            3 => self.password = value,
                            4 => self.phone_number = value,
                            _ => {}
                        }
                    }
                }
                Ok(())
            }
            5 => {
                if data.len() >= 2 {
                    self.authentication_protocol = match data[1] {
                        0 => PppAuthProtocol::None,
                        1 => PppAuthProtocol::PAP,
                        2 => PppAuthProtocol::CHAP,
                        _ => PppAuthProtocol::None,
                    };
                }
                Ok(())
            }
            6 => {
                if data.len() >= 2 {
                    self.enabled = data[1] != 0;
                }
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
    fn test_ppp_setup_new() {
        let ppp = PppSetup::new(ObisCode::new(0, 0, 44, 0, 0, 255));
        assert_eq!(ppp.class_id(), 44);
    }

    #[test]
    fn test_ppp_setup_credentials() {
        let mut ppp = PppSetup::new(ObisCode::new(0, 0, 44, 0, 0, 255));
        ppp.set_username("user123".to_string());
        ppp.set_password("pass456".to_string());
        assert_eq!(ppp.username(), "user123");
        assert_eq!(ppp.password(), "pass456");
    }

    #[test]
    fn test_ppp_setup_phone_number() {
        let mut ppp = PppSetup::new(ObisCode::new(0, 0, 44, 0, 0, 255));
        ppp.set_phone_number("+861234567890".to_string());
        assert_eq!(ppp.phone_number(), "+861234567890");
    }

    #[test]
    fn test_ppp_setup_auth_protocol() {
        let mut ppp = PppSetup::new(ObisCode::new(0, 0, 44, 0, 0, 255));
        ppp.set_authentication_protocol(PppAuthProtocol::CHAP);
        assert_eq!(ppp.authentication_protocol(), PppAuthProtocol::CHAP);
    }

    #[test]
    fn test_ppp_setup_enabled() {
        let mut ppp = PppSetup::new(ObisCode::new(0, 0, 44, 0, 0, 255));
        ppp.set_enabled(true);
        assert!(ppp.is_enabled());
    }

    #[test]
    fn test_ppp_setup_attribute_count() {
        let ppp = PppSetup::new(ObisCode::new(0, 0, 44, 0, 0, 255));
        assert_eq!(ppp.attribute_count(), 6);
    }

    #[test]
    fn test_ppp_setup_method_count() {
        let ppp = PppSetup::new(ObisCode::new(0, 0, 44, 0, 0, 255));
        assert_eq!(ppp.method_count(), 0);
    }
}
