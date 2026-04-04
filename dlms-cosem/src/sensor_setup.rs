//! IC074 Sensor Setup

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Sensor Setup - configuration for a sensor
pub struct SensorSetup {
    logical_name: ObisCode,
    enabled: bool,
    update_interval: u32,
    threshold_low: DlmsData,
    threshold_high: DlmsData,
}

impl SensorSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            enabled: true,
            update_interval: 60,
            threshold_low: DlmsData::DoubleLong(0),
            threshold_high: DlmsData::DoubleLong(100),
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn update_interval(&self) -> u32 {
        self.update_interval
    }

    pub fn set_update_interval(&mut self, interval: u32) {
        self.update_interval = interval;
    }
}

impl CosemObject for SensorSetup {
    fn class_id(&self) -> u16 {
        202
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
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.update_interval,
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
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_i32() {
                    self.update_interval = v as u32;
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
    fn test_sensor_setup_new() {
        let ss = SensorSetup::new(ObisCode::CLOCK);
        assert_eq!(ss.class_id(), 202);
        assert!(ss.enabled());
    }

    #[test]
    fn test_sensor_setup_interval() {
        let mut ss = SensorSetup::new(ObisCode::CLOCK);
        ss.set_update_interval(120);
        assert_eq!(ss.update_interval(), 120);
    }

    #[test]
    fn test_sensor_setup_roundtrip() {
        let mut ss = SensorSetup::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Boolean(false));
        ss.attribute_from_bytes(2, &bytes).unwrap();
        assert!(!ss.enabled());
    }
}
