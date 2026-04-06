//! IC142 HsplcIpSsasSetup
//! Blue Book Ed16: class_id=142, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// HsplcIpSsasSetup
pub struct HsplcIpSsasSetup {
    logical_name: ObisCode,
    ip_ssas_enable: bool,
    ip_ssas_mtu: u16,
}

impl HsplcIpSsasSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            ip_ssas_enable: false,
            ip_ssas_mtu: 0,
        }
    }
    pub fn ip_ssas_enable(&self) -> bool { self.ip_ssas_enable }
    pub fn set_ip_ssas_enable(&mut self, v: bool) { self.ip_ssas_enable = v; }
    pub fn ip_ssas_mtu(&self) -> u16 { self.ip_ssas_mtu }
    pub fn set_ip_ssas_mtu(&mut self, v: u16) { self.ip_ssas_mtu = v; }
}

impl CosemObject for HsplcIpSsasSetup {
    fn class_id(&self) -> u16 { 142 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.ip_ssas_enable))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.ip_ssas_mtu))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Boolean(v) => { self.ip_ssas_enable=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::LongUnsigned(v) => { self.ip_ssas_mtu=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = HsplcIpSsasSetup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 142);
    }
    #[test]
    fn test_getter() {
        let obj = HsplcIpSsasSetup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.ip_ssas_mtu();
    }
}
