//! IC83 PrimeMacFuncParams
//! Blue Book Ed16: class_id=83, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// PrimeMacFuncParams
pub struct PrimeMacFuncParams {
    logical_name: ObisCode,
    mac_address: Vec<u8>,
    mac_frame_counter: u32,
}

impl PrimeMacFuncParams {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            mac_address: Vec::new(),
            mac_frame_counter: 0,
        }
    }
    pub fn mac_address(&self) -> &[u8] { &self.mac_address }
    pub fn set_mac_address(&mut self, v: Vec<u8>) { self.mac_address = v; }
    pub fn mac_frame_counter(&self) -> u32 { self.mac_frame_counter }
    pub fn set_mac_frame_counter(&mut self, v: u32) { self.mac_frame_counter = v; }
}

impl CosemObject for PrimeMacFuncParams {
    fn class_id(&self) -> u16 { 83 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(self.mac_address.clone()))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.mac_frame_counter))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::OctetString(v) => { self.mac_address=v.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.mac_frame_counter=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = PrimeMacFuncParams::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 83);
    }
    #[test]
    fn test_getter() {
        let obj = PrimeMacFuncParams::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.mac_frame_counter();
    }
}
