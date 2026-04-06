//! IC59 LlcType3Setup
//! Blue Book Ed16: class_id=59, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// LlcType3Setup
pub struct LlcType3Setup {
    logical_name: ObisCode,
    llc_type_3_enable: bool,
    window_size: u16,
}

impl LlcType3Setup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            llc_type_3_enable: false,
            window_size: 0,
        }
    }
    pub fn llc_type_3_enable(&self) -> bool { self.llc_type_3_enable }
    pub fn set_llc_type_3_enable(&mut self, v: bool) { self.llc_type_3_enable = v; }
    pub fn window_size(&self) -> u16 { self.window_size }
    pub fn set_window_size(&mut self, v: u16) { self.window_size = v; }
}

impl CosemObject for LlcType3Setup {
    fn class_id(&self) -> u16 { 59 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.llc_type_3_enable))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.window_size))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Boolean(v) => { self.llc_type_3_enable=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::LongUnsigned(v) => { self.window_size=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = LlcType3Setup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 59);
    }
    #[test]
    fn test_getter() {
        let obj = LlcType3Setup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.window_size();
    }
}
