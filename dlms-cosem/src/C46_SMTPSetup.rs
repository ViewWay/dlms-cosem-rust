//! IC46 SMTP Setup - Email Notification Configuration

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IC46 SMTP Setup - Email Notification Configuration Object
/// 
/// This class manages SMTP server configuration for email notifications.
/// Used in smart metering and IoT systems for alert notifications.
#[derive(Debug, Clone)]
pub struct SmtpSetup {
    logical_name: ObisCode,
    server_address: String,
    server_port: u16,
    username: String,
    password: String,
    sender_address: String,
    use_tls: bool,
    enabled: bool,
}

impl SmtpSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            server_address: String::new(),
            server_port: 25,
            username: String::new(),
            password: String::new(),
            sender_address: String::new(),
            use_tls: false,
            enabled: false,
        }
    }

    pub fn server_address(&self) -> &str {
        &self.server_address
    }

    pub fn set_server_address(&mut self, address: String) {
        self.server_address = address;
    }

    pub fn server_port(&self) -> u16 {
        self.server_port
    }

    pub fn set_server_port(&mut self, port: u16) {
        self.server_port = port;
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

    pub fn sender_address(&self) -> &str {
        &self.sender_address
    }

    pub fn set_sender_address(&mut self, address: String) {
        self.sender_address = address;
    }

    pub fn use_tls(&self) -> bool {
        self.use_tls
    }

    pub fn set_use_tls(&mut self, use_tls: bool) {
        self.use_tls = use_tls;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl CosemObject for SmtpSetup {
    fn class_id(&self) -> u16 {
        46
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
            2 => {
                // Server address
                let mut bytes = vec![0x09];
                bytes.push(self.server_address.len() as u8);
                bytes.extend_from_slice(self.server_address.as_bytes());
                Some(bytes)
            }
            3 => {
                // Server port
                let mut bytes = vec![0x12];
                bytes.extend_from_slice(&self.server_port.to_be_bytes());
                Some(bytes)
            }
            4 => {
                // Username
                let mut bytes = vec![0x09];
                bytes.push(self.username.len() as u8);
                bytes.extend_from_slice(self.username.as_bytes());
                Some(bytes)
            }
            5 => {
                // Password
                let mut bytes = vec![0x09];
                bytes.push(self.password.len() as u8);
                bytes.extend_from_slice(self.password.as_bytes());
                Some(bytes)
            }
            6 => {
                // Sender address
                let mut bytes = vec![0x09];
                bytes.push(self.sender_address.len() as u8);
                bytes.extend_from_slice(self.sender_address.as_bytes());
                Some(bytes)
            }
            7 => {
                // Use TLS
                Some(vec![0x0F, if self.use_tls { 1 } else { 0 }])
            }
            8 => {
                // Enabled
                Some(vec![0x0F, if self.enabled { 1 } else { 0 }])
            }
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 | 4 | 5 | 6 => {
                // Parse octet string
                if data.len() > 2 {
                    let len = data[1] as usize;
                    if data.len() >= 2 + len {
                        let value = String::from_utf8_lossy(&data[2..2 + len]).to_string();
                        match attr {
                            2 => self.server_address = value,
                            4 => self.username = value,
                            5 => self.password = value,
                            6 => self.sender_address = value,
                            _ => {}
                        }
                    }
                }
                Ok(())
            }
            3 => {
                // Parse port
                if data.len() >= 3 {
                    self.server_port = u16::from_be_bytes([data[1], data[2]]);
                }
                Ok(())
            }
            7 => {
                if data.len() >= 2 {
                    self.use_tls = data[1] != 0;
                }
                Ok(())
            }
            8 => {
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
    fn test_smtp_setup_new() {
        let smtp = SmtpSetup::new(ObisCode::new(0, 0, 46, 0, 0, 255));
        assert_eq!(smtp.class_id(), 46);
    }

    #[test]
    fn test_smtp_setup_server_address() {
        let mut smtp = SmtpSetup::new(ObisCode::new(0, 0, 46, 0, 0, 255));
        smtp.set_server_address("smtp.example.com".to_string());
        assert_eq!(smtp.server_address(), "smtp.example.com");
    }

    #[test]
    fn test_smtp_setup_server_port() {
        let mut smtp = SmtpSetup::new(ObisCode::new(0, 0, 46, 0, 0, 255));
        smtp.set_server_port(587);
        assert_eq!(smtp.server_port(), 587);
    }

    #[test]
    fn test_smtp_setup_credentials() {
        let mut smtp = SmtpSetup::new(ObisCode::new(0, 0, 46, 0, 0, 255));
        smtp.set_username("user@example.com".to_string());
        smtp.set_password("secret123".to_string());
        assert_eq!(smtp.username(), "user@example.com");
        assert_eq!(smtp.password(), "secret123");
    }

    #[test]
    fn test_smtp_setup_sender() {
        let mut smtp = SmtpSetup::new(ObisCode::new(0, 0, 46, 0, 0, 255));
        smtp.set_sender_address("noreply@example.com".to_string());
        assert_eq!(smtp.sender_address(), "noreply@example.com");
    }

    #[test]
    fn test_smtp_setup_tls() {
        let mut smtp = SmtpSetup::new(ObisCode::new(0, 0, 46, 0, 0, 255));
        smtp.set_use_tls(true);
        assert!(smtp.use_tls());
    }

    #[test]
    fn test_smtp_setup_enabled() {
        let mut smtp = SmtpSetup::new(ObisCode::new(0, 0, 46, 0, 0, 255));
        smtp.set_enabled(true);
        assert!(smtp.is_enabled());
    }

    #[test]
    fn test_smtp_setup_attribute_count() {
        let smtp = SmtpSetup::new(ObisCode::new(0, 0, 46, 0, 0, 255));
        assert_eq!(smtp.attribute_count(), 8);
    }

    #[test]
    fn test_smtp_setup_method_count() {
        let smtp = SmtpSetup::new(ObisCode::new(0, 0, 46, 0, 0, 255));
        assert_eq!(smtp.method_count(), 0);
    }
}
