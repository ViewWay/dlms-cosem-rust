//! RLRE (Release Response) APDU

use super::{Asn1Error, BerDecoder, BerEncoder};
use std::vec::Vec;

/// RLRE APDU - Release Response
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RlreApdu {
    pub result: u8,
}

impl RlreApdu {
    pub fn encode(&self) -> Vec<u8> {
        let mut content = Vec::new();

        // Result (IMPLICIT [0])
        content.push(0x80);
        content.extend(BerEncoder::encode_length(1));
        content.push(self.result);

        BerEncoder::encode_tlv(0x63, &content)
    }

    pub fn decode(data: &[u8]) -> Result<Self, Asn1Error> {
        let (tag, content, _) = BerDecoder::decode_tlv(data)?;
        if tag != 0x63 {
            return Err(Asn1Error::InvalidTag);
        }

        let mut result = 0;
        let mut pos = 0;
        while pos < content.len() {
            let (inner_tag, inner_value, inner_consumed) = BerDecoder::decode_tlv(&content[pos..])?;
            if inner_tag == 0x80 && !inner_value.is_empty() {
                result = inner_value[0];
            }
            pos += inner_consumed;
        }

        Ok(Self { result })
    }
}
