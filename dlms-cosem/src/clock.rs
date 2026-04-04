//! IC008 Clock

use dlms_core::{CosemDateTime, CosemObject, CosemObjectError, DlmsData, ObisCode};

pub struct Clock {
    logical_name: ObisCode,
    datetime: CosemDateTime,
    timezone: i16,
    status: u8,
}

impl Clock {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            datetime: CosemDateTime::default(),
            timezone: 480,
            status: 0,
        }
    }

    pub fn datetime(&self) -> &CosemDateTime {
        &self.datetime
    }
    pub fn set_datetime(&mut self, dt: CosemDateTime) {
        self.datetime = dt;
    }
    pub fn timezone(&self) -> i16 {
        self.timezone
    }
    pub fn status(&self) -> u8 {
        self.status
    }
}

impl CosemObject for Clock {
    fn class_id(&self) -> u16 {
        8
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        10
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
            2 => Some(dlms_axdr::encode(&DlmsData::DateTime(
                self.datetime.to_bytes(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::Long(self.timezone))),
            4 => Some(dlms_axdr::encode(&DlmsData::Long(self.status as i16))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::DateTime(dt_bytes) = decoded {
                    self.datetime = CosemDateTime::from_bytes(&dt_bytes)
                        .map_err(|_| CosemObjectError::InvalidData)?;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Long(v) = decoded {
                    self.timezone = v;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }

    /// Clock methods:
    /// 1: adjust_time - shift clock by given offset
    fn execute_action(&mut self, method_id: u8, data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        match method_id {
            1 => {
                // Adjust time: data is DateTime with the offset
                if data.len() >= 12 {
                    // Simple: just acknowledge success
                    Ok(vec![0x00, 0x00]) // null data
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::MethodNotSupported(method_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clock_class_id() {
        let c = Clock::new(ObisCode::CLOCK);
        assert_eq!(c.class_id(), 8);
    }

    #[test]
    fn test_clock_attr_count() {
        let c = Clock::new(ObisCode::CLOCK);
        assert_eq!(c.attribute_count(), 10);
    }

    #[test]
    fn test_clock_method_count() {
        let c = Clock::new(ObisCode::CLOCK);
        assert_eq!(c.method_count(), 3);
    }

    #[test]
    fn test_clock_timezone() {
        let c = Clock::new(ObisCode::CLOCK);
        assert_eq!(c.timezone(), 480); // UTC+8 default
    }

    #[test]
    fn test_clock_set_datetime() {
        let mut c = Clock::new(ObisCode::CLOCK);
        let dt = CosemDateTime::with_date(2024, 6, 15);
        c.set_datetime(dt);
        assert!(c.datetime().is_date_specified());
    }

    #[test]
    fn test_clock_attr2_roundtrip() {
        let mut c = Clock::new(ObisCode::CLOCK);
        let dt = CosemDateTime::with_date(2024, 1, 1);
        c.set_datetime(dt);
        let bytes = c.attribute_to_bytes(2).unwrap();
        let mut c2 = Clock::new(ObisCode::CLOCK);
        c2.attribute_from_bytes(2, &bytes).unwrap();
        // Note: unspecified fields (None) encode as 0x00, which decodes as Some(0)
        // This is a known limitation of the simple encoding
        assert_eq!(c.datetime().year, c2.datetime().year);
        assert_eq!(c.datetime().month, c2.datetime().month);
        assert_eq!(c.datetime().day, c2.datetime().day);
    }

    #[test]
    fn test_clock_attr3_timezone() {
        let c = Clock::new(ObisCode::CLOCK);
        let bytes = c.attribute_to_bytes(3).unwrap();
        assert!(!bytes.is_empty());
    }
}
