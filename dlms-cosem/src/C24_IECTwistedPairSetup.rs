//! IC24 IecTwistedPairSetup
//! Blue Book Ed16: class_id=24, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IecTwistedPairSetup
pub struct IecTwistedPairSetup {
    logical_name: ObisCode,
    mode: u8,
    speed: u32,
    convert_time: u32,
    repetitions: u8,
}

impl IecTwistedPairSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            mode: 0,
            speed: 0,
            convert_time: 0,
            repetitions: 0,
        }
    }
    pub fn mode(&self) -> u8 { self.mode }
    pub fn set_mode(&mut self, v: u8) { self.mode = v; }
    pub fn speed(&self) -> u32 { self.speed }
    pub fn set_speed(&mut self, v: u32) { self.speed = v; }
    pub fn convert_time(&self) -> u32 { self.convert_time }
    pub fn set_convert_time(&mut self, v: u32) { self.convert_time = v; }
    pub fn repetitions(&self) -> u8 { self.repetitions }
    pub fn set_repetitions(&mut self, v: u8) { self.repetitions = v; }
}

impl CosemObject for IecTwistedPairSetup {
    fn class_id(&self) -> u16 { 24 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 5 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.mode))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.speed))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.convert_time))),
            5 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.repetitions))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.mode=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.speed=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.convert_time=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            5 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.repetitions=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = IecTwistedPairSetup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 24);
    }
    #[test]
    fn test_getter() {
        let obj = IecTwistedPairSetup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.repetitions();
    }
}
