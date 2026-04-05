//! IC022 Module
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: module_status (unsigned)
//! 3: module_info (string)
//!
//! Methods:
//! 1: activate
//! 2: deactivate

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModuleStatus {
    Deactivated = 0,
    Activated = 1,
    Error = 2,
}

impl ModuleStatus {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => ModuleStatus::Deactivated,
            1 => ModuleStatus::Activated,
            2 => ModuleStatus::Error,
            _ => ModuleStatus::Deactivated,
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

pub struct Module {
    logical_name: ObisCode,
    status: ModuleStatus,
    module_info: String,
}

impl Module {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            status: ModuleStatus::Deactivated,
            module_info: String::new(),
        }
    }

    pub fn with_info(logical_name: ObisCode, module_info: String) -> Self {
        Self {
            logical_name,
            status: ModuleStatus::Deactivated,
            module_info,
        }
    }

    pub fn status(&self) -> ModuleStatus {
        self.status
    }

    pub fn set_status(&mut self, status: ModuleStatus) {
        self.status = status;
    }

    pub fn module_info(&self) -> &str {
        &self.module_info
    }

    pub fn set_module_info(&mut self, info: String) {
        self.module_info = info;
    }

    pub fn is_activated(&self) -> bool {
        self.status == ModuleStatus::Activated
    }

    pub fn is_deactivated(&self) -> bool {
        self.status == ModuleStatus::Deactivated
    }
}

impl CosemObject for Module {
    fn class_id(&self) -> u16 {
        22
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        3
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
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.status.as_u8()))),
            3 => Some(dlms_axdr::encode(&DlmsData::VisibleString(
                self.module_info.clone(),
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(value) = decoded {
                    self.status = ModuleStatus::from_u8(value);
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::VisibleString(s) = decoded {
                    self.module_info = s;
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
                // activate
                self.status = ModuleStatus::Activated;
                Ok(vec![0x00, 0x00]) // success
            }
            2 => {
                // deactivate
                self.status = ModuleStatus::Deactivated;
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
    fn test_module_class_id() {
        let m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        assert_eq!(m.class_id(), 22);
    }

    #[test]
    fn test_module_attribute_count() {
        let m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        assert_eq!(m.attribute_count(), 3);
    }

    #[test]
    fn test_module_method_count() {
        let m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        assert_eq!(m.method_count(), 2);
    }

    #[test]
    fn test_module_status_default() {
        let m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        assert_eq!(m.status(), ModuleStatus::Deactivated);
    }

    #[test]
    fn test_module_set_status() {
        let mut m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        m.set_status(ModuleStatus::Activated);
        assert_eq!(m.status(), ModuleStatus::Activated);
    }

    #[test]
    fn test_module_is_activated() {
        let mut m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        m.set_status(ModuleStatus::Activated);
        assert!(m.is_activated());
    }

    #[test]
    fn test_module_is_deactivated() {
        let m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        assert!(m.is_deactivated());
    }

    #[test]
    fn test_module_info() {
        let info = "Test Module".to_string();
        let m = Module::with_info(ObisCode::new(0, 0, 22, 0, 0, 255), info.clone());
        assert_eq!(m.module_info(), "Test Module");
    }

    #[test]
    fn test_module_attr1() {
        let m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        let bytes = m.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_module_attr2() {
        let m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        let bytes = m.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_module_attr3() {
        let m = Module::with_info(
            ObisCode::new(0, 0, 22, 0, 0, 255),
            "Test Module".to_string(),
        );
        let bytes = m.attribute_to_bytes(3).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_module_activate_method() {
        let mut m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        assert!(!m.is_activated());
        let result = m.execute_action(1, &[]);
        assert!(result.is_ok());
        assert!(m.is_activated());
    }

    #[test]
    fn test_module_deactivate_method() {
        let mut m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        m.set_status(ModuleStatus::Activated);
        assert!(m.is_activated());
        let result = m.execute_action(2, &[]);
        assert!(result.is_ok());
        assert!(m.is_deactivated());
    }

    #[test]
    fn test_module_status_from_u8() {
        assert_eq!(ModuleStatus::from_u8(0), ModuleStatus::Deactivated);
        assert_eq!(ModuleStatus::from_u8(1), ModuleStatus::Activated);
        assert_eq!(ModuleStatus::from_u8(2), ModuleStatus::Error);
        assert_eq!(ModuleStatus::from_u8(99), ModuleStatus::Deactivated);
    }

    #[test]
    fn test_module_unsupported_method() {
        let mut m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        let result = m.execute_action(99, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_module_attr_from_bytes_status() {
        let mut m = Module::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        let encoded = dlms_axdr::encode(&DlmsData::Unsigned(1));
        let result = m.attribute_from_bytes(2, &encoded);
        assert!(result.is_ok());
        assert_eq!(m.status(), ModuleStatus::Activated);
    }
}
