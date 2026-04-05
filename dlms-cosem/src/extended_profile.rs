//! Extended Profile - Enhanced profile for advanced metering
//!
//! Provides extended load profiling with additional parameters and flexibility
//! for sophisticated metering scenarios.

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Extended profile entry with additional metadata
#[derive(Debug, Clone)]
pub struct ExtendedProfileEntry {
    pub timestamp: DlmsData,
    pub value: DlmsData,
    pub quality_indicator: u8,
    pub status_flags: u16,
}

/// IC200 Extended Profile - Enhanced load profile object
pub struct ExtendedProfile {
    logical_name: ObisCode,
    entries: Vec<ExtendedProfileEntry>,
    capture_period: u16,
    max_entries: u16,
}

impl ExtendedProfile {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            entries: Vec::new(),
            capture_period: 900, // 15 minutes default
            max_entries: 1000,
        }
    }

    pub fn add_entry(&mut self, entry: ExtendedProfileEntry) {
        if self.entries.len() < self.max_entries as usize {
            self.entries.push(entry);
        }
    }

    pub fn entries(&self) -> &[ExtendedProfileEntry] {
        &self.entries
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

impl CosemObject for ExtendedProfile {
    fn class_id(&self) -> u16 {
        200
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        8
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
                            DlmsData::Enum(e.quality_indicator),
                            DlmsData::LongUnsigned(e.status_flags),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(entries)))
            }
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.entries.len() as u16,
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.capture_period))),
            5 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.max_entries))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::LongUnsigned(v) = decoded {
                    self.capture_period = v as u16;
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
    fn test_extended_profile_class_id() {
        let ep = ExtendedProfile::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        assert_eq!(ep.class_id(), 200);
    }

    #[test]
    fn test_extended_profile_new() {
        let ep = ExtendedProfile::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        assert_eq!(ep.entry_count(), 0);
    }

    #[test]
    fn test_extended_profile_add_entry() {
        let mut ep = ExtendedProfile::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        ep.add_entry(ExtendedProfileEntry {
            timestamp: DlmsData::DateTime([0; 12]),
            value: DlmsData::DoubleLong(100),
            quality_indicator: 0,
            status_flags: 0,
        });
        assert_eq!(ep.entry_count(), 1);
    }

    #[test]
    fn test_extended_profile_clear() {
        let mut ep = ExtendedProfile::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        ep.add_entry(ExtendedProfileEntry {
            timestamp: DlmsData::DateTime([0; 12]),
            value: DlmsData::DoubleLong(100),
            quality_indicator: 0,
            status_flags: 0,
        });
        ep.clear();
        assert_eq!(ep.entry_count(), 0);
    }

    #[test]
    fn test_extended_profile_attribute_count() {
        let ep = ExtendedProfile::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        assert_eq!(ep.attribute_count(), 8);
    }

    #[test]
    fn test_extended_profile_method_count() {
        let ep = ExtendedProfile::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        assert_eq!(ep.method_count(), 2);
    }

    #[test]
    fn test_extended_profile_attr1() {
        let ep = ExtendedProfile::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        let bytes = ep.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }
}
