//! IC19 IecLocalPortSetup
//! Blue Book Ed16: class_id=19, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IecLocalPortSetup
pub struct IecLocalPortSetup {
    logical_name: ObisCode,
    default_mode: u8,
    default_baud: u8,
    baud: u32,
    local_port_state: u8,
}

impl IecLocalPortSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            default_mode: 0,
            default_baud: 0,
            baud: 0,
            local_port_state: 0,
        }
    }
    pub fn default_mode(&self) -> u8 { self.default_mode }
    pub fn set_default_mode(&mut self, v: u8) { self.default_mode = v; }
    pub fn default_baud(&self) -> u8 { self.default_baud }
    pub fn set_default_baud(&mut self, v: u8) { self.default_baud = v; }
    pub fn baud(&self) -> u32 { self.baud }
    pub fn set_baud(&mut self, v: u32) { self.baud = v; }
    pub fn local_port_state(&self) -> u8 { self.local_port_state }
    pub fn set_local_port_state(&mut self, v: u8) { self.local_port_state = v; }
}

impl CosemObject for IecLocalPortSetup {
    fn class_id(&self) -> u16 { 19 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 5 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.default_mode))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.default_baud))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.baud))),
            5 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.local_port_state))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.default_mode=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.default_baud=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.baud=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            5 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.local_port_state=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = IecLocalPortSetup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 19);
    }
    #[test]
    fn test_getter() {
        let obj = IecLocalPortSetup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.local_port_state();
    }
}
