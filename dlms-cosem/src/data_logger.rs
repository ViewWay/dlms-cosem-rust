//! Data Logger - Logs measurement data over time
//!
//! Captures and stores measurement data with timestamps
//! for historical analysis and reporting.

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Data log entry
#[derive(Debug, Clone)]
pub struct DataLogEntry {
    pub timestamp: DlmsData,
    pub value: DlmsData,
    pub quality: u8,
}

/// IC207 Data Logger - Logs measurement data
pub struct DataLogger {
    logical_name: ObisCode,
    entries: Vec<DataLogEntry>,
    max_entries: u16,
    capture_interval: u16,
}

impl DataLogger {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            entries: Vec::new(),
            max_entries: 1000,
            capture_interval: 60,
        }
    }

    pub fn with_capacity(logical_name: ObisCode, max_entries: u16) -> Self {
        Self {
            logical_name,
            entries: Vec::new(),
            max_entries,
            capture_interval: 60,
        }
    }

    pub fn log(&mut self, value: DlmsData) -> bool {
        if self.entries.len() >= self.max_entries as usize {
            return false;
        }
        self.entries.push(DataLogEntry {
            timestamp: DlmsData::DateTime([0; 12]),
            value,
            quality: 0,
        });
        true
    }

    pub fn get_entry(&self, index: usize) -> Option<&DataLogEntry> {
        self.entries.get(index)
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn capture_interval(&self) -> u16 {
        self.capture_interval
    }

    pub fn set_capture_interval(&mut self, interval: u16) {
        self.capture_interval = interval;
    }

    pub fn get_latest(&self) -> Option<&DataLogEntry> {
        self.entries.last()
    }
}

impl CosemObject for DataLogger {
    fn class_id(&self) -> u16 {
        207
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        6
    }

    fn method_count(&self) -> u8 {
        2
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
                            e.timestamp.clone(),
                            e.value.clone(),
                            DlmsData::Unsigned(e.quality),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(entries)))
            }
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.entries.len() as u16,
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.max_entries))),
            5 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.capture_interval))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            5 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::LongUnsigned(v) = decoded {
                    self.capture_interval = v as u16;
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
    fn test_data_logger_class_id() {
        let dl = DataLogger::new(ObisCode::new(0, 0, 207, 0, 0, 255));
        assert_eq!(dl.class_id(), 207);
    }

    #[test]
    fn test_data_logger_new() {
        let dl = DataLogger::new(ObisCode::new(0, 0, 207, 0, 0, 255));
        assert_eq!(dl.entry_count(), 0);
        assert_eq!(dl.capture_interval(), 60);
    }

    #[test]
    fn test_data_logger_log() {
        let mut dl = DataLogger::new(ObisCode::new(0, 0, 207, 0, 0, 255));
        let result = dl.log(DlmsData::DoubleLong(100));
        assert!(result);
        assert_eq!(dl.entry_count(), 1);
    }

    #[test]
    fn test_data_logger_get_entry() {
        let mut dl = DataLogger::new(ObisCode::new(0, 0, 207, 0, 0, 255));
        dl.log(DlmsData::DoubleLong(100));
        let entry = dl.get_entry(0);
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().value.as_i32(), Some(100));
    }

    #[test]
    fn test_data_logger_clear() {
        let mut dl = DataLogger::new(ObisCode::new(0, 0, 207, 0, 0, 255));
        dl.log(DlmsData::DoubleLong(100));
        dl.clear();
        assert_eq!(dl.entry_count(), 0);
    }

    #[test]
    fn test_data_logger_capture_interval() {
        let mut dl = DataLogger::new(ObisCode::new(0, 0, 207, 0, 0, 255));
        dl.set_capture_interval(300);
        assert_eq!(dl.capture_interval(), 300);
    }

    #[test]
    fn test_data_logger_get_latest() {
        let mut dl = DataLogger::new(ObisCode::new(0, 0, 207, 0, 0, 255));
        dl.log(DlmsData::DoubleLong(100));
        dl.log(DlmsData::DoubleLong(200));
        let latest = dl.get_latest();
        assert!(latest.is_some());
        assert_eq!(latest.unwrap().value.as_i32(), Some(200));
    }

    #[test]
    fn test_data_logger_with_capacity() {
        let dl = DataLogger::with_capacity(ObisCode::new(0, 0, 207, 0, 0, 255), 500);
        assert_eq!(dl.entry_count(), 0);
    }

    #[test]
    fn test_data_logger_attribute_count() {
        let dl = DataLogger::new(ObisCode::new(0, 0, 207, 0, 0, 255));
        assert_eq!(dl.attribute_count(), 6);
    }

    #[test]
    fn test_data_logger_method_count() {
        let dl = DataLogger::new(ObisCode::new(0, 0, 207, 0, 0, 255));
        assert_eq!(dl.method_count(), 2);
    }

    #[test]
    fn test_data_logger_max_entries() {
        let mut dl = DataLogger::with_capacity(ObisCode::new(0, 0, 207, 0, 0, 255), 5);
        for i in 0..10 {
            dl.log(DlmsData::DoubleLong(i));
        }
        assert_eq!(dl.entry_count(), 5); // max_entries
    }
}
