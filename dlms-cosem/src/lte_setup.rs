//! IC104 LTE Setup - LTE Network Configuration

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// LTE Setup - LTE Network Configuration Object
/// 
/// This class manages LTE/4G network configuration for smart metering.
/// Used in cellular-connected metering devices.
#[derive(Debug, Clone)]
pub struct LteSetup {
    logical_name: ObisCode,
    apn: String,
    pdp_type: u8,
    qos_profile: String,
    pin_code: String,
    network_mode: u8,
    signal_strength: u8,
    registration_status: u8,
}

/// PDP (Packet Data Protocol) types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PdpType {
    /// IPv4 only
    Ipv4 = 0,
    /// IPv6 only
    Ipv6 = 1,
    /// IPv4 and IPv6 (dual stack)
    Ipv4v6 = 2,
}

impl PdpType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => PdpType::Ipv4,
            1 => PdpType::Ipv6,
            _ => PdpType::Ipv4v6,
        }
    }
}

/// Network mode selection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NetworkMode {
    /// Automatic selection
    Auto = 0,
    /// LTE only
    LteOnly = 1,
    /// 3G fallback
    LteWith3g = 2,
    /// 2G fallback
    LteWith2g = 3,
}

impl NetworkMode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => NetworkMode::Auto,
            1 => NetworkMode::LteOnly,
            2 => NetworkMode::LteWith3g,
            _ => NetworkMode::LteWith2g,
        }
    }
}

/// Registration status
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RegistrationStatus {
    /// Not registered
    NotRegistered = 0,
    /// Registered, home network
    RegisteredHome = 1,
    /// Searching for network
    Searching = 2,
    /// Registration denied
    Denied = 3,
    /// Registered, roaming
    RegisteredRoaming = 4,
}

impl RegistrationStatus {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => RegistrationStatus::NotRegistered,
            1 => RegistrationStatus::RegisteredHome,
            2 => RegistrationStatus::Searching,
            3 => RegistrationStatus::Denied,
            _ => RegistrationStatus::RegisteredRoaming,
        }
    }
}

impl LteSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            apn: String::new(),
            pdp_type: PdpType::Ipv4 as u8,
            qos_profile: String::new(),
            pin_code: String::new(),
            network_mode: NetworkMode::Auto as u8,
            signal_strength: 0,
            registration_status: RegistrationStatus::NotRegistered as u8,
        }
    }

    pub fn apn(&self) -> &str {
        &self.apn
    }

    pub fn set_apn(&mut self, apn: String) {
        self.apn = apn;
    }

    pub fn pdp_type(&self) -> PdpType {
        PdpType::from_u8(self.pdp_type)
    }

    pub fn set_pdp_type(&mut self, pdp_type: PdpType) {
        self.pdp_type = pdp_type as u8;
    }

    pub fn qos_profile(&self) -> &str {
        &self.qos_profile
    }

    pub fn set_qos_profile(&mut self, profile: String) {
        self.qos_profile = profile;
    }

    pub fn network_mode(&self) -> NetworkMode {
        NetworkMode::from_u8(self.network_mode)
    }

    pub fn set_network_mode(&mut self, mode: NetworkMode) {
        self.network_mode = mode as u8;
    }

    pub fn signal_strength(&self) -> u8 {
        self.signal_strength
    }

    pub fn registration_status(&self) -> RegistrationStatus {
        RegistrationStatus::from_u8(self.registration_status)
    }

    pub fn is_registered(&self) -> bool {
        matches!(
            self.registration_status(),
            RegistrationStatus::RegisteredHome | RegistrationStatus::RegisteredRoaming
        )
    }
}

impl CosemObject for LteSetup {
    fn class_id(&self) -> u16 {
        104
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        8
    }

    fn method_count(&self) -> u8 {
        1
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.apn.clone()))),
            3 => Some(dlms_axdr::encode(&DlmsData::Enum(self.pdp_type))),
            4 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.qos_profile.clone()))),
            5 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.pin_code.clone()))),
            6 => Some(dlms_axdr::encode(&DlmsData::Enum(self.network_mode))),
            7 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.signal_strength))),
            8 => Some(dlms_axdr::encode(&DlmsData::Enum(self.registration_status))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::VisibleString(apn) = decoded {
                    self.apn = apn;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(pdp) = decoded {
                    self.pdp_type = pdp;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::VisibleString(profile) = decoded {
                    self.qos_profile = profile;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            6 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(mode) = decoded {
                    self.network_mode = mode;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            7 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(ss) = decoded {
                    self.signal_strength = ss;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            8 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(status) = decoded {
                    self.registration_status = status;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }

    fn execute_action(&mut self, method_id: u8, _data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        match method_id {
            1 => {
                // Connect/initialize LTE
                Ok(vec![0x00, 0x00]) // success
            }
            _ => Err(CosemObjectError::MethodNotSupported(method_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lte_setup_new() {
        let lte = LteSetup::new(ObisCode::new(0, 0, 104, 0, 0, 255));
        assert_eq!(lte.class_id(), 104);
    }

    #[test]
    fn test_lte_setup_apn() {
        let mut lte = LteSetup::new(ObisCode::new(0, 0, 104, 0, 0, 255));
        lte.set_apn("internet.example.com".to_string());
        assert_eq!(lte.apn(), "internet.example.com");
    }

    #[test]
    fn test_lte_setup_pdp_type() {
        let mut lte = LteSetup::new(ObisCode::new(0, 0, 104, 0, 0, 255));
        lte.set_pdp_type(PdpType::Ipv6);
        assert_eq!(lte.pdp_type(), PdpType::Ipv6);
    }

    #[test]
    fn test_lte_setup_network_mode() {
        let mut lte = LteSetup::new(ObisCode::new(0, 0, 104, 0, 0, 255));
        lte.set_network_mode(NetworkMode::LteOnly);
        assert_eq!(lte.network_mode(), NetworkMode::LteOnly);
    }

    #[test]
    fn test_lte_setup_signal_strength() {
        let mut lte = LteSetup::new(ObisCode::new(0, 0, 104, 0, 0, 255));
        let encoded = dlms_axdr::encode(&DlmsData::Unsigned(75));
        lte.attribute_from_bytes(7, &encoded).unwrap();
        assert_eq!(lte.signal_strength(), 75);
    }

    #[test]
    fn test_lte_setup_is_registered() {
        let mut lte = LteSetup::new(ObisCode::new(0, 0, 104, 0, 0, 255));
        assert!(!lte.is_registered());
        
        let encoded = dlms_axdr::encode(&DlmsData::Enum(1));
        lte.attribute_from_bytes(8, &encoded).unwrap();
        assert!(lte.is_registered());
    }

    #[test]
    fn test_lte_setup_attribute_count() {
        let lte = LteSetup::new(ObisCode::new(0, 0, 104, 0, 0, 255));
        assert_eq!(lte.attribute_count(), 8);
    }

    #[test]
    fn test_lte_setup_method_count() {
        let lte = LteSetup::new(ObisCode::new(0, 0, 104, 0, 0, 255));
        assert_eq!(lte.method_count(), 1);
    }

    #[test]
    fn test_pdp_type_from_u8() {
        assert_eq!(PdpType::from_u8(0), PdpType::Ipv4);
        assert_eq!(PdpType::from_u8(1), PdpType::Ipv6);
        assert_eq!(PdpType::from_u8(2), PdpType::Ipv4v6);
    }

    #[test]
    fn test_network_mode_from_u8() {
        assert_eq!(NetworkMode::from_u8(0), NetworkMode::Auto);
        assert_eq!(NetworkMode::from_u8(1), NetworkMode::LteOnly);
        assert_eq!(NetworkMode::from_u8(2), NetworkMode::LteWith3g);
        assert_eq!(NetworkMode::from_u8(3), NetworkMode::LteWith2g);
    }

    #[test]
    fn test_registration_status_from_u8() {
        assert_eq!(RegistrationStatus::from_u8(0), RegistrationStatus::NotRegistered);
        assert_eq!(RegistrationStatus::from_u8(1), RegistrationStatus::RegisteredHome);
        assert_eq!(RegistrationStatus::from_u8(4), RegistrationStatus::RegisteredRoaming);
    }
}
