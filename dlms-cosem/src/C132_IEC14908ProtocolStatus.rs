//! IC132 Iec14908ProtocolStatus
//! Blue Book Ed16: class_id=132, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Iec14908ProtocolStatus
pub struct Iec14908ProtocolStatus {
    logical_name: ObisCode,
    protocol_status: u32,
    connection_status: u32,
}

impl Iec14908ProtocolStatus {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            protocol_status: 0,
            connection_status: 0,
        }
    }
    pub fn protocol_status(&self) -> u32 { self.protocol_status }
    pub fn set_protocol_status(&mut self, v: u32) { self.protocol_status = v; }
    pub fn connection_status(&self) -> u32 { self.connection_status }
    pub fn set_connection_status(&mut self, v: u32) { self.connection_status = v; }
}

impl CosemObject for Iec14908ProtocolStatus {
    fn class_id(&self) -> u16 { 132 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.protocol_status))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.connection_status))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.protocol_status=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.connection_status=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = Iec14908ProtocolStatus::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 132);
    }
    #[test]
    fn test_getter() {
        let obj = Iec14908ProtocolStatus::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.connection_status();
    }
}
