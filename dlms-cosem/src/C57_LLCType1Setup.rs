//! IC57 LlcType1Setup
//! Blue Book Ed16: class_id=57, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// LlcType1Setup
pub struct LlcType1Setup {
    logical_name: ObisCode,
    llc_type_1_enable: bool,
}

impl LlcType1Setup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            llc_type_1_enable: false,
        }
    }
    pub fn llc_type_1_enable(&self) -> bool { self.llc_type_1_enable }
    pub fn set_llc_type_1_enable(&mut self, v: bool) { self.llc_type_1_enable = v; }
}

impl CosemObject for LlcType1Setup {
    fn class_id(&self) -> u16 { 57 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 2 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.llc_type_1_enable))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Boolean(v) => { self.llc_type_1_enable=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = LlcType1Setup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 57);
    }
    #[test]
    fn test_getter() {
        let obj = LlcType1Setup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.llc_type_1_enable();
    }
}
