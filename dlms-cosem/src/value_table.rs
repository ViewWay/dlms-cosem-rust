//! IC029 Value Table
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: values (array of values)
//! 3: value_descriptors (array of descriptors)
//!
//! Methods:
//! None typically

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct ValueEntry {
    pub index: u16,
    pub value: DlmsData,
    pub timestamp: Option<[u8; 12]>, // DLMS datetime
}

#[derive(Debug, Clone)]
pub struct ValueDescriptor {
    pub index: u16,
    pub description: String,
    pub unit: u8,
    pub scaler: i8,
}

pub struct ValueTable {
    logical_name: ObisCode,
    values: Vec<ValueEntry>,
    descriptors: Vec<ValueDescriptor>,
}

impl ValueTable {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            values: Vec::new(),
            descriptors: Vec::new(),
        }
    }

    pub fn values(&self) -> &[ValueEntry] {
        &self.values
    }

    pub fn descriptors(&self) -> &[ValueDescriptor] {
        &self.descriptors
    }

    pub fn add_value(&mut self, entry: ValueEntry) {
        self.values.push(entry);
    }

    pub fn add_descriptor(&mut self, descriptor: ValueDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub fn remove_value(&mut self, index: usize) -> Option<ValueEntry> {
        if index < self.values.len() {
            Some(self.values.remove(index))
        } else {
            None
        }
    }

    pub fn value_count(&self) -> usize {
        self.values.len()
    }

    pub fn descriptor_count(&self) -> usize {
        self.descriptors.len()
    }
}

impl CosemObject for ValueTable {
    fn class_id(&self) -> u16 {
        29
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
            2 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.values
                    .iter()
                    .map(|e| {
                        let timestamp = match &e.timestamp {
                            Some(dt) => DlmsData::DateTime(*dt),
                            None => DlmsData::None,
                        };
                        DlmsData::Structure(vec![
                            DlmsData::Long(e.index as i16),
                            e.value.clone(),
                            timestamp,
                        ])
                    })
                    .collect(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.descriptors
                    .iter()
                    .map(|d| {
                        DlmsData::Structure(vec![
                            DlmsData::Long(d.index as i16),
                            DlmsData::VisibleString(d.description.clone()),
                            DlmsData::Unsigned(d.unit),
                            DlmsData::Enum(d.scaler as u8),
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
    fn test_value_table_class_id() {
        let vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        assert_eq!(vt.class_id(), 29);
    }

    #[test]
    fn test_value_table_attribute_count() {
        let vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        assert_eq!(vt.attribute_count(), 3);
    }

    #[test]
    fn test_value_table_method_count() {
        let vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        assert_eq!(vt.method_count(), 0);
    }

    #[test]
    fn test_value_table_add_value() {
        let mut vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        vt.add_value(ValueEntry {
            index: 1,
            value: DlmsData::DoubleLong(100),
            timestamp: None,
        });
        assert_eq!(vt.value_count(), 1);
    }

    #[test]
    fn test_value_table_add_descriptor() {
        let mut vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        vt.add_descriptor(ValueDescriptor {
            index: 1,
            description: "Voltage".to_string(),
            unit: 27, // V
            scaler: 0,
        });
        assert_eq!(vt.descriptor_count(), 1);
    }

    #[test]
    fn test_value_table_remove_value() {
        let mut vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        vt.add_value(ValueEntry {
            index: 1,
            value: DlmsData::DoubleLong(100),
            timestamp: None,
        });
        let removed = vt.remove_value(0);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().index, 1);
        assert_eq!(vt.value_count(), 0);
    }

    #[test]
    fn test_value_table_attr1() {
        let vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        let bytes = vt.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_value_table_attr2() {
        let mut vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        vt.add_value(ValueEntry {
            index: 1,
            value: DlmsData::DoubleLong(100),
            timestamp: None,
        });
        let bytes = vt.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_value_table_attr3() {
        let mut vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        vt.add_descriptor(ValueDescriptor {
            index: 1,
            description: "Voltage".to_string(),
            unit: 27,
            scaler: 0,
        });
        let bytes = vt.attribute_to_bytes(3).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_value_entry_with_timestamp() {
        let mut vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        let ts = [0x07, 0xE8, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x80, 0x00, 0x00];
        vt.add_value(ValueEntry {
            index: 1,
            value: DlmsData::DoubleLong(100),
            timestamp: Some(ts),
        });
        assert_eq!(vt.value_count(), 1);
        assert!(vt.values()[0].timestamp.is_some());
    }

    #[test]
    fn test_value_descriptor_unit_scaler() {
        let mut vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        vt.add_descriptor(ValueDescriptor {
            index: 1,
            description: "Power".to_string(),
            unit: 27,
            scaler: -1,
        });
        assert_eq!(vt.descriptors()[0].unit, 27);
        assert_eq!(vt.descriptors()[0].scaler, -1);
    }

    #[test]
    fn test_value_table_multiple_values() {
        let mut vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        vt.add_value(ValueEntry {
            index: 1,
            value: DlmsData::DoubleLong(100),
            timestamp: None,
        });
        vt.add_value(ValueEntry {
            index: 2,
            value: DlmsData::DoubleLong(200),
            timestamp: None,
        });
        vt.add_value(ValueEntry {
            index: 3,
            value: DlmsData::DoubleLong(300),
            timestamp: None,
        });
        assert_eq!(vt.value_count(), 3);
        assert_eq!(vt.values()[2].index, 3);
    }

    #[test]
    fn test_value_descriptor_description() {
        let mut vt = ValueTable::new(ObisCode::new(0, 0, 29, 0, 0, 255));
        vt.add_descriptor(ValueDescriptor {
            index: 1,
            description: "Total Energy".to_string(),
            unit: 30, // Wh
            scaler: 0,
        });
        assert_eq!(vt.descriptors()[0].description, "Total Energy");
    }
}
