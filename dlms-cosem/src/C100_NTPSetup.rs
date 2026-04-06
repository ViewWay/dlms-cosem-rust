//! IC100 NTP Setup
//! Blue Book Ed16: class_id=100, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// NTP Setup - Network Time Protocol configuration
pub struct NtpSetup {
    logical_name: ObisCode,
    ntp_server_address: String,
    ntp_port: u16,
    authentication_method: u8,
    randomization_interval: u32,
    time_offset: i32,
    time_threshold: u32,
}

impl NtpSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            ntp_server_address: "pool.ntp.org".to_string(),
            ntp_port: 123,
            authentication_method: 0,
            randomization_interval: 300,
            time_offset: 0,
            time_threshold: 1,
        }
    }

    pub fn ntp_server(&self) -> &str {
        &self.ntp_server_address
    }
    pub fn set_ntp_server(&mut self, addr: String) {
        self.ntp_server_address = addr;
    }
    pub fn ntp_port(&self) -> u16 {
        self.ntp_port
    }
    pub fn set_ntp_port(&mut self, port: u16) {
        self.ntp_port = port;
    }
}

impl CosemObject for NtpSetup {
    fn class_id(&self) -> u16 {
        100
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        7
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
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.ntp_server_address.as_bytes().to_vec(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.ntp_port))),
            4 => Some(dlms_axdr::encode(&DlmsData::Unsigned(
                self.authentication_method,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(bytes) = d.as_octet_string() {
                    self.ntp_server_address = String::from_utf8_lossy(bytes).to_string();
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
    fn test_ntp_setup_new() {
        let ntp = NtpSetup::new(ObisCode::CLOCK);
        assert_eq!(ntp.class_id(), 100);
        assert_eq!(ntp.ntp_port(), 123);
    }

    #[test]
    fn test_ntp_setup_server() {
        let mut ntp = NtpSetup::new(ObisCode::CLOCK);
        ntp.set_ntp_server("time.example.com".to_string());
        assert_eq!(ntp.ntp_server(), "time.example.com");
    }

    #[test]
    fn test_ntp_setup_roundtrip() {
        let mut ntp = NtpSetup::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::OctetString(b"ntp.example.com".to_vec()));
        ntp.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(ntp.ntp_server(), "ntp.example.com");
    }
}
