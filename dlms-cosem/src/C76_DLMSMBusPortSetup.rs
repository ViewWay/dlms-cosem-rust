//! IC76 DlmsMBusPortSetup
//! Blue Book Ed16: class_id=76, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// DlmsMBusPortSetup
pub struct DlmsMBusPortSetup {
    logical_name: ObisCode,
    m_bus_port_reference: ObisCode,
    listen_port: u16,
    slave_devices: u16,
}

impl DlmsMBusPortSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            m_bus_port_reference: ObisCode::CLOCK,
            listen_port: 0,
            slave_devices: 0,
        }
    }
    pub fn m_bus_port_reference(&self) -> ObisCode { self.m_bus_port_reference }
    pub fn set_m_bus_port_reference(&mut self, v: ObisCode) { self.m_bus_port_reference = v; }
    pub fn listen_port(&self) -> u16 { self.listen_port }
    pub fn set_listen_port(&mut self, v: u16) { self.listen_port = v; }
    pub fn slave_devices(&self) -> u16 { self.slave_devices }
    pub fn set_slave_devices(&mut self, v: u16) { self.slave_devices = v; }
}

impl CosemObject for DlmsMBusPortSetup {
    fn class_id(&self) -> u16 { 76 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 4 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => { let n=self.m_bus_port_reference.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) },
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.listen_port))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.slave_devices))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { if data.len()>=8{let arr:[u8;6]=data[2..8].try_into().map_err(|_|CosemObjectError::InvalidData)?;self.m_bus_port_reference=ObisCode::from_bytes(arr);Ok(())}else{Err(CosemObjectError::InvalidData)} }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::LongUnsigned(v) => { self.listen_port=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::LongUnsigned(v) => { self.slave_devices=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = DlmsMBusPortSetup::new(ObisCode::CLOCK);
        assert_eq!(obj.class_id(), 76);
    }
    #[test]
    fn test_getter() {
        let obj = DlmsMBusPortSetup::new(ObisCode::CLOCK);
        let _ = obj.slave_devices();
    }
}
