//! IC048 MAC Address Setup

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// MAC Address Setup - configuration for MAC layer
pub struct MacAddressSetup {
    logical_name: ObisCode,
    mac_address: [u8; 6],
}

impl MacAddressSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            mac_address: [0x00; 6],
        }
    }

    pub fn with_mac(logical_name: ObisCode, mac: [u8; 6]) -> Self {
        Self {
            logical_name,
            mac_address: mac,
        }
    }

    pub fn mac_address(&self) -> &[u8; 6] {
        &self.mac_address
    }

    pub fn set_mac_address(&mut self, mac: [u8; 6]) {
        self.mac_address = mac;
    }

    pub fn mac_string(&self) -> String {
        format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.mac_address[0],
            self.mac_address[1],
            self.mac_address[2],
            self.mac_address[3],
            self.mac_address[4],
            self.mac_address[5]
        )
    }
}

impl CosemObject for MacAddressSetup {
    fn class_id(&self) -> u16 {
        43
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        3
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
                self.mac_address.to_vec(),
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
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
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_address_setup_new() {
        let mac = MacAddressSetup::new(ObisCode::CLOCK);
        assert_eq!(mac.class_id(), 43);
    }

    #[test]
    fn test_mac_address_setup_mac() {
        let mac = MacAddressSetup::with_mac(ObisCode::CLOCK, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(mac.mac_string(), "AA:BB:CC:DD:EE:FF");
    }

    #[test]
    fn test_mac_address_setup_roundtrip() {
        let mut mac = MacAddressSetup::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::OctetString(vec![1, 2, 3, 4, 5, 6]));
        mac.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(*mac.mac_address(), [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_mac_address_setup_invalid_length() {
        let mut mac = MacAddressSetup::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::OctetString(vec![1, 2, 3]));
        assert!(mac.attribute_from_bytes(2, &bytes).is_err());
    }
}
