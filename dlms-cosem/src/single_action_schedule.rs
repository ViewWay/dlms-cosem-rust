//! IC22 Single Action Schedule
//! Blue Book Ed16: class_id=22, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Schedule entry
#[derive(Debug, Clone)]
pub struct ActionScheduleEntry {
    pub executed_script_logical_name: ObisCode,
    pub executed_at: DlmsData,
}

/// Single Action Schedule - schedules single actions
pub struct SingleActionSchedule {
    logical_name: ObisCode,
    entries: Vec<ActionScheduleEntry>,
}

impl SingleActionSchedule {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            entries: vec![],
        }
    }

    pub fn entries(&self) -> &[ActionScheduleEntry] {
        &self.entries
    }
    pub fn add_entry(&mut self, entry: ActionScheduleEntry) {
        self.entries.push(entry);
    }
}

impl CosemObject for SingleActionSchedule {
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
            2 => {
                let list: Vec<DlmsData> = self
                    .entries
                    .iter()
                    .map(|e| {
                        DlmsData::Structure(vec![
                            DlmsData::OctetString(
                                e.executed_script_logical_name.to_bytes().to_vec(),
                            ),
                            e.executed_at.clone(),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(list)))
            }
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
    fn test_single_action_schedule_new() {
        let s = SingleActionSchedule::new(ObisCode::CLOCK);
        assert_eq!(s.class_id(), 22);
    }

    #[test]
    fn test_single_action_schedule_add() {
        let mut s = SingleActionSchedule::new(ObisCode::CLOCK);
        s.add_entry(ActionScheduleEntry {
            executed_script_logical_name: ObisCode::CLOCK,
            executed_at: DlmsData::OctetString(vec![0; 12]),
        });
        assert_eq!(s.entries().len(), 1);
    }

    #[test]
    fn test_single_action_schedule_attribute_count() {
        let s = SingleActionSchedule::new(ObisCode::CLOCK);
        assert_eq!(s.attribute_count(), 3);
    }

    #[test]
    fn test_single_action_schedule_method_count() {
        let s = SingleActionSchedule::new(ObisCode::CLOCK);
        assert_eq!(s.method_count(), 0);
    }

    #[test]
    fn test_single_action_schedule_attr1() {
        let s = SingleActionSchedule::new(ObisCode::CLOCK);
        let bytes = s.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_single_action_schedule_attr2() {
        let mut s = SingleActionSchedule::new(ObisCode::CLOCK);
        s.add_entry(ActionScheduleEntry {
            executed_script_logical_name: ObisCode::CLOCK,
            executed_at: DlmsData::DateTime([0; 12]),
        });
        let bytes = s.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_single_action_schedule_multiple_entries() {
        let mut s = SingleActionSchedule::new(ObisCode::CLOCK);
        s.add_entry(ActionScheduleEntry {
            executed_script_logical_name: ObisCode::CLOCK,
            executed_at: DlmsData::DateTime([0; 12]),
        });
        s.add_entry(ActionScheduleEntry {
            executed_script_logical_name: ObisCode::new(0, 0, 1, 0, 0, 255),
            executed_at: DlmsData::DateTime([0; 12]),
        });
        assert_eq!(s.entries().len(), 2);
    }
}
