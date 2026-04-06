//! IC72 M-Bus Client - M-Bus Master Device

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IC72 M-Bus Client - M-Bus Master Device Object
/// 
/// This class manages M-Bus communication for meter reading.
/// Used in utility metering systems.
#[derive(Debug, Clone)]
pub struct MbusClient {
    logical_name: ObisCode,
    mbus_port: u8,
    primary_address: u8,
    identification_number: u32,
    manufacturer_id: String,
    version: u8,
    device_type: u8,
    access_number: u8,
    status: u8,
    last_readout: Option<Vec<u8>>,
}

impl MbusClient {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            mbus_port: 0,
            primary_address: 0,
            identification_number: 0,
            manufacturer_id: String::new(),
            version: 0,
            device_type: 0,
            access_number: 0,
            status: 0,
            last_readout: None,
        }
    }

    pub fn mbus_port(&self) -> u8 {
        self.mbus_port
    }

    pub fn set_mbus_port(&mut self, port: u8) {
        self.mbus_port = port;
    }

    pub fn primary_address(&self) -> u8 {
        self.primary_address
    }

    pub fn set_primary_address(&mut self, addr: u8) {
        self.primary_address = addr;
    }

    pub fn identification_number(&self) -> u32 {
        self.identification_number
    }

    pub fn set_identification_number(&mut self, id: u32) {
        self.identification_number = id;
    }

    pub fn manufacturer_id(&self) -> &str {
        &self.manufacturer_id
    }

    pub fn set_manufacturer_id(&mut self, id: String) {
        self.manufacturer_id = id;
    }

    pub fn version(&self) -> u8 {
        self.version
    }

    pub fn set_version(&mut self, version: u8) {
        self.version = version;
    }

    pub fn device_type(&self) -> u8 {
        self.device_type
    }

    pub fn set_device_type(&mut self, device_type: u8) {
        self.device_type = device_type;
    }

    pub fn access_number(&self) -> u8 {
        self.access_number
    }

    pub fn set_access_number(&mut self, num: u8) {
        self.access_number = num;
    }

    pub fn status(&self) -> u8 {
        self.status
    }

    pub fn set_status(&mut self, status: u8) {
        self.status = status;
    }

    pub fn last_readout(&self) -> Option<&[u8]> {
        self.last_readout.as_deref()
    }

    pub fn set_last_readout(&mut self, data: Vec<u8>) {
        self.last_readout = Some(data);
    }
}

impl CosemObject for MbusClient {
    fn class_id(&self) -> u16 {
        72
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        9
    }

    fn method_count(&self) -> u8 {
        4
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
                // M-Bus port
                Some(vec![0x0F, self.mbus_port])
            }
            3 => {
                // Primary address
                Some(vec![0x0F, self.primary_address])
            }
            4 => {
                // Identification number
                let mut bytes = vec![0x06];
                bytes.extend_from_slice(&self.identification_number.to_be_bytes());
                Some(bytes)
            }
            5 => {
                // Manufacturer ID
                let mut bytes = vec![0x09];
                bytes.push(self.manufacturer_id.len() as u8);
                bytes.extend_from_slice(self.manufacturer_id.as_bytes());
                Some(bytes)
            }
            6 => {
                // Version
                Some(vec![0x0F, self.version])
            }
            7 => {
                // Device type
                Some(vec![0x0F, self.device_type])
            }
            8 => {
                // Access number
                Some(vec![0x0F, self.access_number])
            }
            9 => {
                // Status
                Some(vec![0x0F, self.status])
            }
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                if data.len() >= 2 {
                    self.mbus_port = data[1];
                }
                Ok(())
            }
            3 => {
                if data.len() >= 2 {
                    self.primary_address = data[1];
                }
                Ok(())
            }
            4 => {
                if data.len() >= 5 {
                    self.identification_number = u32::from_be_bytes([data[1], data[2], data[3], data[4]]);
                }
                Ok(())
            }
            5 => {
                if data.len() > 2 {
                    let len = data[1] as usize;
                    if data.len() >= 2 + len {
                        self.manufacturer_id = String::from_utf8_lossy(&data[2..2 + len]).to_string();
                    }
                }
                Ok(())
            }
            6 => {
                if data.len() >= 2 {
                    self.version = data[1];
                }
                Ok(())
            }
            7 => {
                if data.len() >= 2 {
                    self.device_type = data[1];
                }
                Ok(())
            }
            8 => {
                if data.len() >= 2 {
                    self.access_number = data[1];
                }
                Ok(())
            }
            9 => {
                if data.len() >= 2 {
                    self.status = data[1];
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
    fn test_mbus_client_new() {
        let client = MbusClient::new(ObisCode::new(0, 0, 72, 0, 0, 255));
        assert_eq!(client.class_id(), 72);
    }

    #[test]
    fn test_mbus_client_addresses() {
        let mut client = MbusClient::new(ObisCode::new(0, 0, 72, 0, 0, 255));
        client.set_mbus_port(1);
        client.set_primary_address(5);
        assert_eq!(client.mbus_port(), 1);
        assert_eq!(client.primary_address(), 5);
    }

    #[test]
    fn test_mbus_client_identification() {
        let mut client = MbusClient::new(ObisCode::new(0, 0, 72, 0, 0, 255));
        client.set_identification_number(12345678);
        client.set_manufacturer_id("ABC".to_string());
        assert_eq!(client.identification_number(), 12345678);
        assert_eq!(client.manufacturer_id(), "ABC");
    }

    #[test]
    fn test_mbus_client_version() {
        let mut client = MbusClient::new(ObisCode::new(0, 0, 72, 0, 0, 255));
        client.set_version(2);
        assert_eq!(client.version(), 2);
    }

    #[test]
    fn test_mbus_client_device_type() {
        let mut client = MbusClient::new(ObisCode::new(0, 0, 72, 0, 0, 255));
        client.set_device_type(0x07); // Water meter
        assert_eq!(client.device_type(), 0x07);
    }

    #[test]
    fn test_mbus_client_status() {
        let mut client = MbusClient::new(ObisCode::new(0, 0, 72, 0, 0, 255));
        client.set_status(0x01);
        assert_eq!(client.status(), 0x01);
    }

    #[test]
    fn test_mbus_client_last_readout() {
        let mut client = MbusClient::new(ObisCode::new(0, 0, 72, 0, 0, 255));
        client.set_last_readout(vec![0x68, 0x02, 0x02, 0x00, 0x16]);
        assert!(client.last_readout().is_some());
        assert_eq!(client.last_readout().unwrap().len(), 5);
    }

    #[test]
    fn test_mbus_client_attribute_count() {
        let client = MbusClient::new(ObisCode::new(0, 0, 72, 0, 0, 255));
        assert_eq!(client.attribute_count(), 9);
    }

    #[test]
    fn test_mbus_client_method_count() {
        let client = MbusClient::new(ObisCode::new(0, 0, 72, 0, 0, 255));
        assert_eq!(client.method_count(), 4);
    }
}
