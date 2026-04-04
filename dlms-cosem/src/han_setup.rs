//! IC052 HAN (Home Area Network)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// HAN Setup - Home Area Network configuration
pub struct HanSetup {
    logical_name: ObisCode,
    enabled: bool,
    mac_address: [u8; 6],
    network_key: Vec<u8>,
}

impl HanSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            enabled: false,
            mac_address: [0x00; 6],
            network_key: vec![],
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn mac_address(&self) -> &[u8; 6] {
        &self.mac_address
    }

    pub fn set_mac_address(&mut self, mac: [u8; 6]) {
        self.mac_address = mac;
    }

    pub fn network_key(&self) -> &[u8] {
        &self.network_key
    }

    pub fn set_network_key(&mut self, key: Vec<u8>) {
        self.network_key = key;
    }
}

impl CosemObject for HanSetup {
    fn class_id(&self) -> u16 {
        209
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        5
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
            2 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.enabled))),
            3 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.mac_address.to_vec(),
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.network_key.clone(),
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(b) = decoded.as_bool() {
                    self.enabled = b;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(bytes) = decoded.as_octet_string() {
                    if bytes.len() == 6 {
                        self.mac_address.copy_from_slice(bytes);
                        Ok(())
                    } else {
                        Err(CosemObjectError::InvalidData)
                    }
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(bytes) = decoded.as_octet_string() {
                    self.network_key = bytes.to_vec();
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
    fn test_han_setup_new() {
        let han = HanSetup::new(ObisCode::CLOCK);
        assert_eq!(han.class_id(), 209);
        assert!(!han.enabled());
    }

    #[test]
    fn test_han_setup_enable() {
        let mut han = HanSetup::new(ObisCode::CLOCK);
        han.set_enabled(true);
        assert!(han.enabled());
    }

    #[test]
    fn test_han_setup_roundtrip() {
        let mut han = HanSetup::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Boolean(true));
        han.attribute_from_bytes(2, &bytes).unwrap();
        assert!(han.enabled());
    }

    #[test]
    fn test_han_setup_network_key() {
        let mut han = HanSetup::new(ObisCode::CLOCK);
        han.set_network_key(vec![0x01, 0x02, 0x03, 0x04]);
        assert_eq!(han.network_key().len(), 4);
    }
}
