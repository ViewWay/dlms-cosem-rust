//! IC025 Image Transfer
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: image_block_size (long-unsigned)
//! 3: image_first_block (octet-string)
//! 4: image_block_count (double-long-unsigned)
//! 5: image_reference (octet-string)
//! 6: image_identification (unsigned)
//! 7: image_transfer_status (enum)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageTransferStatus {
    Idle = 0,
    Initiated = 1,
    InitiatedForVerifying = 2,
    VerifyingInitiated = 3,
    VerificationFailed = 4,
    VerificationSuccessful = 5,
    ImageActivated = 6,
    ImageNotActivated = 7,
}

pub struct ImageTransfer {
    logical_name: ObisCode,
    block_size: u16,
    first_block: Vec<u8>,
    block_count: u32,
    reference: Vec<u8>,
    identification: u8,
    status: ImageTransferStatus,
}

impl ImageTransfer {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            block_size: 200,
            first_block: Vec::new(),
            block_count: 0,
            reference: Vec::new(),
            identification: 0,
            status: ImageTransferStatus::Idle,
        }
    }

    pub fn status(&self) -> ImageTransferStatus {
        self.status
    }
    pub fn set_status(&mut self, s: ImageTransferStatus) {
        self.status = s;
    }
}

impl CosemObject for ImageTransfer {
    fn class_id(&self) -> u16 {
        18
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        7
    }
    fn method_count(&self) -> u8 {
        5
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.block_size))),
            3 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.first_block.clone(),
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.block_count,
            ))),
            5 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.reference.clone(),
            ))),
            6 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.identification))),
            7 => Some(dlms_axdr::encode(&DlmsData::Enum(self.status as u8))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            7 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(v) = decoded {
                    self.status = match v {
                        0 => ImageTransferStatus::Idle,
                        1 => ImageTransferStatus::Initiated,
                        2 => ImageTransferStatus::InitiatedForVerifying,
                        3 => ImageTransferStatus::VerifyingInitiated,
                        4 => ImageTransferStatus::VerificationFailed,
                        5 => ImageTransferStatus::VerificationSuccessful,
                        6 => ImageTransferStatus::ImageActivated,
                        7 => ImageTransferStatus::ImageNotActivated,
                        _ => return Err(CosemObjectError::InvalidData),
                    };
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
    fn test_image_transfer_class_id() {
        let it = ImageTransfer::new(ObisCode::new(0, 0, 44, 0, 0, 255));
        assert_eq!(it.class_id(), 18);
    }

    #[test]
    fn test_image_transfer_status() {
        let mut it = ImageTransfer::new(ObisCode::new(0, 0, 44, 0, 0, 255));
        assert_eq!(it.status(), ImageTransferStatus::Idle);
        it.set_status(ImageTransferStatus::Initiated);
        assert_eq!(it.status(), ImageTransferStatus::Initiated);
    }

    #[test]
    fn test_image_transfer_roundtrip() {
        let mut it = ImageTransfer::new(ObisCode::new(0, 0, 44, 0, 0, 255));
        let bytes = dlms_axdr::encode(&DlmsData::Enum(1));
        it.attribute_from_bytes(7, &bytes).unwrap();
        assert_eq!(it.status(), ImageTransferStatus::Initiated);
    }
}
