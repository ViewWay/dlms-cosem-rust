//! IC084 Compact Data

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Compact Data - compact data storage for meter readings
pub struct CompactData {
    logical_name: ObisCode,
    buffer: Vec<u8>,
    capture_time: DlmsData,
}

impl CompactData {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            buffer: vec![],
            capture_time: DlmsData::OctetString(vec![0; 8]),
        }
    }

    pub fn buffer(&self) -> &[u8] {
        &self.buffer
    }

    pub fn set_buffer(&mut self, buffer: Vec<u8>) {
        self.buffer = buffer;
    }

    pub fn capture_time(&self) -> &DlmsData {
        &self.capture_time
    }

    pub fn set_capture_time(&mut self, time: DlmsData) {
        self.capture_time = time;
    }
}

impl CosemObject for CompactData {
    fn class_id(&self) -> u16 {
        62
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
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.buffer.clone(),
            ))),
            3 => Some(dlms_axdr::encode(&self.capture_time)),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(bytes) = decoded.as_octet_string() {
                    self.buffer = bytes.to_vec();
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
    fn test_compact_data_new() {
        let cd = CompactData::new(ObisCode::CLOCK);
        assert_eq!(cd.class_id(), 62);
        assert!(cd.buffer().is_empty());
    }

    #[test]
    fn test_compact_data_buffer() {
        let mut cd = CompactData::new(ObisCode::CLOCK);
        cd.set_buffer(vec![1, 2, 3, 4, 5]);
        assert_eq!(cd.buffer().len(), 5);
    }

    #[test]
    fn test_compact_data_roundtrip() {
        let mut cd = CompactData::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::OctetString(vec![0xDE, 0xAD, 0xBE, 0xEF]));
        cd.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(cd.buffer(), &[0xDE, 0xAD, 0xBE, 0xEF]);
    }
}
