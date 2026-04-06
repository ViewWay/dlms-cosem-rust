//! IC53 SfskMacCounters
//! Blue Book Ed16: class_id=53, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// SfskMacCounters
pub struct SfskMacCounters {
    logical_name: ObisCode,
    tx_packet_count: u32,
    rx_packet_count: u32,
    crc_error_count: u32,
}

impl SfskMacCounters {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            tx_packet_count: 0,
            rx_packet_count: 0,
            crc_error_count: 0,
        }
    }
    pub fn tx_packet_count(&self) -> u32 { self.tx_packet_count }
    pub fn set_tx_packet_count(&mut self, v: u32) { self.tx_packet_count = v; }
    pub fn rx_packet_count(&self) -> u32 { self.rx_packet_count }
    pub fn set_rx_packet_count(&mut self, v: u32) { self.rx_packet_count = v; }
    pub fn crc_error_count(&self) -> u32 { self.crc_error_count }
    pub fn set_crc_error_count(&mut self, v: u32) { self.crc_error_count = v; }
}

impl CosemObject for SfskMacCounters {
    fn class_id(&self) -> u16 { 53 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.tx_packet_count))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.rx_packet_count))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.crc_error_count))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.tx_packet_count=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.rx_packet_count=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.crc_error_count=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = SfskMacCounters::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 53);
    }
    #[test]
    fn test_getter() {
        let obj = SfskMacCounters::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.crc_error_count();
    }
}
