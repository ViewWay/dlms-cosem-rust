//! IC90 IEC Public Key - Public Key Management

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IEC Public Key - Public Key Management Object
/// 
/// This class manages public keys for cryptographic operations.
/// Used in security-sensitive metering applications.
#[derive(Debug, Clone)]
pub struct IecPublicKey {
    logical_name: ObisCode,
    public_key: Vec<u8>,
    key_id: String,
    algorithm: u8,
    key_usage: u8,
}

/// Key algorithm types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyAlgorithm {
    /// RSA algorithm
    Rsa = 0,
    /// ECC (Elliptic Curve Cryptography)
    Ecc = 1,
    /// AES (Advanced Encryption Standard)
    Aes = 2,
    /// Other/custom algorithm
    Other = 255,
}

impl KeyAlgorithm {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => KeyAlgorithm::Rsa,
            1 => KeyAlgorithm::Ecc,
            2 => KeyAlgorithm::Aes,
            _ => KeyAlgorithm::Other,
        }
    }
}

/// Key usage flags
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyUsage {
    /// Encryption only
    Encryption = 0,
    /// Signature only
    Signature = 1,
    /// Both encryption and signature
    Both = 2,
}

impl KeyUsage {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => KeyUsage::Encryption,
            1 => KeyUsage::Signature,
            _ => KeyUsage::Both,
        }
    }
}

impl IecPublicKey {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            public_key: Vec::new(),
            key_id: String::new(),
            algorithm: KeyAlgorithm::Rsa as u8,
            key_usage: KeyUsage::Both as u8,
        }
    }

    pub fn with_key(logical_name: ObisCode, public_key: Vec<u8>, key_id: String) -> Self {
        Self {
            logical_name,
            public_key,
            key_id,
            algorithm: KeyAlgorithm::Rsa as u8,
            key_usage: KeyUsage::Both as u8,
        }
    }

    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }

    pub fn set_public_key(&mut self, key: Vec<u8>) {
        self.public_key = key;
    }

    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    pub fn set_key_id(&mut self, id: String) {
        self.key_id = id;
    }

    pub fn algorithm(&self) -> KeyAlgorithm {
        KeyAlgorithm::from_u8(self.algorithm)
    }

    pub fn set_algorithm(&mut self, algorithm: KeyAlgorithm) {
        self.algorithm = algorithm as u8;
    }

    pub fn key_usage(&self) -> KeyUsage {
        KeyUsage::from_u8(self.key_usage)
    }

    pub fn set_key_usage(&mut self, usage: KeyUsage) {
        self.key_usage = usage as u8;
    }

    pub fn is_valid(&self) -> bool {
        !self.public_key.is_empty() && !self.key_id.is_empty()
    }
}

impl CosemObject for IecPublicKey {
    fn class_id(&self) -> u16 {
        90
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        5
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
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(self.public_key.clone()))),
            3 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.key_id.clone()))),
            4 => Some(dlms_axdr::encode(&DlmsData::Enum(self.algorithm))),
            5 => Some(dlms_axdr::encode(&DlmsData::Enum(self.key_usage))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::OctetString(key) = decoded {
                    self.public_key = key;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::VisibleString(id) = decoded {
                    self.key_id = id;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(alg) = decoded {
                    self.algorithm = alg;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            5 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(usage) = decoded {
                    self.key_usage = usage;
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
                // Clear/validate key
                if self.is_valid() {
                    Ok(vec![0x00, 0x00]) // success
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::MethodNotSupported(method_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iec_public_key_new() {
        let key = IecPublicKey::new(ObisCode::new(0, 0, 90, 0, 0, 255));
        assert_eq!(key.class_id(), 90);
    }

    #[test]
    fn test_iec_public_key_with_key() {
        let key_data = vec![0x01, 0x02, 0x03, 0x04];
        let key = IecPublicKey::with_key(
            ObisCode::new(0, 0, 90, 0, 0, 255),
            key_data.clone(),
            "key-001".to_string(),
        );
        assert_eq!(key.public_key(), &key_data);
        assert_eq!(key.key_id(), "key-001");
    }

    #[test]
    fn test_iec_public_key_algorithm() {
        let mut key = IecPublicKey::new(ObisCode::new(0, 0, 90, 0, 0, 255));
        key.set_algorithm(KeyAlgorithm::Ecc);
        assert_eq!(key.algorithm(), KeyAlgorithm::Ecc);
    }

    #[test]
    fn test_iec_public_key_usage() {
        let mut key = IecPublicKey::new(ObisCode::new(0, 0, 90, 0, 0, 255));
        key.set_key_usage(KeyUsage::Signature);
        assert_eq!(key.key_usage(), KeyUsage::Signature);
    }

    #[test]
    fn test_iec_public_key_is_valid() {
        let mut key = IecPublicKey::new(ObisCode::new(0, 0, 90, 0, 0, 255));
        assert!(!key.is_valid());
        
        key.set_public_key(vec![0x01, 0x02]);
        key.set_key_id("key-001".to_string());
        assert!(key.is_valid());
    }

    #[test]
    fn test_iec_public_key_attribute_count() {
        let key = IecPublicKey::new(ObisCode::new(0, 0, 90, 0, 0, 255));
        assert_eq!(key.attribute_count(), 5);
    }

    #[test]
    fn test_iec_public_key_method_count() {
        let key = IecPublicKey::new(ObisCode::new(0, 0, 90, 0, 0, 255));
        assert_eq!(key.method_count(), 1);
    }

    #[test]
    fn test_iec_public_key_attr2_roundtrip() {
        let mut key = IecPublicKey::new(ObisCode::new(0, 0, 90, 0, 0, 255));
        let encoded = dlms_axdr::encode(&DlmsData::OctetString(vec![0xAA, 0xBB, 0xCC]));
        key.attribute_from_bytes(2, &encoded).unwrap();
        assert_eq!(key.public_key(), &[0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn test_key_algorithm_from_u8() {
        assert_eq!(KeyAlgorithm::from_u8(0), KeyAlgorithm::Rsa);
        assert_eq!(KeyAlgorithm::from_u8(1), KeyAlgorithm::Ecc);
        assert_eq!(KeyAlgorithm::from_u8(2), KeyAlgorithm::Aes);
        assert_eq!(KeyAlgorithm::from_u8(99), KeyAlgorithm::Other);
    }

    #[test]
    fn test_key_usage_from_u8() {
        assert_eq!(KeyUsage::from_u8(0), KeyUsage::Encryption);
        assert_eq!(KeyUsage::from_u8(1), KeyUsage::Signature);
        assert_eq!(KeyUsage::from_u8(2), KeyUsage::Both);
    }
}
