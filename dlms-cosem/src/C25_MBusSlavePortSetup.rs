//! IC25 MBusSlavePortSetup
//! Blue Book Ed16: class_id=25, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// MBusSlavePortSetup
pub struct MBusSlavePortSetup {
    logical_name: ObisCode,
    primary_address: u8,
    identification_number: u64,
    manufacturer_id: u16,
}

impl MBusSlavePortSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            primary_address: 0,
            identification_number: 0,
            manufacturer_id: 0,
        }
    }
    pub fn primary_address(&self) -> u8 { self.primary_address }
    pub fn set_primary_address(&mut self, v: u8) { self.primary_address = v; }
    pub fn identification_number(&self) -> u64 { self.identification_number }
    pub fn set_identification_number(&mut self, v: u64) { self.identification_number = v; }
    pub fn manufacturer_id(&self) -> u16 { self.manufacturer_id }
    pub fn set_manufacturer_id(&mut self, v: u16) { self.manufacturer_id = v; }
}

impl CosemObject for MBusSlavePortSetup {
    fn class_id(&self) -> u16 { 25 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.primary_address))),
            3 => Some(dlms_axdr::encode(&DlmsData::Long64Unsigned(self.identification_number))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.manufacturer_id))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.primary_address=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Long64Unsigned(v) => { self.identification_number=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::LongUnsigned(v) => { self.manufacturer_id=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = MBusSlavePortSetup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 25);
    }
    #[test]
    fn test_getter() {
        let obj = MBusSlavePortSetup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.manufacturer_id();
    }
}
