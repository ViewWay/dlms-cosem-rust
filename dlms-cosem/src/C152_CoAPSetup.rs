//! IC152 CoapSetup
//! Blue Book Ed16: class_id=152, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// CoapSetup
pub struct CoapSetup {
    logical_name: ObisCode,
    coap_server_address: String,
    coap_server_port: u16,
    response_timeout: u32,
}

impl CoapSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            coap_server_address: String::new(),
            coap_server_port: 0,
            response_timeout: 0,
        }
    }
    pub fn coap_server_address(&self) -> &str { &self.coap_server_address }
    pub fn set_coap_server_address(&mut self, v: impl Into<String>) { self.coap_server_address = v.into(); }
    pub fn coap_server_port(&self) -> u16 { self.coap_server_port }
    pub fn set_coap_server_port(&mut self, v: u16) { self.coap_server_port = v; }
    pub fn response_timeout(&self) -> u32 { self.response_timeout }
    pub fn set_response_timeout(&mut self, v: u32) { self.response_timeout = v; }
}

impl CosemObject for CoapSetup {
    fn class_id(&self) -> u16 { 152 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.coap_server_address.clone()))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.coap_server_port))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.response_timeout))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::VisibleString(s)|DlmsData::Utf8String(s) => { self.coap_server_address=s.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::LongUnsigned(v) => { self.coap_server_port=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.response_timeout=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = CoapSetup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 152);
    }
    #[test]
    fn test_getter() {
        let obj = CoapSetup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.response_timeout();
    }
}
