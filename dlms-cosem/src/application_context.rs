//! IC88 ApplicationContext

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct ApplicationContext {
    logical_name: ObisCode,
    version: u8,
}

impl ApplicationContext {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            version: 0,
        }
    }

    pub fn version(&self) -> u8 {
        self.version
    }
}

impl CosemObject for ApplicationContext {
    fn class_id(&self) -> u16 {
        88
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
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        Err(CosemObjectError::AttributeNotSupported(attr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_context_class_id() {
        let obj = ApplicationContext::new(ObisCode::new(0, 0, 0, 0, 0, 255));
        assert_eq!(obj.class_id(), 88);
    }
}
