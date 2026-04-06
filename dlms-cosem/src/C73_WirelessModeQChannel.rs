//! IC73 WirelessModeQChannel
//! Blue Book Ed16: class_id=73, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// WirelessModeQChannel
pub struct WirelessModeQChannel {
    logical_name: ObisCode,
    channel_info: String,
}

impl WirelessModeQChannel {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            channel_info: String::new(),
        }
    }
    pub fn channel_info(&self) -> &str { &self.channel_info }
    pub fn set_channel_info(&mut self, v: impl Into<String>) { self.channel_info = v.into(); }
}

impl CosemObject for WirelessModeQChannel {
    fn class_id(&self) -> u16 { 73 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 2 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.channel_info.clone()))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::VisibleString(s)|DlmsData::Utf8String(s) => { self.channel_info=s.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = WirelessModeQChannel::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 73);
    }
    #[test]
    fn test_getter() {
        let obj = WirelessModeQChannel::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.channel_info();
    }
}
