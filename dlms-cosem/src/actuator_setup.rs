//! IC076 Actuator Setup

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Actuator Setup - configuration for an actuator
pub struct ActuatorSetup {
    logical_name: ObisCode,
    enabled: bool,
    default_state: u8,
    auto_off_delay: u32,
}

impl ActuatorSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            enabled: true,
            default_state: 0,
            auto_off_delay: 0,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn default_state(&self) -> u8 {
        self.default_state
    }

    pub fn set_default_state(&mut self, state: u8) {
        self.default_state = state;
    }

    pub fn auto_off_delay(&self) -> u32 {
        self.auto_off_delay
    }

    pub fn set_auto_off_delay(&mut self, delay: u32) {
        self.auto_off_delay = delay;
    }
}

impl CosemObject for ActuatorSetup {
    fn class_id(&self) -> u16 {
        76
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        5
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
            2 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.enabled))),
            3 => Some(dlms_axdr::encode(&DlmsData::Enum(self.default_state))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.auto_off_delay,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(b) = decoded.as_bool() {
                    self.enabled = b;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_i32() {
                    self.auto_off_delay = v as u32;
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
    fn test_actuator_setup_new() {
        let asetup = ActuatorSetup::new(ObisCode::CLOCK);
        assert_eq!(asetup.class_id(), 76);
    }

    #[test]
    fn test_actuator_setup_enabled() {
        let mut asetup = ActuatorSetup::new(ObisCode::CLOCK);
        asetup.set_enabled(false);
        assert!(!asetup.enabled());
    }

    #[test]
    fn test_actuator_setup_roundtrip() {
        let mut asetup = ActuatorSetup::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Boolean(false));
        asetup.attribute_from_bytes(2, &bytes).unwrap();
        assert!(!asetup.enabled());
    }

    #[test]
    fn test_actuator_setup_delay() {
        let mut asetup = ActuatorSetup::new(ObisCode::CLOCK);
        asetup.set_auto_off_delay(3600);
        assert_eq!(asetup.auto_off_delay(), 3600);
    }
}
