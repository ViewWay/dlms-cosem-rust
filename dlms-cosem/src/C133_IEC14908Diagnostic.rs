//! IC133 Iec14908Diagnostic
//! Blue Book Ed16: class_id=133, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Iec14908Diagnostic
pub struct Iec14908Diagnostic {
    logical_name: ObisCode,
    messages_sent: u32,
    messages_received: u32,
    crc_errors: u32,
}

impl Iec14908Diagnostic {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            messages_sent: 0,
            messages_received: 0,
            crc_errors: 0,
        }
    }
    pub fn messages_sent(&self) -> u32 { self.messages_sent }
    pub fn set_messages_sent(&mut self, v: u32) { self.messages_sent = v; }
    pub fn messages_received(&self) -> u32 { self.messages_received }
    pub fn set_messages_received(&mut self, v: u32) { self.messages_received = v; }
    pub fn crc_errors(&self) -> u32 { self.crc_errors }
    pub fn set_crc_errors(&mut self, v: u32) { self.crc_errors = v; }
}

impl CosemObject for Iec14908Diagnostic {
    fn class_id(&self) -> u16 { 133 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.messages_sent))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.messages_received))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.crc_errors))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.messages_sent=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.messages_received=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.crc_errors=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = Iec14908Diagnostic::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 133);
    }
    #[test]
    fn test_getter() {
        let obj = Iec14908Diagnostic::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.crc_errors();
    }
}
