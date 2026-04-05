//! IC40 Identity - Device Identification

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IC40 Identity - Device Identification Object
/// 
/// This class provides identification information about the device.
/// Used for device management and inventory purposes.
#[derive(Debug, Clone)]
pub struct Identity {
    logical_name: ObisCode,
    manufacturer: String,
    model: String,
    serial_number: String,
    firmware_version: String,
    hardware_version: String,
    device_type: String,
}

impl Identity {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            manufacturer: String::new(),
            model: String::new(),
            serial_number: String::new(),
            firmware_version: String::new(),
            hardware_version: String::new(),
            device_type: String::new(),
        }
    }

    pub fn manufacturer(&self) -> &str {
        &self.manufacturer
    }

    pub fn set_manufacturer(&mut self, manufacturer: String) {
        self.manufacturer = manufacturer;
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }

    pub fn serial_number(&self) -> &str {
        &self.serial_number
    }

    pub fn set_serial_number(&mut self, serial_number: String) {
        self.serial_number = serial_number;
    }

    pub fn firmware_version(&self) -> &str {
        &self.firmware_version
    }

    pub fn set_firmware_version(&mut self, version: String) {
        self.firmware_version = version;
    }

    pub fn hardware_version(&self) -> &str {
        &self.hardware_version
    }

    pub fn set_hardware_version(&mut self, version: String) {
        self.hardware_version = version;
    }

    pub fn device_type(&self) -> &str {
        &self.device_type
    }

    pub fn set_device_type(&mut self, device_type: String) {
        self.device_type = device_type;
    }
}

impl CosemObject for Identity {
    fn class_id(&self) -> u16 {
        40
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
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => {
                // Manufacturer - octet string
                let mut bytes = vec![0x09];
                bytes.push(self.manufacturer.len() as u8);
                bytes.extend_from_slice(self.manufacturer.as_bytes());
                Some(bytes)
            }
            3 => {
                // Model
                let mut bytes = vec![0x09];
                bytes.push(self.model.len() as u8);
                bytes.extend_from_slice(self.model.as_bytes());
                Some(bytes)
            }
            4 => {
                // Serial number
                let mut bytes = vec![0x09];
                bytes.push(self.serial_number.len() as u8);
                bytes.extend_from_slice(self.serial_number.as_bytes());
                Some(bytes)
            }
            5 => {
                // Firmware version
                let mut bytes = vec![0x09];
                bytes.push(self.firmware_version.len() as u8);
                bytes.extend_from_slice(self.firmware_version.as_bytes());
                Some(bytes)
            }
            6 => {
                // Hardware version
                let mut bytes = vec![0x09];
                bytes.push(self.hardware_version.len() as u8);
                bytes.extend_from_slice(self.hardware_version.as_bytes());
                Some(bytes)
            }
            7 => {
                // Device type
                let mut bytes = vec![0x09];
                bytes.push(self.device_type.len() as u8);
                bytes.extend_from_slice(self.device_type.as_bytes());
                Some(bytes)
            }
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 | 3 | 4 | 5 | 6 | 7 => {
                // Parse octet string (simplified)
                if data.len() > 2 {
                    let len = data[1] as usize;
                    if data.len() >= 2 + len {
                        let value = String::from_utf8_lossy(&data[2..2 + len]).to_string();
                        match attr {
                            2 => self.manufacturer = value,
                            3 => self.model = value,
                            4 => self.serial_number = value,
                            5 => self.firmware_version = value,
                            6 => self.hardware_version = value,
                            7 => self.device_type = value,
                            _ => {}
                        }
                    }
                }
                Ok(())
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_new() {
        let identity = Identity::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        assert_eq!(identity.class_id(), 40);
    }

    #[test]
    fn test_identity_manufacturer() {
        let mut identity = Identity::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        identity.set_manufacturer("ABC Corp".to_string());
        assert_eq!(identity.manufacturer(), "ABC Corp");
    }

    #[test]
    fn test_identity_serial_number() {
        let mut identity = Identity::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        identity.set_serial_number("SN12345678".to_string());
        assert_eq!(identity.serial_number(), "SN12345678");
    }

    #[test]
    fn test_identity_firmware_version() {
        let mut identity = Identity::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        identity.set_firmware_version("v1.2.3".to_string());
        assert_eq!(identity.firmware_version(), "v1.2.3");
    }

    #[test]
    fn test_identity_attribute_count() {
        let identity = Identity::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        assert_eq!(identity.attribute_count(), 7);
    }

    #[test]
    fn test_identity_method_count() {
        let identity = Identity::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        assert_eq!(identity.method_count(), 0);
    }

    #[test]
    fn test_identity_attribute_to_bytes() {
        let identity = Identity::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        let bytes = identity.attribute_to_bytes(1).unwrap();
        assert!(!bytes.is_empty());
    }
}
