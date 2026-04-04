//! IC116 IEC 62055-41 Attributes
//! Blue Book Ed16: class_id=116, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IEC 62055-41 Attributes - STS token carrier attributes
pub struct Iec62055Attributes {
    logical_name: ObisCode,
    sts_key_identification_no: u64,
    sts_key_revision_no: u32,
    sts_key_expiry_date: DlmsData,
    sts_token_carrier_id: u32,
}

impl Iec62055Attributes {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            sts_key_identification_no: 0,
            sts_key_revision_no: 0,
            sts_key_expiry_date: DlmsData::OctetString(vec![0xFF; 12]),
            sts_token_carrier_id: 0,
        }
    }
    pub fn key_id(&self) -> u64 {
        self.sts_key_identification_no
    }
    pub fn set_key_id(&mut self, id: u64) {
        self.sts_key_identification_no = id;
    }
}

impl CosemObject for Iec62055Attributes {
    fn class_id(&self) -> u16 {
        116
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        6
    }
    fn method_count(&self) -> u8 {
        0
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let n = self.logical_name.to_bytes();
                Some(vec![0x09, 0x06, n[0], n[1], n[2], n[3], n[4], n[5]])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.sts_key_identification_no as u32,
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.sts_key_revision_no,
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
    fn test_iec62055_new() {
        let i = Iec62055Attributes::new(ObisCode::CLOCK);
        assert_eq!(i.class_id(), 116);
    }
    #[test]
    fn test_iec62055_key() {
        let mut i = Iec62055Attributes::new(ObisCode::CLOCK);
        i.set_key_id(12345);
        assert_eq!(i.key_id(), 12345);
    }
}
