//! IC52 SfskMacSyncTimeouts
//! Blue Book Ed16: class_id=52, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// SfskMacSyncTimeouts
pub struct SfskMacSyncTimeouts {
    logical_name: ObisCode,
    time_outs: String,
}

impl SfskMacSyncTimeouts {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            time_outs: String::new(),
        }
    }
    pub fn time_outs(&self) -> &str { &self.time_outs }
    pub fn set_time_outs(&mut self, v: impl Into<String>) { self.time_outs = v.into(); }
}

impl CosemObject for SfskMacSyncTimeouts {
    fn class_id(&self) -> u16 { 52 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 2 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.time_outs.clone()))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::VisibleString(s)|DlmsData::Utf8String(s) => { self.time_outs=s.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = SfskMacSyncTimeouts::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 52);
    }
    #[test]
    fn test_getter() {
        let obj = SfskMacSyncTimeouts::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.time_outs();
    }
}
