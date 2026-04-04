//! IC022 Day Profile
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: table (array of structures: day_name, schedule_entries)
//! 3: day_schedule (array of structures: start_time, tariff)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct DayScheduleEntry {
    pub start_time: [u8; 4], // DLMS Time
    pub tariff: u8,
}

pub struct DayProfile {
    logical_name: ObisCode,
    name: String,
    schedules: Vec<DayScheduleEntry>,
}

impl DayProfile {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            name: String::new(),
            schedules: Vec::new(),
        }
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
    }
    pub fn add_schedule(&mut self, start_time: [u8; 4], tariff: u8) {
        self.schedules.push(DayScheduleEntry { start_time, tariff });
    }

    pub fn schedule_count(&self) -> usize {
        self.schedules.len()
    }
}

impl CosemObject for DayProfile {
    fn class_id(&self) -> u16 {
        22
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        3
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
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.name.as_bytes().to_vec(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.schedules
                    .iter()
                    .map(|s| {
                        DlmsData::Structure(vec![
                            DlmsData::Time(s.start_time),
                            DlmsData::Unsigned(s.tariff),
                        ])
                    })
                    .collect(),
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, _attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        Err(CosemObjectError::AttributeNotSupported(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_profile_class_id() {
        let d = DayProfile::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        assert_eq!(d.class_id(), 22);
    }

    #[test]
    fn test_day_profile_add_schedule() {
        let mut d = DayProfile::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        d.add_schedule([0x08, 0x00, 0x00, 0x00], 1);
        assert_eq!(d.schedule_count(), 1);
    }

    #[test]
    fn test_day_profile_attr3() {
        let mut d = DayProfile::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        d.add_schedule([0x08, 0x00, 0x00, 0x00], 1);
        d.add_schedule([0x18, 0x00, 0x00, 0x00], 2);
        let bytes = d.attribute_to_bytes(3).unwrap();
        assert!(!bytes.is_empty());
    }
}
