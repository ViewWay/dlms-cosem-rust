//! AARE (Association Response) APDU

use super::{Asn1Error, BerDecoder, BerEncoder};
use std::vec::Vec;

/// AARE APDU - Association Response
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AareApdu {
    pub result: u8,
    pub result_source_diagnostic: Option<Vec<u8>>,
    pub application_context_name: Vec<u8>,
    pub responding_ap_title: Option<Vec<u8>>,
    pub mechanism_name: Option<Vec<u8>>,
    pub user_information: Option<Vec<u8>>,
}

impl AareApdu {
    pub fn encode(&self) -> Vec<u8> {
        let mut content = Vec::new();

        // Result (IMPLICIT [0])
        content.push(0xA0);
        content.extend(BerEncoder::encode_length(1));
        content.push(self.result);

        // Result source diagnostic (IMPLICIT [1]) - optional
        if let Some(ref diag) = self.result_source_diagnostic {
            content.push(0xA1);
            content.extend(BerEncoder::encode_length(diag.len()));
            content.extend(diag);
        }

        // Application context name (IMPLICIT [2])
        content.push(0xA2);
        content.extend(BerEncoder::encode_length(
            self.application_context_name.len(),
        ));
        content.extend(&self.application_context_name);

        // Responding AP title (IMPLICIT [3]) - optional
        if let Some(ref resp) = self.responding_ap_title {
            content.push(0xA3);
            content.extend(BerEncoder::encode_length(resp.len()));
            content.extend(resp);
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

        BerEncoder::encode_tlv(0x61, &content)
    }

    pub fn decode(data: &[u8]) -> Result<Self, Asn1Error> {
        let (tag, content, _) = BerDecoder::decode_tlv(data)?;
        if tag != 0x61 {
            return Err(Asn1Error::InvalidTag);
        }

        let mut result = AareApdu {
            result: 0,
            result_source_diagnostic: None,
            application_context_name: Vec::new(),
            responding_ap_title: None,
            mechanism_name: None,
            user_information: None,
        };

        let mut pos = 0;
        while pos < content.len() {
            let (inner_tag, inner_value, inner_consumed) = BerDecoder::decode_tlv(&content[pos..])?;
            match inner_tag {
                0xA0 => {
                    if !inner_value.is_empty() {
                        result.result = inner_value[0];
                    }
                }
                0xA1 => {
                    result.result_source_diagnostic = Some(inner_value.to_vec());
                }
                0xA2 => {
                    result.application_context_name = inner_value.to_vec();
                }
                0xA3 => {
                    result.responding_ap_title = Some(inner_value.to_vec());
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

    pub fn is_accepted(&self) -> bool {
        self.result == 0
    }
}
