//! IC015 Push Setup
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: push_object_list (array of structures)
//! 3: service (long-unsigned)
//! 4: destination (octet-string)
//! 5: communication_window (structure: start_time, stop_time)
//! 6: randomisation_start_interval (long-unsigned)
//! 7: number_of_retries (unsigned)
//! 8: repetition_delay (long-unsigned)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct PushObject {
    pub class_id: u16,
    pub logical_name: ObisCode,
    pub attribute: u8,
}

pub struct PushSetup {
    logical_name: ObisCode,
    objects: Vec<PushObject>,
    service: u16,
    destination: Vec<u8>,
    retries: u8,
}

impl PushSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            objects: Vec::new(),
            service: 0,
            destination: Vec::new(),
            retries: 3,
        }
    }

    pub fn add_object(&mut self, class_id: u16, ln: ObisCode, attr: u8) {
        self.objects.push(PushObject {
            class_id,
            logical_name: ln,
            attribute: attr,
        });
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
    pub fn service(&self) -> u16 {
        self.service
    }
    pub fn set_service(&mut self, s: u16) {
        self.service = s;
    }
}

impl CosemObject for PushSetup {
    fn class_id(&self) -> u16 {
        40
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        8
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
                self.objects
                    .iter()
                    .map(|o| {
                        DlmsData::Structure(vec![
                            DlmsData::LongUnsigned(o.class_id),
                            DlmsData::OctetString(o.logical_name.to_bytes().to_vec()),
                            DlmsData::Integer(o.attribute as i8),
                            DlmsData::Unsigned(0),
                        ])
                    })
                    .collect(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.service))),
            4 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.destination.clone(),
            ))),
            7 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.retries))),
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
    fn test_push_setup_class_id() {
        let ps = PushSetup::new(ObisCode::new(0, 0, 15, 0, 0, 255));
        assert_eq!(ps.class_id(), 40);
    }

    #[test]
    fn test_push_setup_add_object() {
        let mut ps = PushSetup::new(ObisCode::new(0, 0, 15, 0, 0, 255));
        ps.add_object(8, ObisCode::CLOCK, 2);
        assert_eq!(ps.object_count(), 1);
    }

    #[test]
    fn test_push_setup_attribute_count() {
        let ps = PushSetup::new(ObisCode::new(0, 0, 15, 0, 0, 255));
        assert_eq!(ps.attribute_count(), 8);
    }

    #[test]
    fn test_push_setup_method_count() {
        let ps = PushSetup::new(ObisCode::new(0, 0, 15, 0, 0, 255));
        assert_eq!(ps.method_count(), 0);
    }

    #[test]
    fn test_push_setup_attr1() {
        let ps = PushSetup::new(ObisCode::new(0, 0, 15, 0, 0, 255));
        let bytes = ps.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_push_setup_attr2_objects() {
        let mut ps = PushSetup::new(ObisCode::new(0, 0, 15, 0, 0, 255));
        ps.add_object(8, ObisCode::CLOCK, 2);
        ps.add_object(3, ObisCode::ACTIVE_POWER_L1, 2);
        let bytes = ps.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_push_setup_service() {
        let mut ps = PushSetup::new(ObisCode::new(0, 0, 15, 0, 0, 255));
        assert_eq!(ps.service(), 0);
        ps.set_service(1234);
        assert_eq!(ps.service(), 1234);
    }

    #[test]
    fn test_push_setup_multiple_objects() {
        let mut ps = PushSetup::new(ObisCode::new(0, 0, 15, 0, 0, 255));
        ps.add_object(8, ObisCode::CLOCK, 2);
        ps.add_object(1, ObisCode::new(1, 0, 0, 9, 0, 255), 2);
        ps.add_object(7, ObisCode::new(1, 0, 99, 1, 0, 255), 2);
        assert_eq!(ps.object_count(), 3);
    }
}
