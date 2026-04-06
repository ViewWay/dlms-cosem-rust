//! IC81 PrimePhysicalCounters
//! Blue Book Ed16: class_id=81, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// PrimePhysicalCounters
pub struct PrimePhysicalCounters {
    logical_name: ObisCode,
    phy_tx_drop: u32,
    phy_rx_total: u32,
    phy_rx_crc_error: u32,
}

impl PrimePhysicalCounters {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            phy_tx_drop: 0,
            phy_rx_total: 0,
            phy_rx_crc_error: 0,
        }
    }
    pub fn phy_tx_drop(&self) -> u32 { self.phy_tx_drop }
    pub fn set_phy_tx_drop(&mut self, v: u32) { self.phy_tx_drop = v; }
    pub fn phy_rx_total(&self) -> u32 { self.phy_rx_total }
    pub fn set_phy_rx_total(&mut self, v: u32) { self.phy_rx_total = v; }
    pub fn phy_rx_crc_error(&self) -> u32 { self.phy_rx_crc_error }
    pub fn set_phy_rx_crc_error(&mut self, v: u32) { self.phy_rx_crc_error = v; }
}

impl CosemObject for PrimePhysicalCounters {
    fn class_id(&self) -> u16 { 81 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.phy_tx_drop))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.phy_rx_total))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.phy_rx_crc_error))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.phy_tx_drop=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.phy_rx_total=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.phy_rx_crc_error=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = PrimePhysicalCounters::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 81);
    }
    #[test]
    fn test_getter() {
        let obj = PrimePhysicalCounters::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.phy_rx_crc_error();
    }
}
