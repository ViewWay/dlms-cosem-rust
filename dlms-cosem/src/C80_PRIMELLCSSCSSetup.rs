//! IC80 PrimeLlcSscsSetup
//! Blue Book Ed16: class_id=80, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// PrimeLlcSscsSetup
pub struct PrimeLlcSscsSetup {
    logical_name: ObisCode,
    sscs_type: u8,
    sscs_enable: bool,
}

impl PrimeLlcSscsSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            sscs_type: 0,
            sscs_enable: false,
        }
    }
    pub fn sscs_type(&self) -> u8 { self.sscs_type }
    pub fn set_sscs_type(&mut self, v: u8) { self.sscs_type = v; }
    pub fn sscs_enable(&self) -> bool { self.sscs_enable }
    pub fn set_sscs_enable(&mut self, v: bool) { self.sscs_enable = v; }
}

impl CosemObject for PrimeLlcSscsSetup {
    fn class_id(&self) -> u16 { 80 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.sscs_type))),
            3 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.sscs_enable))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.sscs_type=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Boolean(v) => { self.sscs_enable=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = PrimeLlcSscsSetup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 80);
    }
    #[test]
    fn test_getter() {
        let obj = PrimeLlcSscsSetup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.sscs_enable();
    }
}
