//! IC092 Utility Sub-Schedule

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Utility Sub-Schedule - sub-scheduling for utility operations
pub struct UtilitySubSchedule {
    logical_name: ObisCode,
    entries: Vec<SubScheduleEntry>,
    active: bool,
}

/// A sub-schedule entry
#[derive(Debug, Clone)]
pub struct SubScheduleEntry {
    pub index: u16,
    pub start_time: u32,
    pub end_time: u32,
    pub value: DlmsData,
}

impl UtilitySubSchedule {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            entries: vec![],
            active: false,
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    pub fn entries(&self) -> &[SubScheduleEntry] {
        &self.entries
    }

    pub fn add_entry(&mut self, entry: SubScheduleEntry) {
        self.entries.push(entry);
    }
}

impl CosemObject for UtilitySubSchedule {
    fn class_id(&self) -> u16 {
        217
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
            2 => {
                let entries: Vec<DlmsData> = self
                    .entries
                    .iter()
                    .map(|e| {
                        DlmsData::Structure(vec![
                            DlmsData::Unsigned(e.index as u8),
                            DlmsData::DoubleLongUnsigned(e.start_time),
                            DlmsData::DoubleLongUnsigned(e.end_time),
                            e.value.clone(),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(entries)))
            }
            3 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.active))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(b) = decoded.as_bool() {
                    self.active = b;
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
    fn test_utility_sub_schedule_new() {
        let uss = UtilitySubSchedule::new(ObisCode::CLOCK);
        assert_eq!(uss.class_id(), 217);
    }

    #[test]
    fn test_utility_sub_schedule_entries() {
        let mut uss = UtilitySubSchedule::new(ObisCode::CLOCK);
        uss.add_entry(SubScheduleEntry {
            index: 1,
            start_time: 3600,
            end_time: 7200,
            value: DlmsData::Unsigned(1),
        });
        assert_eq!(uss.entries().len(), 1);
    }

    #[test]
    fn test_utility_sub_schedule_roundtrip() {
        let mut uss = UtilitySubSchedule::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Boolean(true));
        uss.attribute_from_bytes(3, &bytes).unwrap();
        assert!(uss.active());
    }
}
