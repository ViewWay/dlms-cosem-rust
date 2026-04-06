//! IC84 PrimeMacCounters
//! Blue Book Ed16: class_id=84, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// PrimeMacCounters
pub struct PrimeMacCounters {
    logical_name: ObisCode,
    mac_tx_total: u32,
    mac_rx_total: u32,
    mac_tx_error: u32,
}

impl PrimeMacCounters {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            mac_tx_total: 0,
            mac_rx_total: 0,
            mac_tx_error: 0,
        }
    }
    pub fn mac_tx_total(&self) -> u32 { self.mac_tx_total }
    pub fn set_mac_tx_total(&mut self, v: u32) { self.mac_tx_total = v; }
    pub fn mac_rx_total(&self) -> u32 { self.mac_rx_total }
    pub fn set_mac_rx_total(&mut self, v: u32) { self.mac_rx_total = v; }
    pub fn mac_tx_error(&self) -> u32 { self.mac_tx_error }
    pub fn set_mac_tx_error(&mut self, v: u32) { self.mac_tx_error = v; }
}

impl CosemObject for PrimeMacCounters {
    fn class_id(&self) -> u16 { 84 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.mac_tx_total))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.mac_rx_total))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.mac_tx_error))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.mac_tx_total=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.mac_rx_total=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.mac_tx_error=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = PrimeMacCounters::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 84);
    }
    #[test]
    fn test_getter() {
        let obj = PrimeMacCounters::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.mac_tx_error();
    }
}
