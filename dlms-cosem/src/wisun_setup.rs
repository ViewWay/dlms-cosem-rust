//! IC95 Wi-SUN Setup
//! Blue Book Ed16: class_id=95, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Wi-SUN Setup - Wi-SUN network configuration
pub struct WiSunSetup {
    logical_name: ObisCode,
    phy_operating_mode: u8,
    network_mode: u8,
    pan_id: u16,
    routing_method: u8,
}

impl WiSunSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            phy_operating_mode: 0,
            network_mode: 0,
            pan_id: 0,
            routing_method: 0,
        }
    }
    pub fn pan_id(&self) -> u16 {
        self.pan_id
    }
    pub fn set_pan_id(&mut self, id: u16) {
        self.pan_id = id;
    }
    pub fn network_mode(&self) -> u8 {
        self.network_mode
    }
    pub fn set_network_mode(&mut self, mode: u8) {
        self.network_mode = mode;
    }
}

impl CosemObject for WiSunSetup {
    fn class_id(&self) -> u16 {
        95
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        18
    }
    fn method_count(&self) -> u8 {
        0
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let n = self.logical_name.to_bytes();
                Some(vec![0x09, 0x06, n[0], n[1], n[2], n[3], n[4], n[5]])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(
                self.phy_operating_mode,
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.network_mode))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.pan_id))),
            5 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.routing_method))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            4 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = d.as_i32() {
                    self.pan_id = v as u16;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_wisun_new() {
        let w = WiSunSetup::new(ObisCode::CLOCK);
        assert_eq!(w.class_id(), 95);
    }
    #[test]
    fn test_wisun_pan() {
        let mut w = WiSunSetup::new(ObisCode::CLOCK);
        w.set_pan_id(0xABCD);
        assert_eq!(w.pan_id(), 0xABCD);
    }
}
