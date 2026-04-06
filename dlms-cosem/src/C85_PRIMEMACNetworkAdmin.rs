//! IC85 PrimeMacNetworkAdmin
//! Blue Book Ed16: class_id=85, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// PrimeMacNetworkAdmin
pub struct PrimeMacNetworkAdmin {
    logical_name: ObisCode,
    network_id: u32,
    network_role: u8,
    network_state: u8,
}

impl PrimeMacNetworkAdmin {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            network_id: 0,
            network_role: 0,
            network_state: 0,
        }
    }
    pub fn network_id(&self) -> u32 { self.network_id }
    pub fn set_network_id(&mut self, v: u32) { self.network_id = v; }
    pub fn network_role(&self) -> u8 { self.network_role }
    pub fn set_network_role(&mut self, v: u8) { self.network_role = v; }
    pub fn network_state(&self) -> u8 { self.network_state }
    pub fn set_network_state(&mut self, v: u8) { self.network_state = v; }
}

impl CosemObject for PrimeMacNetworkAdmin {
    fn class_id(&self) -> u16 { 85 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.network_id))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.network_role))),
            4 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.network_state))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.network_id=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.network_role=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.network_state=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = PrimeMacNetworkAdmin::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 85);
    }
    #[test]
    fn test_getter() {
        let obj = PrimeMacNetworkAdmin::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.network_state();
    }
}
