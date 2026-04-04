//! IC124 Communication Port Protection
//! Blue Book Ed16: class_id=124, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Communication Port Protection - port protection parameters
pub struct CommPortProtection {
    logical_name: ObisCode,
    port_protection_parameters: Vec<u8>,
}

impl CommPortProtection {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            port_protection_parameters: vec![],
        }
    }
    pub fn parameters(&self) -> &[u8] {
        &self.port_protection_parameters
    }
    pub fn set_parameters(&mut self, params: Vec<u8>) {
        self.port_protection_parameters = params;
    }
}

impl CosemObject for CommPortProtection {
    fn class_id(&self) -> u16 {
        124
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        2
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
                self.port_protection_parameters.clone(),
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(b) = d.as_octet_string() {
                    self.port_protection_parameters = b.to_vec();
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
    fn test_cpp_new() {
        let c = CommPortProtection::new(ObisCode::CLOCK);
        assert_eq!(c.class_id(), 124);
    }
    #[test]
    fn test_cpp_params() {
        let mut c = CommPortProtection::new(ObisCode::CLOCK);
        c.set_parameters(vec![1, 2, 3]);
        assert_eq!(c.parameters().len(), 3);
    }
    #[test]
    fn test_cpp_roundtrip() {
        let mut c = CommPortProtection::new(ObisCode::CLOCK);
        let b = dlms_axdr::encode(&DlmsData::OctetString(vec![0xAA]));
        c.attribute_from_bytes(2, &b).unwrap();
        assert_eq!(c.parameters(), &[0xAA]);
    }
}
