//! IC105 TLS Setup - TLS Security Configuration

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// TLS Setup - TLS Security Configuration Object
/// 
/// This class manages TLS/SSL configuration for secure communications.
/// Used in secure metering and IoT systems.
#[derive(Debug, Clone)]
pub struct TlsSetup {
    logical_name: ObisCode,
    tls_version: u8,
    cipher_suites: Vec<u16>,
    certificate_reference: String,
    verify_peer: bool,
    session_timeout: u32,
    max_sessions: u8,
}

/// TLS version enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TlsVersion {
    /// TLS 1.0 (deprecated)
    Tls10 = 0,
    /// TLS 1.1 (deprecated)
    Tls11 = 1,
    /// TLS 1.2
    Tls12 = 2,
    /// TLS 1.3
    Tls13 = 3,
}

impl TlsVersion {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => TlsVersion::Tls10,
            1 => TlsVersion::Tls11,
            2 => TlsVersion::Tls12,
            _ => TlsVersion::Tls13,
        }
    }
}

impl TlsSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            tls_version: TlsVersion::Tls12 as u8,
            cipher_suites: Vec::new(),
            certificate_reference: String::new(),
            verify_peer: true,
            session_timeout: 3600,
            max_sessions: 10,
        }
    }

    pub fn tls_version(&self) -> TlsVersion {
        TlsVersion::from_u8(self.tls_version)
    }

    pub fn set_tls_version(&mut self, version: TlsVersion) {
        self.tls_version = version as u8;
    }

    pub fn cipher_suites(&self) -> &[u16] {
        &self.cipher_suites
    }

    pub fn add_cipher_suite(&mut self, suite: u16) {
        if !self.cipher_suites.contains(&suite) {
            self.cipher_suites.push(suite);
        }
    }

    pub fn clear_cipher_suites(&mut self) {
        self.cipher_suites.clear();
    }

    pub fn certificate_reference(&self) -> &str {
        &self.certificate_reference
    }

    pub fn set_certificate_reference(&mut self, reference: String) {
        self.certificate_reference = reference;
    }

    pub fn verify_peer(&self) -> bool {
        self.verify_peer
    }

    pub fn set_verify_peer(&mut self, verify: bool) {
        self.verify_peer = verify;
    }

    pub fn session_timeout(&self) -> u32 {
        self.session_timeout
    }

    pub fn set_session_timeout(&mut self, timeout: u32) {
        self.session_timeout = timeout;
    }

    pub fn max_sessions(&self) -> u8 {
        self.max_sessions
    }

    pub fn set_max_sessions(&mut self, max: u8) {
        self.max_sessions = max;
    }
}

impl CosemObject for TlsSetup {
    fn class_id(&self) -> u16 {
        105
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        7
    }

    fn method_count(&self) -> u8 {
        2
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::Enum(self.tls_version))),
            3 => {
                let suites: Vec<DlmsData> = self
                    .cipher_suites
                    .iter()
                    .map(|&s| DlmsData::LongUnsigned(s))
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(suites)))
            }
            4 => Some(dlms_axdr::encode(&DlmsData::VisibleString(
                self.certificate_reference.clone(),
            ))),
            5 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.verify_peer))),
            6 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.session_timeout,
            ))),
            7 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.max_sessions))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(version) = decoded {
                    self.tls_version = version;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::VisibleString(reference) = decoded {
                    self.certificate_reference = reference;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            5 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Boolean(verify) = decoded {
                    self.verify_peer = verify;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            6 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::DoubleLongUnsigned(timeout) = decoded {
                    self.session_timeout = timeout;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            7 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(max) = decoded {
                    self.max_sessions = max;
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
                // Initialize TLS session
                Ok(vec![0x00, 0x00])
            }
            2 => {
                // Close all TLS sessions
                Ok(vec![0x00, 0x00])
            }
            _ => Err(CosemObjectError::MethodNotSupported(method_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_setup_new() {
        let tls = TlsSetup::new(ObisCode::new(0, 0, 105, 0, 0, 255));
        assert_eq!(tls.class_id(), 105);
    }

    #[test]
    fn test_tls_setup_version() {
        let mut tls = TlsSetup::new(ObisCode::new(0, 0, 105, 0, 0, 255));
        tls.set_tls_version(TlsVersion::Tls13);
        assert_eq!(tls.tls_version(), TlsVersion::Tls13);
    }

    #[test]
    fn test_tls_setup_cipher_suites() {
        let mut tls = TlsSetup::new(ObisCode::new(0, 0, 105, 0, 0, 255));
        tls.add_cipher_suite(0xC02F); // TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256
        tls.add_cipher_suite(0xC030);
        assert_eq!(tls.cipher_suites().len(), 2);
    }

    #[test]
    fn test_tls_setup_cipher_suites_no_duplicates() {
        let mut tls = TlsSetup::new(ObisCode::new(0, 0, 105, 0, 0, 255));
        tls.add_cipher_suite(0xC02F);
        tls.add_cipher_suite(0xC02F);
        assert_eq!(tls.cipher_suites().len(), 1);
    }

    #[test]
    fn test_tls_setup_certificate_reference() {
        let mut tls = TlsSetup::new(ObisCode::new(0, 0, 105, 0, 0, 255));
        tls.set_certificate_reference("cert-001".to_string());
        assert_eq!(tls.certificate_reference(), "cert-001");
    }

    #[test]
    fn test_tls_setup_verify_peer() {
        let mut tls = TlsSetup::new(ObisCode::new(0, 0, 105, 0, 0, 255));
        assert!(tls.verify_peer());
        tls.set_verify_peer(false);
        assert!(!tls.verify_peer());
    }

    #[test]
    fn test_tls_setup_session_timeout() {
        let mut tls = TlsSetup::new(ObisCode::new(0, 0, 105, 0, 0, 255));
        tls.set_session_timeout(7200);
        assert_eq!(tls.session_timeout(), 7200);
    }

    #[test]
    fn test_tls_setup_max_sessions() {
        let mut tls = TlsSetup::new(ObisCode::new(0, 0, 105, 0, 0, 255));
        tls.set_max_sessions(5);
        assert_eq!(tls.max_sessions(), 5);
    }

    #[test]
    fn test_tls_setup_attribute_count() {
        let tls = TlsSetup::new(ObisCode::new(0, 0, 105, 0, 0, 255));
        assert_eq!(tls.attribute_count(), 7);
    }

    #[test]
    fn test_tls_setup_method_count() {
        let tls = TlsSetup::new(ObisCode::new(0, 0, 105, 0, 0, 255));
        assert_eq!(tls.method_count(), 2);
    }

    #[test]
    fn test_tls_version_from_u8() {
        assert_eq!(TlsVersion::from_u8(0), TlsVersion::Tls10);
        assert_eq!(TlsVersion::from_u8(1), TlsVersion::Tls11);
        assert_eq!(TlsVersion::from_u8(2), TlsVersion::Tls12);
        assert_eq!(TlsVersion::from_u8(3), TlsVersion::Tls13);
    }

    #[test]
    fn test_tls_setup_attr_roundtrip() {
        let mut tls = TlsSetup::new(ObisCode::new(0, 0, 105, 0, 0, 255));
        let encoded = dlms_axdr::encode(&DlmsData::Enum(3));
        tls.attribute_from_bytes(2, &encoded).unwrap();
        assert_eq!(tls.tls_version(), TlsVersion::Tls13);
    }
}
