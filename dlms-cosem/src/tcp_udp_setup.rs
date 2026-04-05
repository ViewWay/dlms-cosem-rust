//! IC43 TcpUdpSetup

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct TcpUdpSetup {
    logical_name: ObisCode,
    version: u8,
}

impl TcpUdpSetup {
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

impl CosemObject for TcpUdpSetup {
    fn class_id(&self) -> u16 {
        43
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        6
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
    fn test_tcp_udp_setup_class_id() {
        let obj = TcpUdpSetup::new(ObisCode::new(0, 0, 96, 0, 0, 255));
        assert_eq!(obj.class_id(), 43);
    }

    #[test]
    fn test_tcp_udp_setup_attribute_count() {
        let obj = TcpUdpSetup::new(ObisCode::new(0, 0, 96, 0, 0, 255));
        assert_eq!(obj.attribute_count(), 6);
    }

    #[test]
    fn test_tcp_udp_setup_method_count() {
        let obj = TcpUdpSetup::new(ObisCode::new(0, 0, 96, 0, 0, 255));
        assert_eq!(obj.method_count(), 0);
    }

    #[test]
    fn test_tcp_udp_setup_version() {
        let obj = TcpUdpSetup::new(ObisCode::new(0, 0, 96, 0, 0, 255));
        assert_eq!(obj.version(), 0);
    }

    #[test]
    fn test_tcp_udp_setup_attr1() {
        let obj = TcpUdpSetup::new(ObisCode::new(0, 0, 96, 0, 0, 255));
        let bytes = obj.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_tcp_udp_setup_unsupported_attr() {
        let obj = TcpUdpSetup::new(ObisCode::new(0, 0, 96, 0, 0, 255));
        assert!(obj.attribute_to_bytes(99).is_none());
    }
}
