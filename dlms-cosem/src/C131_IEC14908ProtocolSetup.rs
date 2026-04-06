//! IC131 Iec14908ProtocolSetup
//! Blue Book Ed16: class_id=131, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Iec14908ProtocolSetup
pub struct Iec14908ProtocolSetup {
    logical_name: ObisCode,
    protocol_mode: u8,
    protocol_version: u8,
}

impl Iec14908ProtocolSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            protocol_mode: 0,
            protocol_version: 0,
        }
    }
    pub fn protocol_mode(&self) -> u8 { self.protocol_mode }
    pub fn set_protocol_mode(&mut self, v: u8) { self.protocol_mode = v; }
    pub fn protocol_version(&self) -> u8 { self.protocol_version }
    pub fn set_protocol_version(&mut self, v: u8) { self.protocol_version = v; }
}

impl CosemObject for Iec14908ProtocolSetup {
    fn class_id(&self) -> u16 { 131 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.protocol_mode))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.protocol_version))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.protocol_mode=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.protocol_version=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = Iec14908ProtocolSetup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 131);
    }
    #[test]
    fn test_getter() {
        let obj = Iec14908ProtocolSetup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.protocol_version();
    }
}
