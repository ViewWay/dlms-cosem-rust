//! IC012 Schedule
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: entries (array of schedule entries)
//!
//! Methods:
//! None typically

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct ScheduleEntry {
    pub index: u16,
    pub enable: bool,
    pub script_selector: u8,
    pub start_time: [u8; 4], // DLMS time
    pub valid_for_days: u8,
}

pub struct Schedule {
    logical_name: ObisCode,
    entries: Vec<ScheduleEntry>,
}

impl Schedule {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            entries: Vec::new(),
        }
    }

    pub fn entries(&self) -> &[ScheduleEntry] {
        &self.entries
    }

    pub fn add_entry(&mut self, entry: ScheduleEntry) {
        self.entries.push(entry);
    }

    pub fn remove_entry(&mut self, index: usize) -> Option<ScheduleEntry> {
        if index < self.entries.len() {
            Some(self.entries.remove(index))
        } else {
            None
        }
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

impl CosemObject for Schedule {
    fn class_id(&self) -> u16 {
        12
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
                            DlmsData::Unsigned(e.index as u8),
                            DlmsData::Boolean(e.enable),
                            DlmsData::Unsigned(e.script_selector),
                            DlmsData::Time(e.start_time),
                            DlmsData::Unsigned(e.valid_for_days),
                        ])
                    })
                    .collect(),
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, _attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        Err(CosemObjectError::AttributeNotSupported(_attr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schedule_class_id() {
        let s = Schedule::new(ObisCode::new(0, 0, 12, 0, 0, 255));
        assert_eq!(s.class_id(), 12);
    }

    #[test]
    fn test_schedule_attribute_count() {
        let s = Schedule::new(ObisCode::new(0, 0, 12, 0, 0, 255));
        assert_eq!(s.attribute_count(), 2);
    }

    #[test]
    fn test_schedule_method_count() {
        let s = Schedule::new(ObisCode::new(0, 0, 12, 0, 0, 255));
        assert_eq!(s.method_count(), 0);
    }

    #[test]
    fn test_schedule_add_entry() {
        let mut s = Schedule::new(ObisCode::new(0, 0, 12, 0, 0, 255));
        s.add_entry(ScheduleEntry {
            index: 1,
            enable: true,
            script_selector: 1,
            start_time: [0x00, 0x00, 0x00, 0x00],
            valid_for_days: 0x7F, // all days
        });
        assert_eq!(s.entry_count(), 1);
    }

    #[test]
    fn test_schedule_remove_entry() {
        let mut s = Schedule::new(ObisCode::new(0, 0, 12, 0, 0, 255));
        s.add_entry(ScheduleEntry {
            index: 1,
            enable: true,
            script_selector: 1,
            start_time: [0x00, 0x00, 0x00, 0x00],
            valid_for_days: 0x7F,
        });
        let removed = s.remove_entry(0);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().index, 1);
        assert_eq!(s.entry_count(), 0);
    }

    #[test]
    fn test_schedule_attr1() {
        let s = Schedule::new(ObisCode::new(0, 0, 12, 0, 0, 255));
        let bytes = s.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_schedule_attr2() {
        let mut s = Schedule::new(ObisCode::new(0, 0, 12, 0, 0, 255));
        s.add_entry(ScheduleEntry {
            index: 1,
            enable: true,
            script_selector: 1,
            start_time: [0x00, 0x00, 0x00, 0x00],
            valid_for_days: 0x7F,
        });
        let bytes = s.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_schedule_entry_enable() {
        let mut s = Schedule::new(ObisCode::new(0, 0, 12, 0, 0, 255));
        s.add_entry(ScheduleEntry {
            index: 1,
            enable: false,
            script_selector: 1,
            start_time: [0x00, 0x00, 0x00, 0x00],
            valid_for_days: 0x7F,
        });
        assert!(!s.entries()[0].enable);
    }

    #[test]
    fn test_schedule_valid_for_days() {
        let mut s = Schedule::new(ObisCode::new(0, 0, 12, 0, 0, 255));
        s.add_entry(ScheduleEntry {
            index: 1,
            enable: true,
            script_selector: 1,
            start_time: [0x00, 0x00, 0x00, 0x00],
            valid_for_days: 0x1F, // Mon-Fri
        });
        assert_eq!(s.entries()[0].valid_for_days, 0x1F);
    }
}
