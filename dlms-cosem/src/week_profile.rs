//! IC021 Week Profile
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: table (array of structures: week_name, monday..sunday day_id)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct WeekProfileEntry {
    pub week_name: String,
    pub monday: u8,
    pub tuesday: u8,
    pub wednesday: u8,
    pub thursday: u8,
    pub friday: u8,
    pub saturday: u8,
    pub sunday: u8,
}

pub struct WeekProfile {
    logical_name: ObisCode,
    entries: Vec<WeekProfileEntry>,
}

impl WeekProfile {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(
        &mut self,
        name: &str,
        mon: u8,
        tue: u8,
        wed: u8,
        thu: u8,
        fri: u8,
        sat: u8,
        sun: u8,
    ) {
        self.entries.push(WeekProfileEntry {
            week_name: name.into(),
            monday: mon,
            tuesday: tue,
            wednesday: wed,
            thursday: thu,
            friday: fri,
            saturday: sat,
            sunday: sun,
        });
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

impl CosemObject for WeekProfile {
    fn class_id(&self) -> u16 {
        21
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
                            DlmsData::OctetString(e.week_name.as_bytes().to_vec()),
                            DlmsData::Unsigned(e.monday),
                            DlmsData::Unsigned(e.tuesday),
                            DlmsData::Unsigned(e.wednesday),
                            DlmsData::Unsigned(e.thursday),
                            DlmsData::Unsigned(e.friday),
                            DlmsData::Unsigned(e.saturday),
                            DlmsData::Unsigned(e.sunday),
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
    fn test_week_profile_class_id() {
        let w = WeekProfile::new(ObisCode::new(0, 0, 21, 0, 0, 255));
        assert_eq!(w.class_id(), 21);
    }

    #[test]
    fn test_week_profile_add_entry() {
        let mut w = WeekProfile::new(ObisCode::new(0, 0, 21, 0, 0, 255));
        w.add_entry("Normal", 1, 1, 1, 1, 1, 2, 2);
        assert_eq!(w.entry_count(), 1);
    }
}
