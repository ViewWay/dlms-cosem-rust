//! IC007 Profile Generic (Load Profile)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Capture object in profile
#[derive(Debug, Clone)]
pub struct CaptureObject {
    pub class_id: u16,
    pub logical_name: ObisCode,
    pub attribute_index: u8,
    pub data_index: u8,
}

/// IC007 Profile Generic
pub struct ProfileGeneric {
    logical_name: ObisCode,
    capture_objects: Vec<CaptureObject>,
    period: u16,
    entries: Vec<Vec<DlmsData>>,
    #[allow(dead_code)]
    sort_method: u8,
}

impl ProfileGeneric {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            capture_objects: Vec::new(),
            period: 60,
            entries: Vec::new(),
            #[allow(dead_code)]
            sort_method: 0,
        }
    }

    pub fn add_capture_object(&mut self, class_id: u16, ln: ObisCode, attr_idx: u8, data_idx: u8) {
        self.capture_objects.push(CaptureObject {
            class_id,
            logical_name: ln,
            attribute_index: attr_idx,
            data_index: data_idx,
        });
    }

    pub fn add_entry(&mut self, entry: Vec<DlmsData>) {
        self.entries.push(entry);
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
    pub fn capture_object_count(&self) -> usize {
        self.capture_objects.len()
    }
}

impl CosemObject for ProfileGeneric {
    fn class_id(&self) -> u16 {
        7
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
                // Buffer: array of structures
                Some(dlms_axdr::encode(&DlmsData::Array(
                    self.entries
                        .iter()
                        .map(|e| DlmsData::Structure(e.clone()))
                        .collect(),
                )))
            }
            3 => Some(dlms_axdr::encode(&DlmsData::Structure(vec![
                DlmsData::Array(
                    self.capture_objects
                        .iter()
                        .map(|co| {
                            DlmsData::Structure(vec![
                                DlmsData::LongUnsigned(co.class_id),
                                DlmsData::OctetString(co.logical_name.to_bytes().to_vec()),
                                DlmsData::Integer(co.attribute_index as i8),
                                DlmsData::Unsigned(co.data_index),
                            ])
                        })
                        .collect(),
                ),
            ]))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.entries.len() as u16,
            ))),
            5 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.period as u16,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        Err(CosemObjectError::AttributeNotSupported(attr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_generic_class_id() {
        let pg = ProfileGeneric::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        assert_eq!(pg.class_id(), 7);
    }

    #[test]
    fn test_profile_generic_empty() {
        let pg = ProfileGeneric::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        assert_eq!(pg.entry_count(), 0);
    }

    #[test]
    fn test_profile_generic_add_capture_object() {
        let mut pg = ProfileGeneric::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        pg.add_capture_object(3, ObisCode::ACTIVE_POWER_L1, 2, 0);
        assert_eq!(pg.capture_object_count(), 1);
    }

    #[test]
    fn test_profile_generic_add_entry() {
        let mut pg = ProfileGeneric::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        pg.add_entry(vec![DlmsData::DateTime([0; 12]), DlmsData::DoubleLong(100)]);
        assert_eq!(pg.entry_count(), 1);
    }

    #[test]
    fn test_profile_generic_attr4() {
        let mut pg = ProfileGeneric::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        pg.add_entry(vec![DlmsData::None]);
        pg.add_entry(vec![DlmsData::None]);
        let bytes = pg.attribute_to_bytes(4).unwrap();
        // Attr 4 is LongUnsigned (entry count)
        assert_eq!(bytes[0], 0x12);
    }
}
