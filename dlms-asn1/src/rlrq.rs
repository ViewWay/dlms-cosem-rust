//! RLRQ (Release Request) APDU

use std::vec::Vec;
use super::{BerEncoder, BerDecoder, Asn1Error};

/// RLRQ APDU - Release Request
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RlrqApdu {
    pub reason: u8,
}

impl RlrqApdu {
    pub fn encode(&self) -> Vec<u8> {
        let mut content = Vec::new();

        // Reason (IMPLICIT [0])
        content.push(0x80);
        content.extend(BerEncoder::encode_length(1));
        content.push(self.reason);

        BerEncoder::encode_tlv(0x62, &content)
    }

    pub fn decode(data: &[u8]) -> Result<Self, Asn1Error> {
        let (tag, content, _) = BerDecoder::decode_tlv(data)?;
        if tag != 0x62 {
            return Err(Asn1Error::InvalidTag);
        }

        let mut reason = 0;
        let mut pos = 0;
        while pos < content.len() {
            let (inner_tag, inner_value, inner_consumed) = BerDecoder::decode_tlv(&content[pos..])?;
            if inner_tag == 0x80 && !inner_value.is_empty() {
                reason = inner_value[0];
            }
            pos += inner_consumed;
        }

        Ok(Self { reason })
    }
}
