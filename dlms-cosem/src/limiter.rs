//! IC71 Limiter
//! Blue Book Ed16: class_id=71, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Limiter - limits supply based on monitored values
pub struct Limiter {
    logical_name: ObisCode,
    monitored_value: DlmsData,
    threshold_normal: DlmsData,
    threshold_over: DlmsData,
    threshold_under: DlmsData,
    min_over_duration: u32,
    min_under_duration: u32,
    emergency_profile_active: bool,
}

impl Limiter {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            monitored_value: DlmsData::DoubleLong(0),
            threshold_normal: DlmsData::DoubleLong(100),
            threshold_over: DlmsData::DoubleLong(120),
            threshold_under: DlmsData::DoubleLong(0),
            min_over_duration: 60,
            min_under_duration: 60,
            emergency_profile_active: false,
        }
    }

    pub fn monitored_value(&self) -> &DlmsData {
        &self.monitored_value
    }
    pub fn set_monitored_value(&mut self, v: DlmsData) {
        self.monitored_value = v;
    }
    pub fn emergency_active(&self) -> bool {
        self.emergency_profile_active
    }
    pub fn set_emergency_active(&mut self, a: bool) {
        self.emergency_profile_active = a;
    }
}

impl CosemObject for Limiter {
    fn class_id(&self) -> u16 {
        71
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        11
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
            2 => Some(dlms_axdr::encode(&self.monitored_value)),
            3 => Some(dlms_axdr::encode(&self.threshold_normal)),
            4 => Some(dlms_axdr::encode(&self.threshold_over)),
            5 => Some(dlms_axdr::encode(&self.threshold_under)),
            6 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.min_over_duration,
            ))),
            7 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.min_under_duration,
            ))),
            11 => Some(dlms_axdr::encode(&DlmsData::Boolean(
                self.emergency_profile_active,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                self.monitored_value =
                    dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                Ok(())
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_limiter_new() {
        let l = Limiter::new(ObisCode::CLOCK);
        assert_eq!(l.class_id(), 71);
    }

    #[test]
    fn test_limiter_monitored_value() {
        let mut l = Limiter::new(ObisCode::CLOCK);
        l.set_monitored_value(DlmsData::DoubleLong(50));
        assert_eq!(l.monitored_value().as_i32(), Some(50));
    }

    #[test]
    fn test_limiter_roundtrip() {
        let mut l = Limiter::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(999));
        l.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(l.monitored_value().as_i32(), Some(999));
    }

    #[test]
    fn test_limiter_emergency() {
        let mut l = Limiter::new(ObisCode::CLOCK);
        l.set_emergency_active(true);
        assert!(l.emergency_active());
    }
}
