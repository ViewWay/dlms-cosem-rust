//! IC105 ZigbeeTunnelSetup
//! Blue Book Ed16: class_id=105, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// ZigbeeTunnelSetup
pub struct ZigbeeTunnelSetup {
    logical_name: ObisCode,
    tunnel_address: String,
    tunnel_port: u16,
}

impl ZigbeeTunnelSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            tunnel_address: String::new(),
            tunnel_port: 0,
        }
    }
    pub fn tunnel_address(&self) -> &str { &self.tunnel_address }
    pub fn set_tunnel_address(&mut self, v: impl Into<String>) { self.tunnel_address = v.into(); }
    pub fn tunnel_port(&self) -> u16 { self.tunnel_port }
    pub fn set_tunnel_port(&mut self, v: u16) { self.tunnel_port = v; }
}

impl CosemObject for ZigbeeTunnelSetup {
    fn class_id(&self) -> u16 { 105 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.tunnel_address.clone()))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.tunnel_port))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::VisibleString(s)|DlmsData::Utf8String(s) => { self.tunnel_address=s.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::LongUnsigned(v) => { self.tunnel_port=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = ZigbeeTunnelSetup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 105);
    }
    #[test]
    fn test_getter() {
        let obj = ZigbeeTunnelSetup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.tunnel_port();
    }
}
