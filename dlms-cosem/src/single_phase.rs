//! IC031 Single Phase Electricity Meter
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: status (double-long-unsigned)
//! 3: error_flags (long-unsigned)
//! 4: alarm_flags (long-unsigned)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct SinglePhase {
    logical_name: ObisCode,
    status: u32,
    error_flags: u16,
    alarm_flags: u16,
}

impl SinglePhase {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            status: 0,
            error_flags: 0,
            alarm_flags: 0,
        }
    }

    pub fn status(&self) -> u32 {
        self.status
    }
    pub fn set_status(&mut self, status: u32) {
        self.status = status;
    }
}

impl CosemObject for SinglePhase {
    fn class_id(&self) -> u16 {
        31
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
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.status,
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.error_flags))),
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.alarm_flags))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::DoubleLongUnsigned(v) = decoded {
                    self.status = v;
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
    fn test_single_phase_class_id() {
        let s = SinglePhase::new(ObisCode::new(0, 0, 31, 0, 0, 255));
        assert_eq!(s.class_id(), 31);
    }

    #[test]
    fn test_single_phase_status() {
        let mut s = SinglePhase::new(ObisCode::new(0, 0, 31, 0, 0, 255));
        s.set_status(0x00010000);
        assert_eq!(s.status(), 0x00010000);
    }

    #[test]
    fn test_single_phase_attr2_roundtrip() {
        let mut s = SinglePhase::new(ObisCode::new(0, 0, 31, 0, 0, 255));
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(0xABCD1234));
        s.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(s.status(), 0xABCD1234);
    }
}
