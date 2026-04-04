//! IC98 MPL Diagnostic
//! Blue Book Ed16: class_id=98, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// MPL Diagnostic - Multicast Protocol for Low-Power and Lossy Networks diagnostics
pub struct MplDiagnostic {
    logical_name: ObisCode,
    mpl_domain_id: u8,
    mpl_seed_set_version: u32,
    messages_sent: u32,
    messages_received: u32,
}

impl MplDiagnostic {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            mpl_domain_id: 0,
            mpl_seed_set_version: 0,
            messages_sent: 0,
            messages_received: 0,
        }
    }
    pub fn domain_id(&self) -> u8 {
        self.mpl_domain_id
    }
    pub fn set_domain_id(&mut self, id: u8) {
        self.mpl_domain_id = id;
    }
}

impl CosemObject for MplDiagnostic {
    fn class_id(&self) -> u16 {
        98
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
                let n = self.logical_name.to_bytes();
                Some(vec![0x09, 0x06, n[0], n[1], n[2], n[3], n[4], n[5]])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.mpl_domain_id))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.mpl_seed_set_version,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, _attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        Err(CosemObjectError::AttributeNotSupported(_attr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mpl_diag_new() {
        let m = MplDiagnostic::new(ObisCode::CLOCK);
        assert_eq!(m.class_id(), 98);
    }
    #[test]
    fn test_mpl_diag_domain() {
        let mut m = MplDiagnostic::new(ObisCode::CLOCK);
        m.set_domain_id(5);
        assert_eq!(m.domain_id(), 5);
    }
}
