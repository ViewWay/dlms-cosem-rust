//! IC103 Ethernet Setup - IEEE 802.3 Ethernet Configuration

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Ethernet Setup - IEEE 802.3 Ethernet Configuration Object
/// 
/// This class manages Ethernet (IEEE 802.3) network configuration.
/// Used in wired network metering installations.
#[derive(Debug, Clone)]
pub struct EthernetSetup {
    logical_name: ObisCode,
    mac_address: [u8; 6],
    link_speed: u32,
    duplex_mode: u8,
    auto_negotiation: bool,
    link_status: bool,
    mtu: u16,
}

/// Duplex mode selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DuplexMode {
    /// Half duplex
    Half = 0,
    /// Full duplex
    Full = 1,
}

impl DuplexMode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => DuplexMode::Half,
            _ => DuplexMode::Full,
        }
    }
}

impl EthernetSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            mac_address: [0x00; 6],
            link_speed: 100_000_000, // 100 Mbps default
            duplex_mode: DuplexMode::Full as u8,
            auto_negotiation: true,
            link_status: false,
            mtu: 1500,
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

    pub fn link_speed(&self) -> u32 {
        self.link_speed
    }

    pub fn set_link_speed(&mut self, speed: u32) {
        self.link_speed = speed;
    }

    pub fn link_speed_mbps(&self) -> u32 {
        self.link_speed / 1_000_000
    }

    pub fn duplex_mode(&self) -> DuplexMode {
        DuplexMode::from_u8(self.duplex_mode)
    }

    pub fn set_duplex_mode(&mut self, mode: DuplexMode) {
        self.duplex_mode = mode as u8;
    }

    pub fn auto_negotiation(&self) -> bool {
        self.auto_negotiation
    }

    pub fn set_auto_negotiation(&mut self, enabled: bool) {
        self.auto_negotiation = enabled;
    }

    pub fn link_status(&self) -> bool {
        self.link_status
    }

    pub fn set_link_status(&mut self, status: bool) {
        self.link_status = status;
    }

    pub fn mtu(&self) -> u16 {
        self.mtu
    }

    pub fn set_mtu(&mut self, mtu: u16) {
        self.mtu = mtu;
    }

    pub fn is_connected(&self) -> bool {
        self.link_status
    }
}

impl CosemObject for EthernetSetup {
    fn class_id(&self) -> u16 {
        103
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
                self.mac_address.to_vec(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.link_speed,
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::Enum(self.duplex_mode))),
            5 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.auto_negotiation))),
            6 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.link_status))),
            7 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.mtu))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::OctetString(mac) = decoded {
                    if mac.len() == 6 {
                        self.mac_address.copy_from_slice(&mac);
                        Ok(())
                    } else {
                        Err(CosemObjectError::InvalidData)
                    }
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::DoubleLongUnsigned(speed) = decoded {
                    self.link_speed = speed;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(mode) = decoded {
                    self.duplex_mode = mode;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            5 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Boolean(auto) = decoded {
                    self.auto_negotiation = auto;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            7 => {
                if let DlmsData::LongUnsigned(mtu) = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)? {
                    self.mtu = mtu;
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
    fn test_ethernet_setup_new() {
        let eth = EthernetSetup::new(ObisCode::new(0, 0, 103, 0, 0, 255));
        assert_eq!(eth.class_id(), 103);
    }

    #[test]
    fn test_ethernet_setup_mac_address() {
        let mut eth = EthernetSetup::new(ObisCode::new(0, 0, 103, 0, 0, 255));
        eth.set_mac_address([0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
        assert_eq!(eth.mac_string(), "AA:BB:CC:DD:EE:FF");
    }

    #[test]
    fn test_ethernet_setup_link_speed() {
        let mut eth = EthernetSetup::new(ObisCode::new(0, 0, 103, 0, 0, 255));
        eth.set_link_speed(1_000_000_000); // 1 Gbps
        assert_eq!(eth.link_speed(), 1_000_000_000);
        assert_eq!(eth.link_speed_mbps(), 1000);
    }

    #[test]
    fn test_ethernet_setup_duplex_mode() {
        let mut eth = EthernetSetup::new(ObisCode::new(0, 0, 103, 0, 0, 255));
        eth.set_duplex_mode(DuplexMode::Full);
        assert_eq!(eth.duplex_mode(), DuplexMode::Full);
    }

    #[test]
    fn test_ethernet_setup_auto_negotiation() {
        let mut eth = EthernetSetup::new(ObisCode::new(0, 0, 103, 0, 0, 255));
        assert!(eth.auto_negotiation());
        eth.set_auto_negotiation(false);
        assert!(!eth.auto_negotiation());
    }

    #[test]
    fn test_ethernet_setup_link_status() {
        let mut eth = EthernetSetup::new(ObisCode::new(0, 0, 103, 0, 0, 255));
        assert!(!eth.is_connected());
        eth.set_link_status(true);
        assert!(eth.is_connected());
    }

    #[test]
    fn test_ethernet_setup_mtu() {
        let mut eth = EthernetSetup::new(ObisCode::new(0, 0, 103, 0, 0, 255));
        eth.set_mtu(9000); // Jumbo frames
        assert_eq!(eth.mtu(), 9000);
    }

    #[test]
    fn test_ethernet_setup_attribute_count() {
        let eth = EthernetSetup::new(ObisCode::new(0, 0, 103, 0, 0, 255));
        assert_eq!(eth.attribute_count(), 7);
    }

    #[test]
    fn test_ethernet_setup_method_count() {
        let eth = EthernetSetup::new(ObisCode::new(0, 0, 103, 0, 0, 255));
        assert_eq!(eth.method_count(), 0);
    }

    #[test]
    fn test_duplex_mode_from_u8() {
        assert_eq!(DuplexMode::from_u8(0), DuplexMode::Half);
        assert_eq!(DuplexMode::from_u8(1), DuplexMode::Full);
    }

    #[test]
    fn test_ethernet_setup_mac_roundtrip() {
        let mut eth = EthernetSetup::new(ObisCode::new(0, 0, 103, 0, 0, 255));
        let encoded = dlms_axdr::encode(&DlmsData::OctetString(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06]));
        eth.attribute_from_bytes(2, &encoded).unwrap();
        assert_eq!(*eth.mac_address(), [0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
    }
}
