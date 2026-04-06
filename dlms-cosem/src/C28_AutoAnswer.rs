//! IC28 AutoAnswer
//! Blue Book Ed16: class_id=28, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// AutoAnswer
pub struct AutoAnswer {
    logical_name: ObisCode,
    mode: u8,
    listening_window: String,
    number_of_calls: u16,
    number_of_rings: u8,
}

impl AutoAnswer {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            mode: 0,
            listening_window: String::new(),
            number_of_calls: 0,
            number_of_rings: 0,
        }
    }
    pub fn mode(&self) -> u8 { self.mode }
    pub fn set_mode(&mut self, v: u8) { self.mode = v; }
    pub fn listening_window(&self) -> &str { &self.listening_window }
    pub fn set_listening_window(&mut self, v: impl Into<String>) { self.listening_window = v.into(); }
    pub fn number_of_calls(&self) -> u16 { self.number_of_calls }
    pub fn set_number_of_calls(&mut self, v: u16) { self.number_of_calls = v; }
    pub fn number_of_rings(&self) -> u8 { self.number_of_rings }
    pub fn set_number_of_rings(&mut self, v: u8) { self.number_of_rings = v; }
}

impl CosemObject for AutoAnswer {
    fn class_id(&self) -> u16 { 28 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 5 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.mode))),
            3 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.listening_window.clone()))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.number_of_calls))),
            5 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.number_of_rings))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.mode=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::VisibleString(s)|DlmsData::Utf8String(s) => { self.listening_window=s.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::LongUnsigned(v) => { self.number_of_calls=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            5 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.number_of_rings=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = AutoAnswer::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 28);
    }
    #[test]
    fn test_getter() {
        let obj = AutoAnswer::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.number_of_rings();
    }
}
