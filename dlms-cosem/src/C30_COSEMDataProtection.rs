//! IC30 COSEM Data Protection
//! Blue Book Ed16: class_id=30, version=0
//! Protects COSEM attributes and method invocations

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// COSEM Data Protection - attribute/method level protection
pub struct CosemDataProtection {
    logical_name: ObisCode,
    protection_parameters_get: Vec<u8>,
    protection_parameters_set: Vec<u8>,
    protection_buffer: Vec<u8>,
}

impl CosemDataProtection {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            protection_parameters_get: vec![],
            protection_parameters_set: vec![],
            protection_buffer: vec![],
        }
    }

    pub fn protection_parameters_get(&self) -> &[u8] {
        &self.protection_parameters_get
    }

    pub fn set_protection_parameters_get(&mut self, params: Vec<u8>) {
        self.protection_parameters_get = params;
    }

    pub fn protection_parameters_set(&self) -> &[u8] {
        &self.protection_parameters_set
    }

    pub fn set_protection_parameters_set(&mut self, params: Vec<u8>) {
        self.protection_parameters_set = params;
    }

    pub fn protection_buffer(&self) -> &[u8] {
        &self.protection_buffer
    }
}

impl CosemObject for CosemDataProtection {
    fn class_id(&self) -> u16 {
        30
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        5
    }
    fn method_count(&self) -> u8 {
        3
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
                self.protection_parameters_get.clone(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.protection_parameters_set.clone(),
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.protection_buffer.clone(),
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 | 3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(bytes) = decoded.as_octet_string() {
                    if attr == 2 {
                        self.protection_parameters_get = bytes.to_vec();
                    } else {
                        self.protection_parameters_set = bytes.to_vec();
                    }
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
            1 | 2 | 3 => Ok(vec![]),
            _ => Err(CosemObjectError::MethodNotSupported(method_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_protection_new() {
        let dp = CosemDataProtection::new(ObisCode::CLOCK);
        assert_eq!(dp.class_id(), 30);
        assert_eq!(dp.method_count(), 3);
    }

    #[test]
    fn test_data_protection_params() {
        let mut dp = CosemDataProtection::new(ObisCode::CLOCK);
        dp.set_protection_parameters_get(vec![1, 2, 3]);
        assert_eq!(dp.protection_parameters_get(), &[1, 2, 3]);
    }

    #[test]
    fn test_data_protection_roundtrip() {
        let mut dp = CosemDataProtection::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::OctetString(vec![0xAA, 0xBB]));
        dp.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(dp.protection_parameters_get(), &[0xAA, 0xBB]);
    }

    #[test]
    fn test_data_protection_methods() {
        let mut dp = CosemDataProtection::new(ObisCode::CLOCK);
        assert!(dp.execute_action(1, &[]).is_ok());
        assert!(dp.execute_action(4, &[]).is_err());
    }
}
