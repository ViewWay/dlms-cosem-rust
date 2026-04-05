//! Firmware Management - Manages firmware updates and versions
//!
//! Handles firmware versioning, update scheduling, and verification
//! for metering devices.

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Firmware component information
#[derive(Debug, Clone)]
pub struct FirmwareComponent {
    pub component_id: u16,
    pub version: String,
    pub status: u8,
}

/// IC205 Firmware Management - Manages firmware operations
pub struct FirmwareManagement {
    logical_name: ObisCode,
    components: Vec<FirmwareComponent>,
    update_pending: bool,
    current_operation: u8,
}

impl FirmwareManagement {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            components: Vec::new(),
            update_pending: false,
            current_operation: 0,
        }
    }

    pub fn add_component(&mut self, component_id: u16, version: String) {
        self.components.push(FirmwareComponent {
            component_id,
            version,
            status: 0, // Normal
        });
    }

    pub fn get_component(&self, component_id: u16) -> Option<&FirmwareComponent> {
        self.components.iter().find(|c| c.component_id == component_id)
    }

    pub fn update_version(&mut self, component_id: u16, new_version: String) -> bool {
        if let Some(comp) = self.components.iter_mut().find(|c| c.component_id == component_id) {
            comp.version = new_version;
            true
        } else {
            false
        }
    }

    pub fn set_update_pending(&mut self, pending: bool) {
        self.update_pending = pending;
    }

    pub fn is_update_pending(&self) -> bool {
        self.update_pending
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }

    pub fn set_current_operation(&mut self, operation: u8) {
        self.current_operation = operation;
    }
}

impl CosemObject for FirmwareManagement {
    fn class_id(&self) -> u16 {
        205
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        6
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
            2 => {
                let components: Vec<DlmsData> = self
                    .components
                    .iter()
                    .map(|c| {
                        DlmsData::Structure(vec![
                            DlmsData::LongUnsigned(c.component_id),
                            DlmsData::OctetString(c.version.as_bytes().to_vec()),
                            DlmsData::Unsigned(c.status),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(components)))
            }
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.components.len() as u16,
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.update_pending))),
            5 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.current_operation))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Boolean(v) = decoded {
                    self.update_pending = v;
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
    fn test_firmware_management_class_id() {
        let fm = FirmwareManagement::new(ObisCode::new(0, 0, 205, 0, 0, 255));
        assert_eq!(fm.class_id(), 205);
    }

    #[test]
    fn test_firmware_management_new() {
        let fm = FirmwareManagement::new(ObisCode::new(0, 0, 205, 0, 0, 255));
        assert_eq!(fm.component_count(), 0);
        assert!(!fm.is_update_pending());
    }

    #[test]
    fn test_firmware_management_add_component() {
        let mut fm = FirmwareManagement::new(ObisCode::new(0, 0, 205, 0, 0, 255));
        fm.add_component(1, "1.0.0".to_string());
        assert_eq!(fm.component_count(), 1);
    }

    #[test]
    fn test_firmware_management_get_component() {
        let mut fm = FirmwareManagement::new(ObisCode::new(0, 0, 205, 0, 0, 255));
        fm.add_component(1, "1.0.0".to_string());
        let comp = fm.get_component(1);
        assert!(comp.is_some());
        assert_eq!(comp.unwrap().version, "1.0.0");
    }

    #[test]
    fn test_firmware_management_update_version() {
        let mut fm = FirmwareManagement::new(ObisCode::new(0, 0, 205, 0, 0, 255));
        fm.add_component(1, "1.0.0".to_string());
        let result = fm.update_version(1, "2.0.0".to_string());
        assert!(result);
        assert_eq!(fm.get_component(1).unwrap().version, "2.0.0");
    }

    #[test]
    fn test_firmware_management_update_pending() {
        let mut fm = FirmwareManagement::new(ObisCode::new(0, 0, 205, 0, 0, 255));
        fm.set_update_pending(true);
        assert!(fm.is_update_pending());
        fm.set_update_pending(false);
        assert!(!fm.is_update_pending());
    }

    #[test]
    fn test_firmware_management_current_operation() {
        let mut fm = FirmwareManagement::new(ObisCode::new(0, 0, 205, 0, 0, 255));
        fm.set_current_operation(1);
        assert_eq!(fm.current_operation, 1);
    }

    #[test]
    fn test_firmware_management_multiple_components() {
        let mut fm = FirmwareManagement::new(ObisCode::new(0, 0, 205, 0, 0, 255));
        fm.add_component(1, "1.0.0".to_string());
        fm.add_component(2, "1.0.0".to_string());
        fm.add_component(3, "1.0.0".to_string());
        assert_eq!(fm.component_count(), 3);
    }

    #[test]
    fn test_firmware_management_attribute_count() {
        let fm = FirmwareManagement::new(ObisCode::new(0, 0, 205, 0, 0, 255));
        assert_eq!(fm.attribute_count(), 6);
    }

    #[test]
    fn test_firmware_management_method_count() {
        let fm = FirmwareManagement::new(ObisCode::new(0, 0, 205, 0, 0, 255));
        assert_eq!(fm.method_count(), 3);
    }
}
