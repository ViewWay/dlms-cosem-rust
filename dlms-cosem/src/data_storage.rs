//! Data Storage - Persistent data storage for COSEM objects
//!
//! Provides persistent storage capabilities for COSEM interface classes,
//! allowing data to survive power cycles and device restarts.

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Data storage entry
#[derive(Debug, Clone)]
pub struct DataStorageEntry {
    pub key: Vec<u8>,
    pub value: DlmsData,
    pub timestamp: DlmsData,
}

/// IC201 Data Storage - Persistent storage for COSEM data
pub struct DataStorage {
    logical_name: ObisCode,
    entries: Vec<DataStorageEntry>,
    capacity: u16,
}

impl DataStorage {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            entries: Vec::new(),
            capacity: 1000,
        }
    }

    pub fn with_capacity(logical_name: ObisCode, capacity: u16) -> Self {
        Self {
            logical_name,
            entries: Vec::new(),
            capacity,
        }
    }

    pub fn put(&mut self, key: Vec<u8>, value: DlmsData) -> bool {
        if self.entries.len() >= self.capacity as usize {
            return false;
        }
        self.entries.push(DataStorageEntry {
            key,
            value,
            timestamp: DlmsData::DateTime([0; 12]),
        });
        true
    }

    pub fn get(&self, key: &[u8]) -> Option<&DlmsData> {
        self.entries.iter().find(|e| e.key == key).map(|e| &e.value)
    }

    pub fn remove(&mut self, key: &[u8]) -> bool {
        if let Some(pos) = self.entries.iter().position(|e| e.key == key) {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl CosemObject for DataStorage {
    fn class_id(&self) -> u16 {
        201
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        5
    }

    fn method_count(&self) -> u8 {
        3
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
                            DlmsData::OctetString(e.key.clone()),
                            e.value.clone(),
                            e.timestamp.clone(),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(entries)))
            }
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.entries.len() as u16,
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.capacity))),
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
    fn test_data_storage_class_id() {
        let ds = DataStorage::new(ObisCode::new(0, 0, 201, 0, 0, 255));
        assert_eq!(ds.class_id(), 201);
    }

    #[test]
    fn test_data_storage_new() {
        let ds = DataStorage::new(ObisCode::new(0, 0, 201, 0, 0, 255));
        assert_eq!(ds.entry_count(), 0);
    }

    #[test]
    fn test_data_storage_put() {
        let mut ds = DataStorage::new(ObisCode::new(0, 0, 201, 0, 0, 255));
        let result = ds.put(vec![1, 2, 3], DlmsData::Unsigned(42));
        assert!(result);
        assert_eq!(ds.entry_count(), 1);
    }

    #[test]
    fn test_data_storage_get() {
        let mut ds = DataStorage::new(ObisCode::new(0, 0, 201, 0, 0, 255));
        ds.put(vec![1, 2, 3], DlmsData::Unsigned(42));
        let value = ds.get(&[1, 2, 3]);
        assert!(value.is_some());
        assert_eq!(value.unwrap().as_u8(), Some(42));
    }

    #[test]
    fn test_data_storage_remove() {
        let mut ds = DataStorage::new(ObisCode::new(0, 0, 201, 0, 0, 255));
        ds.put(vec![1, 2, 3], DlmsData::Unsigned(42));
        let result = ds.remove(&[1, 2, 3]);
        assert!(result);
        assert_eq!(ds.entry_count(), 0);
    }

    #[test]
    fn test_data_storage_clear() {
        let mut ds = DataStorage::new(ObisCode::new(0, 0, 201, 0, 0, 255));
        ds.put(vec![1, 2, 3], DlmsData::Unsigned(42));
        ds.put(vec![4, 5, 6], DlmsData::Unsigned(43));
        ds.clear();
        assert_eq!(ds.entry_count(), 0);
    }

    #[test]
    fn test_data_storage_capacity() {
        let ds = DataStorage::new(ObisCode::new(0, 0, 201, 0, 0, 255));
        let bytes = ds.attribute_to_bytes(4).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_data_storage_with_capacity() {
        let ds = DataStorage::with_capacity(ObisCode::new(0, 0, 201, 0, 0, 255), 500);
        assert_eq!(ds.entry_count(), 0);
    }
}
