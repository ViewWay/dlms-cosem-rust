//! IC160 G3HybridRfCounters
//! Blue Book Ed16: class_id=160, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// G3HybridRfCounters
pub struct G3HybridRfCounters {
    logical_name: ObisCode,
    mac_tx_packet_count: u32,
    mac_rx_packet_count: u32,
}

impl G3HybridRfCounters {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            mac_tx_packet_count: 0,
            mac_rx_packet_count: 0,
        }
    }
    pub fn mac_tx_packet_count(&self) -> u32 { self.mac_tx_packet_count }
    pub fn set_mac_tx_packet_count(&mut self, v: u32) { self.mac_tx_packet_count = v; }
    pub fn mac_rx_packet_count(&self) -> u32 { self.mac_rx_packet_count }
    pub fn set_mac_rx_packet_count(&mut self, v: u32) { self.mac_rx_packet_count = v; }
}

impl CosemObject for G3HybridRfCounters {
    fn class_id(&self) -> u16 { 160 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 3 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.mac_tx_packet_count))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.mac_rx_packet_count))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.mac_tx_packet_count=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.mac_rx_packet_count=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = G3HybridRfCounters::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 160);
    }
    #[test]
    fn test_getter() {
        let obj = G3HybridRfCounters::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.mac_rx_packet_count();
    }
}
