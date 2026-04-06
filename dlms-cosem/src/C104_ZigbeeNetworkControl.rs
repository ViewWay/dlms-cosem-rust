//! IC104 ZigbeeNetworkControl
//! Blue Book Ed16: class_id=104, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// ZigbeeNetworkControl
pub struct ZigbeeNetworkControl {
    logical_name: ObisCode,
    network_mode: u8,
    pan_id: u16,
    channel: u8,
}

impl ZigbeeNetworkControl {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            network_mode: 0,
            pan_id: 0,
            channel: 0,
        }
    }
    pub fn network_mode(&self) -> u8 { self.network_mode }
    pub fn set_network_mode(&mut self, v: u8) { self.network_mode = v; }
    pub fn pan_id(&self) -> u16 { self.pan_id }
    pub fn set_pan_id(&mut self, v: u16) { self.pan_id = v; }
    pub fn channel(&self) -> u8 { self.channel }
    pub fn set_channel(&mut self, v: u8) { self.channel = v; }
}

impl CosemObject for ZigbeeNetworkControl {
    fn class_id(&self) -> u16 { 104 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.network_mode))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.pan_id))),
            4 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.channel))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.network_mode=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::LongUnsigned(v) => { self.pan_id=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.channel=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = ZigbeeNetworkControl::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 104);
    }
    #[test]
    fn test_getter() {
        let obj = ZigbeeNetworkControl::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.channel();
    }
}
