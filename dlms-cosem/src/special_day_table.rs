//! IC011 Special Day Table
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: table (array of structures: date, day_id)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct SpecialDayEntry {
    pub date: [u8; 5], // DLMS Date
    pub day_id: u8,
}

pub struct SpecialDayTable {
    logical_name: ObisCode,
    entries: Vec<SpecialDayEntry>,
}

impl SpecialDayTable {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, date: [u8; 5], day_id: u8) {
        self.entries.push(SpecialDayEntry { date, day_id });
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

impl CosemObject for SpecialDayTable {
    fn class_id(&self) -> u16 {
        11
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
                            DlmsData::Date(e.date),
                            DlmsData::Unsigned(e.day_id),
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
    fn test_special_day_table_class_id() {
        let t = SpecialDayTable::new(ObisCode::new(0, 0, 11, 0, 0, 255));
        assert_eq!(t.class_id(), 11);
    }

    #[test]
    fn test_special_day_table_add_entry() {
        let mut t = SpecialDayTable::new(ObisCode::new(0, 0, 11, 0, 0, 255));
        t.add_entry([0x07, 0xE8, 0x01, 0x01, 0xFF], 1);
        assert_eq!(t.entry_count(), 1);
    }

    #[test]
    fn test_special_day_table_attr2() {
        let mut t = SpecialDayTable::new(ObisCode::new(0, 0, 11, 0, 0, 255));
        t.add_entry([0x07, 0xE8, 0x01, 0x01, 0xFF], 1);
        let bytes = t.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }
}
