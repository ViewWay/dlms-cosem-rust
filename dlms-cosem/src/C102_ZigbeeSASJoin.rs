//! IC102 ZigbeeSasJoin
//! Blue Book Ed16: class_id=102, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// ZigbeeSasJoin
pub struct ZigbeeSasJoin {
    logical_name: ObisCode,
    join_control: u8,
    security_level: u8,
    network_key: Vec<u8>,
}

impl ZigbeeSasJoin {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            join_control: 0,
            security_level: 0,
            network_key: Vec::new(),
        }
    }
    pub fn join_control(&self) -> u8 { self.join_control }
    pub fn set_join_control(&mut self, v: u8) { self.join_control = v; }
    pub fn security_level(&self) -> u8 { self.security_level }
    pub fn set_security_level(&mut self, v: u8) { self.security_level = v; }
    pub fn network_key(&self) -> &[u8] { &self.network_key }
    pub fn set_network_key(&mut self, v: Vec<u8>) { self.network_key = v; }
}

impl CosemObject for ZigbeeSasJoin {
    fn class_id(&self) -> u16 { 102 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.join_control))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.security_level))),
            4 => Some(dlms_axdr::encode(&DlmsData::OctetString(self.network_key.clone()))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.join_control=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.security_level=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::OctetString(v) => { self.network_key=v.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = ZigbeeSasJoin::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 102);
    }
    #[test]
    fn test_getter() {
        let obj = ZigbeeSasJoin::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.network_key();
    }
}
