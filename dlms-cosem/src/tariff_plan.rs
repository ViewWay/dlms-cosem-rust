//! IC018 Tariff Plan / Schedule
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: table (array of structures: season_name, season_profile, week_name, week_profile, day_name, day_profile)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct TariffPlanEntry {
    pub season_name: String,
    pub week_name: String,
    pub day_name: String,
}

pub struct TariffPlan {
    logical_name: ObisCode,
    entries: Vec<TariffPlanEntry>,
}

impl TariffPlan {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, season: &str, week: &str, day: &str) {
        self.entries.push(TariffPlanEntry {
            season_name: season.into(),
            week_name: week.into(),
            day_name: day.into(),
        });
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

impl CosemObject for TariffPlan {
    fn class_id(&self) -> u16 {
        18
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        2
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
            2 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.entries
                    .iter()
                    .map(|e| {
                        DlmsData::Structure(vec![
                            DlmsData::OctetString(e.season_name.as_bytes().to_vec()),
                            DlmsData::OctetString(e.week_name.as_bytes().to_vec()),
                            DlmsData::OctetString(e.day_name.as_bytes().to_vec()),
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
    fn test_tariff_plan_class_id() {
        let t = TariffPlan::new(ObisCode::new(0, 0, 18, 0, 0, 255));
        assert_eq!(t.class_id(), 18);
    }

    #[test]
    fn test_tariff_plan_add_entry() {
        let mut t = TariffPlan::new(ObisCode::new(0, 0, 18, 0, 0, 255));
        t.add_entry("Summer", "Week1", "Day1");
        assert_eq!(t.entry_count(), 1);
    }

    #[test]
    fn test_tariff_plan_attr2() {
        let mut t = TariffPlan::new(ObisCode::new(0, 0, 18, 0, 0, 255));
        t.add_entry("Summer", "Week1", "Day1");
        let bytes = t.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }
}
