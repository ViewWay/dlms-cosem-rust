//! IC130 Iec14908Identification
//! Blue Book Ed16: class_id=130, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Iec14908Identification
pub struct Iec14908Identification {
    logical_name: ObisCode,
    domain_address: u32,
    subnet_address: u16,
    node_address: u16,
}

impl Iec14908Identification {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            domain_address: 0,
            subnet_address: 0,
            node_address: 0,
        }
    }
    pub fn domain_address(&self) -> u32 { self.domain_address }
    pub fn set_domain_address(&mut self, v: u32) { self.domain_address = v; }
    pub fn subnet_address(&self) -> u16 { self.subnet_address }
    pub fn set_subnet_address(&mut self, v: u16) { self.subnet_address = v; }
    pub fn node_address(&self) -> u16 { self.node_address }
    pub fn set_node_address(&mut self, v: u16) { self.node_address = v; }
}

impl CosemObject for Iec14908Identification {
    fn class_id(&self) -> u16 { 130 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.domain_address))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.subnet_address))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.node_address))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.domain_address=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::LongUnsigned(v) => { self.subnet_address=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::LongUnsigned(v) => { self.node_address=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = Iec14908Identification::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 130);
    }
    #[test]
    fn test_getter() {
        let obj = Iec14908Identification::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.node_address();
    }
}
