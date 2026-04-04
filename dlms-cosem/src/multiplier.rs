//! IC090 Multiplier

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Multiplier - applies a multiplier to register values
pub struct Multiplier {
    logical_name: ObisCode,
    multiplier: f64,
    offset: f64,
}

impl Multiplier {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            multiplier: 1.0,
            offset: 0.0,
        }
    }

    pub fn with_params(logical_name: ObisCode, multiplier: f64, offset: f64) -> Self {
        Self {
            logical_name,
            multiplier,
            offset,
        }
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }

    pub fn set_multiplier(&mut self, mult: f64) {
        self.multiplier = mult;
    }

    pub fn offset(&self) -> f64 {
        self.offset
    }

    pub fn set_offset(&mut self, offset: f64) {
        self.offset = offset;
    }

    /// Apply multiplier and offset to a value
    pub fn apply(&self, value: f64) -> f64 {
        value * self.multiplier + self.offset
    }
}

impl CosemObject for Multiplier {
    fn class_id(&self) -> u16 {
        90
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
            2 => Some(dlms_axdr::encode(&DlmsData::Double(self.multiplier))),
            3 => Some(dlms_axdr::encode(&DlmsData::Double(self.offset))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 | 3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_f64() {
                    if attr == 2 {
                        self.multiplier = v;
                    } else {
                        self.offset = v;
                    }
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
    fn test_multiplier_new() {
        let m = Multiplier::new(ObisCode::CLOCK);
        assert_eq!(m.class_id(), 90);
        assert_eq!(m.multiplier(), 1.0);
    }

    #[test]
    fn test_multiplier_apply() {
        let m = Multiplier::with_params(ObisCode::CLOCK, 2.0, 10.0);
        assert!((m.apply(5.0) - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_multiplier_roundtrip() {
        let mut m = Multiplier::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Double(3.5));
        m.attribute_from_bytes(2, &bytes).unwrap();
        assert!((m.multiplier() - 3.5).abs() < f64::EPSILON);
    }
}
