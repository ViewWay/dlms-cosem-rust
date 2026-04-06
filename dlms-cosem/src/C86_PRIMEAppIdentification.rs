//! IC86 PrimeAppIdentification
//! Blue Book Ed16: class_id=86, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// PrimeAppIdentification
pub struct PrimeAppIdentification {
    logical_name: ObisCode,
    application_name: String,
    application_version: String,
}

impl PrimeAppIdentification {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            application_name: String::new(),
            application_version: String::new(),
        }
    }
    pub fn application_name(&self) -> &str { &self.application_name }
    pub fn set_application_name(&mut self, v: impl Into<String>) { self.application_name = v.into(); }
    pub fn application_version(&self) -> &str { &self.application_version }
    pub fn set_application_version(&mut self, v: impl Into<String>) { self.application_version = v.into(); }
}

impl CosemObject for PrimeAppIdentification {
    fn class_id(&self) -> u16 { 86 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.application_name.clone()))),
            3 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.application_version.clone()))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::VisibleString(s)|DlmsData::Utf8String(s) => { self.application_name=s.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::VisibleString(s)|DlmsData::Utf8String(s) => { self.application_version=s.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = PrimeAppIdentification::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 86);
    }
    #[test]
    fn test_getter() {
        let obj = PrimeAppIdentification::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.application_version();
    }
}
