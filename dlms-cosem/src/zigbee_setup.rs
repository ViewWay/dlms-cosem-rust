//! IC27 ZigBee Setup - ZigBee Communication Configuration

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IC27 ZigBee Setup - ZigBee Communication Configuration Object
/// 
/// This class manages ZigBee network configuration and communication parameters.
/// Used in smart metering and home automation systems.
#[derive(Debug, Clone)]
pub struct ZigBeeSetup {
    logical_name: ObisCode,
    pan_id: u16,
    channel: u8,
    device_type: ZigBeeDeviceType,
    network_key: Vec<u8>,
    link_key: Vec<u8>,
    short_address: u16,
    enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZigBeeDeviceType {
    Coordinator = 0,
    Router = 1,
    EndDevice = 2,
}

impl ZigBeeSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            pan_id: 0,
            channel: 11,
            device_type: ZigBeeDeviceType::EndDevice,
            network_key: vec![0; 16],
            link_key: vec![0; 16],
            short_address: 0,
            enabled: false,
        }
    }

    pub fn pan_id(&self) -> u16 {
        self.pan_id
    }

    pub fn set_pan_id(&mut self, pan_id: u16) {
        self.pan_id = pan_id;
    }

    pub fn channel(&self) -> u8 {
        self.channel
    }

    pub fn set_channel(&mut self, channel: u8) {
        if channel >= 11 && channel <= 26 {
            self.channel = channel;
        }
    }

    pub fn device_type(&self) -> ZigBeeDeviceType {
        self.device_type
    }

    pub fn set_device_type(&mut self, device_type: ZigBeeDeviceType) {
        self.device_type = device_type;
    }

    pub fn network_key(&self) -> &[u8] {
        &self.network_key
    }

    pub fn set_network_key(&mut self, key: Vec<u8>) {
        if key.len() == 16 {
            self.network_key = key;
        }
    }

    pub fn link_key(&self) -> &[u8] {
        &self.link_key
    }

    pub fn set_link_key(&mut self, key: Vec<u8>) {
        if key.len() == 16 {
            self.link_key = key;
        }
    }

    pub fn short_address(&self) -> u16 {
        self.short_address
    }

    pub fn set_short_address(&mut self, addr: u16) {
        self.short_address = addr;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl CosemObject for ZigBeeSetup {
    fn class_id(&self) -> u16 {
        27
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        8
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
                // PAN ID
                let mut bytes = vec![0x12]; // Unsigned16
                bytes.extend_from_slice(&self.pan_id.to_be_bytes());
                Some(bytes)
            }
            3 => {
                // Channel
                Some(vec![0x0F, self.channel])
            }
            4 => {
                // Device type
                Some(vec![0x0F, self.device_type as u8])
            }
            5 => {
                // Network key - octet string
                let mut bytes = vec![0x09, 0x10];
                bytes.extend_from_slice(&self.network_key);
                Some(bytes)
            }
            6 => {
                // Link key
                let mut bytes = vec![0x09, 0x10];
                bytes.extend_from_slice(&self.link_key);
                Some(bytes)
            }
            7 => {
                // Short address
                let mut bytes = vec![0x12];
                bytes.extend_from_slice(&self.short_address.to_be_bytes());
                Some(bytes)
            }
            8 => {
                // Enabled
                Some(vec![0x0F, if self.enabled { 1 } else { 0 }])
            }
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                if data.len() >= 3 {
                    self.pan_id = u16::from_be_bytes([data[1], data[2]]);
                }
                Ok(())
            }
            3 => {
                if data.len() >= 2 {
                    self.channel = data[1];
                }
                Ok(())
            }
            4 => {
                if data.len() >= 2 {
                    self.device_type = match data[1] {
                        0 => ZigBeeDeviceType::Coordinator,
                        1 => ZigBeeDeviceType::Router,
                        2 => ZigBeeDeviceType::EndDevice,
                        _ => ZigBeeDeviceType::EndDevice,
                    };
                }
                Ok(())
            }
            5 => {
                if data.len() >= 18 {
                    self.network_key = data[2..18].to_vec();
                }
                Ok(())
            }
            6 => {
                if data.len() >= 18 {
                    self.link_key = data[2..18].to_vec();
                }
                Ok(())
            }
            7 => {
                if data.len() >= 3 {
                    self.short_address = u16::from_be_bytes([data[1], data[2]]);
                }
                Ok(())
            }
            8 => {
                if data.len() >= 2 {
                    self.enabled = data[1] != 0;
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
    fn test_zigbee_setup_new() {
        let setup = ZigBeeSetup::new(ObisCode::new(0, 0, 27, 0, 0, 255));
        assert_eq!(setup.class_id(), 27);
    }

    #[test]
    fn test_zigbee_setup_pan_id() {
        let mut setup = ZigBeeSetup::new(ObisCode::new(0, 0, 27, 0, 0, 255));
        setup.set_pan_id(0x1234);
        assert_eq!(setup.pan_id(), 0x1234);
    }

    #[test]
    fn test_zigbee_setup_channel() {
        let mut setup = ZigBeeSetup::new(ObisCode::new(0, 0, 27, 0, 0, 255));
        setup.set_channel(15);
        assert_eq!(setup.channel(), 15);
    }

    #[test]
    fn test_zigbee_setup_device_type() {
        let mut setup = ZigBeeSetup::new(ObisCode::new(0, 0, 27, 0, 0, 255));
        setup.set_device_type(ZigBeeDeviceType::Coordinator);
        assert_eq!(setup.device_type(), ZigBeeDeviceType::Coordinator);
    }

    #[test]
    fn test_zigbee_setup_network_key() {
        let mut setup = ZigBeeSetup::new(ObisCode::new(0, 0, 27, 0, 0, 255));
        let key = vec![0x01; 16];
        setup.set_network_key(key.clone());
        assert_eq!(setup.network_key(), key.as_slice());
    }

    #[test]
    fn test_zigbee_setup_enabled() {
        let mut setup = ZigBeeSetup::new(ObisCode::new(0, 0, 27, 0, 0, 255));
        setup.set_enabled(true);
        assert!(setup.is_enabled());
    }

    #[test]
    fn test_zigbee_setup_attribute_count() {
        let setup = ZigBeeSetup::new(ObisCode::new(0, 0, 27, 0, 0, 255));
        assert_eq!(setup.attribute_count(), 8);
    }

    #[test]
    fn test_zigbee_setup_method_count() {
        let setup = ZigBeeSetup::new(ObisCode::new(0, 0, 27, 0, 0, 255));
        assert_eq!(setup.method_count(), 3);
    }
}
