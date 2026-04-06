//! IC50 SfskPhyMacSetup
//! Blue Book Ed16: class_id=50, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// SfskPhyMacSetup
pub struct SfskPhyMacSetup {
    logical_name: ObisCode,
    mac_address: Vec<u8>,
    tx_level: u8,
    rx_level: u8,
    frequency_band: String,
}

impl SfskPhyMacSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            mac_address: Vec::new(),
            tx_level: 0,
            rx_level: 0,
            frequency_band: String::new(),
        }
    }
    pub fn mac_address(&self) -> &[u8] { &self.mac_address }
    pub fn set_mac_address(&mut self, v: Vec<u8>) { self.mac_address = v; }
    pub fn tx_level(&self) -> u8 { self.tx_level }
    pub fn set_tx_level(&mut self, v: u8) { self.tx_level = v; }
    pub fn rx_level(&self) -> u8 { self.rx_level }
    pub fn set_rx_level(&mut self, v: u8) { self.rx_level = v; }
    pub fn frequency_band(&self) -> &str { &self.frequency_band }
    pub fn set_frequency_band(&mut self, v: impl Into<String>) { self.frequency_band = v.into(); }
}

impl CosemObject for SfskPhyMacSetup {
    fn class_id(&self) -> u16 { 50 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 5 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(self.mac_address.clone()))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.tx_level))),
            4 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.rx_level))),
            5 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.frequency_band.clone()))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::OctetString(v) => { self.mac_address=v.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.tx_level=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::Unsigned(v) => { self.rx_level=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            5 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::VisibleString(s)|DlmsData::Utf8String(s) => { self.frequency_band=s.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = SfskPhyMacSetup::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 50);
    }
    #[test]
    fn test_getter() {
        let obj = SfskPhyMacSetup::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.frequency_band();
    }
}
