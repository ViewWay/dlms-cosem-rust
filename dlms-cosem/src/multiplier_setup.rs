//! IC091 Multiplier Setup

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Multiplier Setup - configuration for a multiplier
pub struct MultiplierSetup {
    logical_name: ObisCode,
    target_obis: ObisCode,
    enabled: bool,
}

impl MultiplierSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            target_obis: ObisCode::CLOCK,
            enabled: false,
        }
    }

    pub fn target_obis(&self) -> ObisCode {
        self.target_obis
    }

    pub fn set_target_obis(&mut self, obis: ObisCode) {
        self.target_obis = obis;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl CosemObject for MultiplierSetup {
    fn class_id(&self) -> u16 {
        91
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        4
    }
    fn method_count(&self) -> u8 {
        0
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => {
                let obis = self.target_obis.to_bytes();
                Some(dlms_axdr::encode(&DlmsData::OctetString(obis.to_vec())))
            }
            3 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.enabled))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(bytes) = decoded.as_octet_string() {
                    if bytes.len() == 6 {
                        let arr: [u8; 6] = bytes.try_into().unwrap();
                        self.target_obis = ObisCode::from_bytes(arr);
                        Ok(())
                    } else {
                        Err(CosemObjectError::InvalidData)
                    }
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(b) = decoded.as_bool() {
                    self.enabled = b;
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
    fn test_multiplier_setup_new() {
        let ms = MultiplierSetup::new(ObisCode::CLOCK);
        assert_eq!(ms.class_id(), 91);
    }

    #[test]
    fn test_multiplier_setup_enabled() {
        let mut ms = MultiplierSetup::new(ObisCode::CLOCK);
        ms.set_enabled(true);
        assert!(ms.enabled());
    }

    #[test]
    fn test_multiplier_setup_roundtrip() {
        let mut ms = MultiplierSetup::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Boolean(true));
        ms.attribute_from_bytes(3, &bytes).unwrap();
        assert!(ms.enabled());
    }
}
