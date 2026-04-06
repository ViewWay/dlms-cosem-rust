//! IC68 Arbitrator
//! Blue Book Ed16: class_id=68, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Arbitrator - manages arbitration between competing actions
pub struct Arbitrator {
    logical_name: ObisCode,
    arbitrator_list: Vec<DlmsData>,
    weights: Vec<DlmsData>,
    longest_backup_time: u32,
}

impl Arbitrator {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            arbitrator_list: vec![],
            weights: vec![],
            longest_backup_time: 3600,
        }
    }

    pub fn longest_backup_time(&self) -> u32 {
        self.longest_backup_time
    }
    pub fn set_longest_backup_time(&mut self, t: u32) {
        self.longest_backup_time = t;
    }
}

impl CosemObject for Arbitrator {
    fn class_id(&self) -> u16 {
        68
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
            2 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.arbitrator_list.clone(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::Array(self.weights.clone()))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.longest_backup_time,
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
    fn test_arbitrator_new() {
        let a = Arbitrator::new(ObisCode::CLOCK);
        assert_eq!(a.class_id(), 68);
    }

    #[test]
    fn test_arbitrator_backup_time() {
        let mut a = Arbitrator::new(ObisCode::CLOCK);
        a.set_longest_backup_time(7200);
        assert_eq!(a.longest_backup_time(), 7200);
    }

    #[test]
    fn test_arbitrator_attr1() {
        let a = Arbitrator::new(ObisCode::CLOCK);
        let bytes = a.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }
}
