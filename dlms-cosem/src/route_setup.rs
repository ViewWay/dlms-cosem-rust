//! IC087 Route Setup

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Route Setup - routing table configuration
pub struct RouteSetup {
    logical_name: ObisCode,
    routing_enabled: bool,
    max_routes: u8,
}

impl RouteSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            routing_enabled: false,
            max_routes: 16,
        }
    }

    pub fn routing_enabled(&self) -> bool {
        self.routing_enabled
    }

    pub fn set_routing_enabled(&mut self, enabled: bool) {
        self.routing_enabled = enabled;
    }

    pub fn max_routes(&self) -> u8 {
        self.max_routes
    }

    pub fn set_max_routes(&mut self, max: u8) {
        self.max_routes = max;
    }
}

impl CosemObject for RouteSetup {
    fn class_id(&self) -> u16 {
        213
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        4
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
            2 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.routing_enabled))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.max_routes))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(b) = decoded.as_bool() {
                    self.routing_enabled = b;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_u8() {
                    self.max_routes = v;
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
    fn test_route_setup_new() {
        let rs = RouteSetup::new(ObisCode::CLOCK);
        assert_eq!(rs.class_id(), 213);
        assert!(!rs.routing_enabled());
    }

    #[test]
    fn test_route_setup_enable() {
        let mut rs = RouteSetup::new(ObisCode::CLOCK);
        rs.set_routing_enabled(true);
        assert!(rs.routing_enabled());
    }

    #[test]
    fn test_route_setup_roundtrip() {
        let mut rs = RouteSetup::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Boolean(true));
        rs.attribute_from_bytes(2, &bytes).unwrap();
        assert!(rs.routing_enabled());
    }
}
