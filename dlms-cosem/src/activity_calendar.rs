//! IC012 Activity Calendar
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: calendar_name (octet-string)
//! 3: activate_passive_calendar_time (date-time)
//! 4: season_profile_active (array)
//! 5: season_profile_passive (array)
//! 6: week_profile_active (array)
//! 7: week_profile_passive (array)
//! 8: day_profile_active (array)
//! 9: day_profile_passive (array)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct ActivityCalendar {
    logical_name: ObisCode,
    calendar_name: String,
    activate_passive_time: DlmsData,
}

impl ActivityCalendar {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            calendar_name: String::new(),
            activate_passive_time: DlmsData::DateTime([0u8; 12]),
        }
    }

    pub fn set_calendar_name(&mut self, name: &str) {
        self.calendar_name = name.into();
    }
    pub fn calendar_name(&self) -> &str {
        &self.calendar_name
    }
}

impl CosemObject for ActivityCalendar {
    fn class_id(&self) -> u16 {
        20
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        9
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
                self.calendar_name.as_bytes().to_vec(),
            ))),
            3 => Some(dlms_axdr::encode(&self.activate_passive_time)),
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
    fn test_activity_calendar_class_id() {
        let ac = ActivityCalendar::new(ObisCode::new(0, 0, 12, 0, 0, 255));
        assert_eq!(ac.class_id(), 20);
    }

    #[test]
    fn test_activity_calendar_set_name() {
        let mut ac = ActivityCalendar::new(ObisCode::new(0, 0, 12, 0, 0, 255));
        ac.set_calendar_name("Summer");
        assert_eq!(ac.calendar_name(), "Summer");
    }
}
