//! dlms-server: DLMS/COSEM Server
//!
//! Implements a DLMS server (meter simulator):
//! - COSEM object registration
//! - Request routing (GET/SET/ACTION)
//! - HDLC frame handling
//! - Association handling

use dlms_core::{AccessResult, CosemObject, DlmsData, ObisCode};
use std::collections::HashMap;

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub server_address: u8,
    pub logical_device_name: String,
    pub max_clients: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            server_address: 0x01,
            logical_device_name: "DLMS-Server".to_string(),
            max_clients: 1,
        }
    }
}

/// COSEM object registration key: (class_id, logical_name)
type ObjectKey = (u16, [u8; 6]);

/// DLMS Server
pub struct DlmsServer {
    #[allow(dead_code)]
    config: ServerConfig,
    objects: HashMap<ObjectKey, Box<dyn CosemObject>>,
}

impl DlmsServer {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            objects: HashMap::new(),
        }
    }

    /// Register a COSEM object
    pub fn register_object(&mut self, object: Box<dyn CosemObject>) {
        let key = (object.class_id(), object.logical_name().to_bytes());
        self.objects.insert(key, object);
    }

    /// Get a registered object
    pub fn get_object(&self, class_id: u16, logical_name: &ObisCode) -> Option<&dyn CosemObject> {
        let key = (class_id, logical_name.to_bytes());
        self.objects.get(&key).map(|o| o.as_ref())
    }

    /// Get a mutable reference to a registered object
    pub fn get_object_mut(
        &mut self,
        class_id: u16,
        logical_name: &ObisCode,
    ) -> Option<&mut Box<dyn CosemObject>> {
        let key = (class_id, logical_name.to_bytes());
        self.objects.get_mut(&key)
    }

    /// List all registered objects
    pub fn list_objects(&self) -> Vec<(u16, ObisCode)> {
        self.objects
            .values()
            .map(|o| (o.class_id(), o.logical_name()))
            .collect()
    }

    /// Handle a GET request
    pub fn handle_get(
        &self,
        class_id: u16,
        logical_name: &ObisCode,
        attribute_id: u8,
    ) -> Result<DlmsData, ServerError> {
        let object = self
            .get_object(class_id, logical_name)
            .ok_or(ServerError::ObjectNotFound)?;

        if attribute_id as usize > object.attribute_count() as usize {
            return Err(ServerError::AttributeNotSupported);
        }

        let bytes = object
            .attribute_to_bytes(attribute_id)
            .ok_or(ServerError::AttributeNotSupported)?;

        // Parse back to DlmsData
        if attribute_id == 1 {
            // Logical name - construct from bytes
            return Ok(DlmsData::OctetString(logical_name.to_bytes().to_vec()));
        }

        dlms_axdr::decode(&bytes).map_err(|_| ServerError::EncodingError)
    }

    /// Handle a SET request
    pub fn handle_set(
        &mut self,
        class_id: u16,
        logical_name: &ObisCode,
        attribute_id: u8,
        data: &[u8],
    ) -> Result<AccessResult, ServerError> {
        let object = self
            .get_object_mut(class_id, logical_name)
            .ok_or(ServerError::ObjectNotFound)?;

        object
            .attribute_from_bytes(attribute_id, data)
            .map_err(|_| ServerError::WriteFailed)?;

        Ok(AccessResult::Success)
    }

    /// Process raw HDLC frame data
    pub fn process_frame(&mut self, frame_info: &[u8]) -> Result<Vec<u8>, ServerError> {
        if frame_info.len() < 2 {
            return Err(ServerError::InvalidRequest);
        }

        let xdlms_type = frame_info[0];
        match xdlms_type {
            0x05 => {
                // GET-Request
                self.handle_get_request(&frame_info[1..])
            }
            0x06 => {
                // SET-Request
                self.handle_set_request(&frame_info[1..])
            }
            _ => Err(ServerError::UnsupportedService),
        }
    }

    fn handle_get_request(&self, data: &[u8]) -> Result<Vec<u8>, ServerError> {
        if data.len() < 11 {
            return Err(ServerError::InvalidRequest);
        }

        // Parse CosemAttributeDescriptor (simplified)
        let class_id = u16::from_be_bytes([data[3], data[4]]);
        let mut ln = [0u8; 6];
        ln.copy_from_slice(&data[7..13]);
        let attribute_id = data[15];

        let obis = ObisCode::from_bytes(ln);
        let result_data = self.handle_get(class_id, &obis, attribute_id)?;

        // Build response
        let mut response = Vec::new();
        response.push(0x07); // GET-Response
        response.extend_from_slice(&data[0..1]); // invoke_id
        response.push(0x00); // result = success
        let encoded = dlms_axdr::encode(&result_data);
        response.extend_from_slice(&encoded);
        Ok(response)
    }

    fn handle_set_request(&mut self, _data: &[u8]) -> Result<Vec<u8>, ServerError> {
        Err(ServerError::UnsupportedService)
    }

    /// Object count
    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerError {
    ObjectNotFound,
    AttributeNotSupported,
    MethodNotSupported,
    WriteFailed,
    ReadFailed,
    EncodingError,
    InvalidRequest,
    UnsupportedService,
    AssociationFailed,
}

impl core::fmt::Display for ServerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ServerError::ObjectNotFound => write!(f, "Object not found"),
            ServerError::AttributeNotSupported => write!(f, "Attribute not supported"),
            ServerError::MethodNotSupported => write!(f, "Method not supported"),
            ServerError::WriteFailed => write!(f, "Write failed"),
            ServerError::ReadFailed => write!(f, "Read failed"),
            ServerError::EncodingError => write!(f, "Encoding error"),
            ServerError::InvalidRequest => write!(f, "Invalid request"),
            ServerError::UnsupportedService => write!(f, "Unsupported service"),
            ServerError::AssociationFailed => write!(f, "Association failed"),
        }
    }
}

impl std::error::Error for ServerError {}

#[cfg(test)]
mod tests {
    use super::*;
    use dlms_cosem::{Clock, Register};

    #[test]
    fn test_server_new() {
        let server = DlmsServer::new(ServerConfig::default());
        assert_eq!(server.object_count(), 0);
    }

    #[test]
    fn test_server_register_and_list() {
        let mut server = DlmsServer::new(ServerConfig::default());
        server.register_object(Box::new(Clock::new(ObisCode::CLOCK)));
        server.register_object(Box::new(Register::new(
            ObisCode::ACTIVE_POWER_L1,
            DlmsData::DoubleLong(1000),
        )));
        assert_eq!(server.object_count(), 2);
        let objects = server.list_objects();
        assert_eq!(objects.len(), 2);
    }

    #[test]
    fn test_server_get_clock() {
        let mut server = DlmsServer::new(ServerConfig::default());
        server.register_object(Box::new(Clock::new(ObisCode::CLOCK)));
        let result = server.handle_get(8, &ObisCode::CLOCK, 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_server_get_not_found() {
        let server = DlmsServer::new(ServerConfig::default());
        let result = server.handle_get(8, &ObisCode::CLOCK, 2);
        assert!(matches!(result, Err(ServerError::ObjectNotFound)));
    }

    #[test]
    fn test_server_config_default() {
        let cfg = ServerConfig::default();
        assert_eq!(cfg.server_address, 0x01);
        assert_eq!(cfg.logical_device_name, "DLMS-Server");
    }

    #[test]
    fn test_server_error_display() {
        let err = ServerError::ObjectNotFound;
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_server_process_invalid_frame() {
        let mut server = DlmsServer::new(ServerConfig::default());
        let result = server.process_frame(&[0x05]);
        assert!(result.is_err());
    }

    #[test]
    fn test_server_register_get_object() {
        let mut server = DlmsServer::new(ServerConfig::default());
        server.register_object(Box::new(Clock::new(ObisCode::CLOCK)));
        assert!(server.get_object(8, &ObisCode::CLOCK).is_some());
        assert!(server.get_object(3, &ObisCode::ACTIVE_POWER_L1).is_none());
    }

    #[test]
    fn test_server_handle_set() {
        let mut server = DlmsServer::new(ServerConfig::default());
        server.register_object(Box::new(Register::new(
            ObisCode::ACTIVE_POWER_L1,
            DlmsData::DoubleLong(0),
        )));
        let data = dlms_axdr::encode(&DlmsData::DoubleLong(42));
        let result = server.handle_set(3, &ObisCode::ACTIVE_POWER_L1, 2, &data);
        assert!(result.is_ok());
    }
}
