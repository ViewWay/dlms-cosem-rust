//! Event Logger - Records events in the metering system
//!
//! Captures and stores events with timestamps for audit trails
//! and system diagnostics.

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Event log entry
#[derive(Debug, Clone)]
pub struct EventLogEntry {
    pub index: u16,
    pub event_code: u16,
    pub event_timestamp: DlmsData,
    pub event_data: Vec<u8>,
}

/// IC202 Event Logger - Records system events
pub struct EventLogger {
    logical_name: ObisCode,
    entries: Vec<EventLogEntry>,
    max_entries: u16,
    enabled: bool,
}

impl EventLogger {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            entries: Vec::new(),
            max_entries: 100,
            enabled: true,
        }
    }

    pub fn log_event(&mut self, event_code: u16, data: Vec<u8>) -> u16 {
        let index = self.entries.len() as u16;
        self.entries.push(EventLogEntry {
            index,
            event_code,
            event_timestamp: DlmsData::DateTime([0; 12]),
            event_data: data,
        });

        // Keep only max_entries
        if self.entries.len() > self.max_entries as usize {
            self.entries.remove(0);
        }

        self.entries.last().map(|e| e.index).unwrap_or(0)
    }

    pub fn get_entry(&self, index: u16) -> Option<&EventLogEntry> {
        self.entries.iter().find(|e| e.index == index)
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl CosemObject for EventLogger {
    fn class_id(&self) -> u16 {
        202
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        7
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
                            DlmsData::LongUnsigned(e.index),
                            DlmsData::LongUnsigned(e.event_code),
                            e.event_timestamp.clone(),
                            DlmsData::OctetString(e.event_data.clone()),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(entries)))
            }
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.entries.len() as u16,
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.max_entries))),
            5 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.enabled))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            5 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Boolean(v) = decoded {
                    self.enabled = v;
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
    fn test_event_logger_class_id() {
        let el = EventLogger::new(ObisCode::new(0, 0, 202, 0, 0, 255));
        assert_eq!(el.class_id(), 202);
    }

    #[test]
    fn test_event_logger_new() {
        let el = EventLogger::new(ObisCode::new(0, 0, 202, 0, 0, 255));
        assert_eq!(el.entry_count(), 0);
        assert!(el.is_enabled());
    }

    #[test]
    fn test_event_logger_log() {
        let mut el = EventLogger::new(ObisCode::new(0, 0, 202, 0, 0, 255));
        let index = el.log_event(100, vec![1, 2, 3]);
        assert_eq!(el.entry_count(), 1);
        assert_eq!(index, 0);
    }

    #[test]
    fn test_event_logger_get_entry() {
        let mut el = EventLogger::new(ObisCode::new(0, 0, 202, 0, 0, 255));
        el.log_event(100, vec![1, 2, 3]);
        let entry = el.get_entry(0);
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().event_code, 100);
    }

    #[test]
    fn test_event_logger_clear() {
        let mut el = EventLogger::new(ObisCode::new(0, 0, 202, 0, 0, 255));
        el.log_event(100, vec![1, 2, 3]);
        el.clear();
        assert_eq!(el.entry_count(), 0);
    }

    #[test]
    fn test_event_logger_enabled() {
        let mut el = EventLogger::new(ObisCode::new(0, 0, 202, 0, 0, 255));
        assert!(el.is_enabled());
        el.set_enabled(false);
        assert!(!el.is_enabled());
    }

    #[test]
    fn test_event_logger_multiple_events() {
        let mut el = EventLogger::new(ObisCode::new(0, 0, 202, 0, 0, 255));
        el.log_event(100, vec![1, 2, 3]);
        el.log_event(200, vec![4, 5, 6]);
        el.log_event(300, vec![7, 8, 9]);
        assert_eq!(el.entry_count(), 3);
    }

    #[test]
    fn test_event_logger_max_entries() {
        let mut el = EventLogger::new(ObisCode::new(0, 0, 202, 0, 0, 255));
        for i in 0..150 {
            el.log_event(i, vec![i as u8]);
        }
        assert_eq!(el.entry_count(), 100); // max_entries
    }
}
