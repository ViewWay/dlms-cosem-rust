//! IC101 ZigbeeSasStartup
//! Blue Book Ed16: class_id=101, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// ZigbeeSasStartup
pub struct ZigbeeSasStartup {
    logical_name: ObisCode,
    startup_control: u8,
    channel_mask: u32,
    security_level: u8,
}

impl ZigbeeSasStartup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            startup_control: 0,
            channel_mask: 0,
            security_level: 0,
        }
    }
    pub fn startup_control(&self) -> u8 { self.startup_control }
    pub fn set_startup_control(&mut self, v: u8) { self.startup_control = v; }
    pub fn channel_mask(&self) -> u32 { self.channel_mask }
    pub fn set_channel_mask(&mut self, v: u32) { self.channel_mask = v; }
    pub fn security_level(&self) -> u8 { self.security_level }
    pub fn set_security_level(&mut self, v: u8) { self.security_level = v; }
}

impl CosemObject for ZigbeeSasStartup {
    fn class_id(&self) -> u16 { 101 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.startup_control))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.channel_mask))),
            4 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.security_level))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.startup_control=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.channel_mask=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.security_level=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = ZigbeeSasStartup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 101);
    }
    #[test]
    fn test_getter() {
        let obj = ZigbeeSasStartup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.security_level();
    }
}
