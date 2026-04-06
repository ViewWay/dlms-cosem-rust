//! IC026 Utility Tables

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Load Profile Switch - controls load profile capture
pub struct UtilityTables {
    logical_name: ObisCode,
    active_profile: u8,
    profiles: Vec<DlmsData>,
    period: u32,
}

impl UtilityTables {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            active_profile: 0,
            profiles: vec![],
            period: 60,
        }
    }

    pub fn active_profile(&self) -> u8 {
        self.active_profile
    }

    pub fn set_active_profile(&mut self, profile: u8) {
        self.active_profile = profile;
    }

    pub fn profiles(&self) -> &[DlmsData] {
        &self.profiles
    }

    pub fn period(&self) -> u32 {
        self.period
    }

    pub fn set_period(&mut self, period: u32) {
        self.period = period;
    }
}

impl CosemObject for UtilityTables {
    fn class_id(&self) -> u16 {
        90
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
            2 => Some(dlms_axdr::encode(&DlmsData::Enum(self.active_profile))),
            3 => Some(dlms_axdr::encode(&DlmsData::Array(self.profiles.clone()))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.period,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_u8() {
                    self.active_profile = v;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_i32() {
                    self.period = v as u32;
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
    fn test_load_profile_switch_new() {
        let lps = UtilityTables::new(ObisCode::CLOCK);
        assert_eq!(lps.class_id(), 90);
    }

    #[test]
    fn test_load_profile_switch_active_profile() {
        let mut lps = UtilityTables::new(ObisCode::CLOCK);
        lps.set_active_profile(2);
        assert_eq!(lps.active_profile(), 2);
    }

    #[test]
    fn test_load_profile_switch_period() {
        let mut lps = UtilityTables::new(ObisCode::CLOCK);
        lps.set_period(900);
        assert_eq!(lps.period(), 900);
    }

    #[test]
    fn test_load_profile_switch_roundtrip() {
        let mut lps = UtilityTables::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Unsigned(3));
        lps.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(lps.active_profile(), 3);
    }

    #[test]
    fn test_load_profile_switch_attr1() {
        let lps = UtilityTables::new(ObisCode::CLOCK);
        let bytes = lps.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }
}
