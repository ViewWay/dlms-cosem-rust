//! IC51 SfskActiveInitiator
//! Blue Book Ed16: class_id=51, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// SfskActiveInitiator
pub struct SfskActiveInitiator {
    logical_name: ObisCode,
    active_initiator: Vec<u8>,
    active_initiator_count: u8,
}

impl SfskActiveInitiator {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            active_initiator: Vec::new(),
            active_initiator_count: 0,
        }
    }
    pub fn active_initiator(&self) -> &[u8] { &self.active_initiator }
    pub fn set_active_initiator(&mut self, v: Vec<u8>) { self.active_initiator = v; }
    pub fn active_initiator_count(&self) -> u8 { self.active_initiator_count }
    pub fn set_active_initiator_count(&mut self, v: u8) { self.active_initiator_count = v; }
}

impl CosemObject for SfskActiveInitiator {
    fn class_id(&self) -> u16 { 51 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(self.active_initiator.clone()))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.active_initiator_count))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::OctetString(v) => { self.active_initiator=v.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.active_initiator_count=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = SfskActiveInitiator::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 51);
    }
    #[test]
    fn test_getter() {
        let obj = SfskActiveInitiator::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.active_initiator_count();
    }
}
