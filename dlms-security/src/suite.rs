//! Security Suite definitions

/// DLMS/COSEM Security Suite
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecuritySuite {
    /// No security
    Suite0 = 0,
    /// Authentication only (HLS-ISM with password/LSK)
    Suite1 = 1,
    /// Encryption only (AES-128-GCM)
    Suite2 = 2,
    /// Authentication + Encryption
    Suite3 = 3,
    /// Digital Signature
    Suite4 = 4,
    /// Authentication + Encryption + Digital Signature (SM4-GCM)
    Suite5 = 5,
}

impl SecuritySuite {
    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(SecuritySuite::Suite0),
            1 => Some(SecuritySuite::Suite1),
            2 => Some(SecuritySuite::Suite2),
            3 => Some(SecuritySuite::Suite3),
            4 => Some(SecuritySuite::Suite4),
            5 => Some(SecuritySuite::Suite5),
            _ => None,
        }
    }

    pub fn id(&self) -> u8 {
        *self as u8
    }

    pub fn needs_authentication(&self) -> bool {
        matches!(self, SecuritySuite::Suite1 | SecuritySuite::Suite3 | SecuritySuite::Suite4 | SecuritySuite::Suite5)
    }

    pub fn needs_encryption(&self) -> bool {
        matches!(self, SecuritySuite::Suite2 | SecuritySuite::Suite3 | SecuritySuite::Suite5)
    }

    pub fn needs_signing(&self) -> bool {
        matches!(self, SecuritySuite::Suite4 | SecuritySuite::Suite5)
    }
}
