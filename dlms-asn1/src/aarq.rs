//! AARQ (Association Request) APDU

use std::vec::Vec;
use super::{BerEncoder, BerDecoder, Asn1Error};

/// AARQ APDU - Association Request
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AarqApdu {
    pub protocol_version: u8,
    pub application_context_name: Vec<u8>,
    pub called_ap_title: Option<Vec<u8>>,
    pub calling_ap_title: Option<Vec<u8>>,
    pub mechanism_name: Option<Vec<u8>>,
    pub user_information: Option<Vec<u8>>,
}

impl AarqApdu {
    pub fn encode(&self) -> Vec<u8> {
        let mut content = Vec::new();

        // Protocol version (IMPLICIT [0])
        let version = BerEncoder::encode_integer(self.protocol_version as i32);
        let mut version_tlv = Vec::new();
        version_tlv.extend_from_slice(&version[2..]); // skip tag+length, keep value
        content.push(0xA0); // context [0] constructed
        content.extend(BerEncoder::encode_length(version_tlv.len()));
        content.extend(&version_tlv);

        // Application context name (IMPLICIT [1])
        content.push(0xA1);
        content.extend(BerEncoder::encode_length(self.application_context_name.len()));
        content.extend(&self.application_context_name);

        // Called AP title (IMPLICIT [2]) - optional
        if let Some(ref called) = self.called_ap_title {
            content.push(0xA2);
            content.extend(BerEncoder::encode_length(called.len()));
            content.extend(called);
        }

        // Calling AP title (IMPLICIT [3]) - optional
        if let Some(ref calling) = self.calling_ap_title {
            content.push(0xA3);
            content.extend(BerEncoder::encode_length(calling.len()));
            content.extend(calling);
        }

        // Mechanism name (IMPLICIT [4]) - optional
        if let Some(ref mech) = self.mechanism_name {
            content.push(0xA4);
            content.extend(BerEncoder::encode_length(mech.len()));
            content.extend(mech);
        }

        // User information (IMPLICIT [5]) - optional
        if let Some(ref ui) = self.user_information {
            content.push(0xA5);
            content.extend(BerEncoder::encode_length(ui.len()));
            content.extend(ui);
        }

        BerEncoder::encode_tlv(0x60, &content)
    }

    pub fn decode(data: &[u8]) -> Result<Self, Asn1Error> {
        let (tag, content, _) = BerDecoder::decode_tlv(data)?;
        if tag != 0x60 {
            return Err(Asn1Error::InvalidTag);
        }

        let mut result = AarqApdu {
            protocol_version: 0,
            application_context_name: Vec::new(),
            called_ap_title: None,
            calling_ap_title: None,
            mechanism_name: None,
            user_information: None,
        };

        let mut pos = 0;
        while pos < content.len() {
            let (inner_tag, inner_value, inner_consumed) = BerDecoder::decode_tlv(&content[pos..])?;
            match inner_tag {
                0xA0 => {
                    // Protocol version
                    if !inner_value.is_empty() {
                        result.protocol_version = inner_value[0];
                    }
                }
                0xA1 => {
                    result.application_context_name = inner_value.to_vec();
                }
                0xA2 => {
                    result.called_ap_title = Some(inner_value.to_vec());
                }
                0xA3 => {
                    result.calling_ap_title = Some(inner_value.to_vec());
                }
                0xA4 => {
                    result.mechanism_name = Some(inner_value.to_vec());
                }
                0xA5 => {
                    result.user_information = Some(inner_value.to_vec());
                }
                _ => {}
            }
            pos += inner_consumed;
        }

        Ok(result)
    }
}
