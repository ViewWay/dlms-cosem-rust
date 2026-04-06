//! IC55 Iec61334LlcSetup
//! Blue Book Ed16: class_id=55, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Iec61334LlcSetup
pub struct Iec61334LlcSetup {
    logical_name: ObisCode,
    llc_type_1_enable: bool,
    llc_type_2_enable: bool,
}

impl Iec61334LlcSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            llc_type_1_enable: false,
            llc_type_2_enable: false,
        }
    }
    pub fn llc_type_1_enable(&self) -> bool { self.llc_type_1_enable }
    pub fn set_llc_type_1_enable(&mut self, v: bool) { self.llc_type_1_enable = v; }
    pub fn llc_type_2_enable(&self) -> bool { self.llc_type_2_enable }
    pub fn set_llc_type_2_enable(&mut self, v: bool) { self.llc_type_2_enable = v; }
}

impl CosemObject for Iec61334LlcSetup {
    fn class_id(&self) -> u16 { 55 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.llc_type_1_enable))),
            3 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.llc_type_2_enable))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Boolean(v) => { self.llc_type_1_enable=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Boolean(v) => { self.llc_type_2_enable=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = Iec61334LlcSetup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 55);
    }
    #[test]
    fn test_getter() {
        let obj = Iec61334LlcSetup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.llc_type_2_enable();
    }
}
