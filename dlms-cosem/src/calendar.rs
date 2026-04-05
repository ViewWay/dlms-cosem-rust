//! Calendar Management - Manages calendar-related operations
//!
//! Provides calendar functionality for scheduling and time-based
//! operations in the metering system.

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Season entry for calendar
#[derive(Debug, Clone)]
pub struct SeasonEntry {
    pub season_name: String,
    pub start_date: [u8; 5], // day, month, year, etc.
    pub end_date: [u8; 5],
}

/// Week day profile
#[derive(Debug, Clone)]
pub struct WeekDayProfile {
    pub day_id: u8,
    pub day_schedule_id: u8,
}

/// IC204 Calendar - Manages calendar operations
pub struct Calendar {
    logical_name: ObisCode,
    seasons: Vec<SeasonEntry>,
    week_profiles: Vec<WeekDayProfile>,
    special_days: Vec<(u16, u8)>, // (day_of_year, special_day_id)
}

impl Calendar {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            seasons: Vec::new(),
            week_profiles: Vec::new(),
            special_days: Vec::new(),
        }
    }

    pub fn add_season(&mut self, name: String, start: [u8; 5], end: [u8; 5]) {
        self.seasons.push(SeasonEntry {
            season_name: name,
            start_date: start,
            end_date: end,
        });
    }

    pub fn add_week_profile(&mut self, day_id: u8, schedule_id: u8) {
        self.week_profiles.push(WeekDayProfile {
            day_id,
            day_schedule_id: schedule_id,
        });
    }

    pub fn add_special_day(&mut self, day_of_year: u16, special_day_id: u8) {
        self.special_days.push((day_of_year, special_day_id));
    }

    pub fn season_count(&self) -> usize {
        self.seasons.len()
    }

    pub fn week_profile_count(&self) -> usize {
        self.week_profiles.len()
    }

    pub fn special_day_count(&self) -> usize {
        self.special_days.len()
    }

    pub fn get_week_profile(&self, day_id: u8) -> Option<&WeekDayProfile> {
        self.week_profiles.iter().find(|p| p.day_id == day_id)
    }
}

impl CosemObject for Calendar {
    fn class_id(&self) -> u16 {
        204
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
            2 => {
                let seasons: Vec<DlmsData> = self
                    .seasons
                    .iter()
                    .map(|s| {
                        DlmsData::Structure(vec![
                            DlmsData::OctetString(s.season_name.as_bytes().to_vec()),
                            DlmsData::OctetString(s.start_date.to_vec()),
                            DlmsData::OctetString(s.end_date.to_vec()),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(seasons)))
            }
            3 => {
                let profiles: Vec<DlmsData> = self
                    .week_profiles
                    .iter()
                    .map(|p| {
                        DlmsData::Structure(vec![
                            DlmsData::Unsigned(p.day_id),
                            DlmsData::Unsigned(p.day_schedule_id),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(profiles)))
            }
            4 => {
                let days: Vec<DlmsData> = self
                    .special_days
                    .iter()
                    .map(|&(day, id)| {
                        DlmsData::Structure(vec![
                            DlmsData::LongUnsigned(day),
                            DlmsData::Unsigned(id),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(days)))
            }
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
    fn test_calendar_class_id() {
        let cal = Calendar::new(ObisCode::new(0, 0, 204, 0, 0, 255));
        assert_eq!(cal.class_id(), 204);
    }

    #[test]
    fn test_calendar_new() {
        let cal = Calendar::new(ObisCode::new(0, 0, 204, 0, 0, 255));
        assert_eq!(cal.season_count(), 0);
        assert_eq!(cal.week_profile_count(), 0);
    }

    #[test]
    fn test_calendar_add_season() {
        let mut cal = Calendar::new(ObisCode::new(0, 0, 204, 0, 0, 255));
        cal.add_season("Summer".to_string(), [1, 6, 24, 0, 0], [31, 8, 24, 0, 0]);
        assert_eq!(cal.season_count(), 1);
    }

    #[test]
    fn test_calendar_add_week_profile() {
        let mut cal = Calendar::new(ObisCode::new(0, 0, 204, 0, 0, 255));
        cal.add_week_profile(1, 10); // Monday
        assert_eq!(cal.week_profile_count(), 1);
    }

    #[test]
    fn test_calendar_add_special_day() {
        let mut cal = Calendar::new(ObisCode::new(0, 0, 204, 0, 0, 255));
        cal.add_special_day(1, 1); // Jan 1
        assert_eq!(cal.special_day_count(), 1);
    }

    #[test]
    fn test_calendar_get_week_profile() {
        let mut cal = Calendar::new(ObisCode::new(0, 0, 204, 0, 0, 255));
        cal.add_week_profile(1, 10);
        let profile = cal.get_week_profile(1);
        assert!(profile.is_some());
        assert_eq!(profile.unwrap().day_schedule_id, 10);
    }

    #[test]
    fn test_calendar_attribute_count() {
        let cal = Calendar::new(ObisCode::new(0, 0, 204, 0, 0, 255));
        assert_eq!(cal.attribute_count(), 5);
    }

    #[test]
    fn test_calendar_method_count() {
        let cal = Calendar::new(ObisCode::new(0, 0, 204, 0, 0, 255));
        assert_eq!(cal.method_count(), 0);
    }

    #[test]
    fn test_calendar_multiple_seasons() {
        let mut cal = Calendar::new(ObisCode::new(0, 0, 204, 0, 0, 255));
        cal.add_season("Summer".to_string(), [1, 6, 24, 0, 0], [31, 8, 24, 0, 0]);
        cal.add_season("Winter".to_string(), [1, 12, 24, 0, 0], [28, 2, 25, 0, 0]);
        assert_eq!(cal.season_count(), 2);
    }

    #[test]
    fn test_calendar_multiple_special_days() {
        let mut cal = Calendar::new(ObisCode::new(0, 0, 204, 0, 0, 255));
        cal.add_special_day(1, 1); // New Year
        cal.add_special_day(365, 2); // New Year Eve
        assert_eq!(cal.special_day_count(), 2);
    }
}
