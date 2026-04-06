//! IC140 HsplcMacSetup
//! Blue Book Ed16: class_id=140, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// HsplcMacSetup
pub struct HsplcMacSetup {
    logical_name: ObisCode,
    mac_address: Vec<u8>,
    mac_security_enabled: bool,
}

impl HsplcMacSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            mac_address: Vec::new(),
            mac_security_enabled: false,
        }
    }
    pub fn mac_address(&self) -> &[u8] { &self.mac_address }
    pub fn set_mac_address(&mut self, v: Vec<u8>) { self.mac_address = v; }
    pub fn mac_security_enabled(&self) -> bool { self.mac_security_enabled }
    pub fn set_mac_security_enabled(&mut self, v: bool) { self.mac_security_enabled = v; }
}

impl CosemObject for HsplcMacSetup {
    fn class_id(&self) -> u16 { 140 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(self.mac_address.clone()))),
            3 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.mac_security_enabled))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::OctetString(v) => { self.mac_address=v.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Boolean(v) => { self.mac_security_enabled=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = HsplcMacSetup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 140);
    }
    #[test]
    fn test_getter() {
        let obj = HsplcMacSetup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.mac_security_enabled();
    }
}
