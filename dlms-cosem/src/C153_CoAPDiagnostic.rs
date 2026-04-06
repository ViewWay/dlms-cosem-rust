//! IC153 CoapDiagnostic
//! Blue Book Ed16: class_id=153, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// CoapDiagnostic
pub struct CoapDiagnostic {
    logical_name: ObisCode,
    messages_sent: u32,
    messages_received: u32,
    messages_failed: u32,
}

impl CoapDiagnostic {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            messages_sent: 0,
            messages_received: 0,
            messages_failed: 0,
        }
    }
    pub fn messages_sent(&self) -> u32 { self.messages_sent }
    pub fn set_messages_sent(&mut self, v: u32) { self.messages_sent = v; }
    pub fn messages_received(&self) -> u32 { self.messages_received }
    pub fn set_messages_received(&mut self, v: u32) { self.messages_received = v; }
    pub fn messages_failed(&self) -> u32 { self.messages_failed }
    pub fn set_messages_failed(&mut self, v: u32) { self.messages_failed = v; }
}

impl CosemObject for CoapDiagnostic {
    fn class_id(&self) -> u16 { 153 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.messages_sent))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.messages_received))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.messages_failed))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.messages_sent=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.messages_received=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.messages_failed=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = CoapDiagnostic::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 153);
    }
    #[test]
    fn test_getter() {
        let obj = CoapDiagnostic::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.messages_failed();
    }
}
