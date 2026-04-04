//! IC97 RPL Diagnostic
//! Blue Book Ed16: class_id=97, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// RPL Diagnostic - Routing Protocol for Low-Power and Lossy Networks diagnostics
pub struct RplDiagnostic {
    logical_name: ObisCode,
    parent_address: Vec<u8>,
    parent_rank: u16,
    children_addresses: Vec<Vec<u8>>,
}

impl RplDiagnostic {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            parent_address: vec![],
            parent_rank: 0,
            children_addresses: vec![],
        }
    }
    pub fn parent_rank(&self) -> u16 {
        self.parent_rank
    }
    pub fn set_parent_rank(&mut self, rank: u16) {
        self.parent_rank = rank;
    }
}

impl CosemObject for RplDiagnostic {
    fn class_id(&self) -> u16 {
        97
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        12
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
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.parent_address.clone(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.parent_rank))),
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
    fn test_rpl_diag_new() {
        let r = RplDiagnostic::new(ObisCode::CLOCK);
        assert_eq!(r.class_id(), 97);
    }
    #[test]
    fn test_rpl_diag_rank() {
        let mut r = RplDiagnostic::new(ObisCode::CLOCK);
        r.set_parent_rank(256);
        assert_eq!(r.parent_rank(), 256);
    }
}
