//! Basic BER (Basic Encoding Rules) support

use std::vec::Vec;

/// ASN.1 BER tag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BerTag(pub u8);

impl BerTag {
    pub const UNIVERSAL: u8 = 0x00;
    pub const APPLICATION: u8 = 0x40;
    pub const CONTEXT: u8 = 0x80;
    pub const PRIVATE: u8 = 0xC0;
    pub const CONSTRUCTED: u8 = 0x20;

    pub fn new(byte: u8) -> Self {
        Self(byte)
    }

    pub fn class(&self) -> u8 {
        self.0 & 0xC0
    }

    pub fn number(&self) -> u8 {
        self.0 & 0x1F
    }

    pub fn is_constructed(&self) -> bool {
        (self.0 & Self::CONSTRUCTED) != 0
    }

    pub fn context_constructed(n: u8) -> Self {
        Self(Self::CONTEXT | Self::CONSTRUCTED | (n & 0x1F))
    }

    pub fn context_primitive(n: u8) -> Self {
        Self(Self::CONTEXT | (n & 0x1F))
    }
}

/// BER encoder utilities
pub struct BerEncoder;

impl BerEncoder {
    pub fn encode_length(len: usize) -> Vec<u8> {
        if len < 0x80 {
            vec![len as u8]
        } else {
            let mut bytes = Vec::new();
            let mut remaining = len;
            let mut tmp = Vec::new();
            while remaining > 0 {
                tmp.push((remaining & 0xFF) as u8);
                remaining >>= 8;
            }
            bytes.push(0x80 | tmp.len() as u8);
            tmp.reverse();
            bytes.extend(&tmp);
            bytes
        }
    }

    pub fn encode_tlv(tag: u8, value: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(1 + 5 + value.len());
        result.push(tag);
        result.extend(Self::encode_length(value.len()));
        result.extend_from_slice(value);
        result
    }

    pub fn encode_integer(value: i32) -> Vec<u8> {
        let bytes = value.to_be_bytes();
        let mut len = 4;
        while len > 1 && bytes[4 - len] == 0 && (bytes[4 - len + 1] & 0x80) == 0 {
            len -= 1;
        }
        while len > 1 && bytes[4 - len] == 0xFF && (bytes[4 - len + 1] & 0x80) != 0 {
            len -= 1;
        }
        Self::encode_tlv(0x02, &bytes[4 - len..])
    }

    pub fn encode_octet_string(value: &[u8]) -> Vec<u8> {
        Self::encode_tlv(0x04, value)
    }

    pub fn encode_sequence(content: &[u8]) -> Vec<u8> {
        Self::encode_tlv(0x30, content)
    }

    pub fn encode_oid(value: &[u8]) -> Vec<u8> {
        Self::encode_tlv(0x06, value)
    }

    pub fn encode_bit_string(value: &[u8]) -> Vec<u8> {
        let mut content = Vec::with_capacity(value.len() + 1);
        content.push(0); // no unused bits
        content.extend_from_slice(value);
        Self::encode_tlv(0x03, &content)
    }

    pub fn encode_null() -> Vec<u8> {
        vec![0x05, 0x00]
    }

    pub fn encode_context_tagged(tag_num: u8, constructed: bool, value: &[u8]) -> Vec<u8> {
        let tag_byte = if constructed {
            BerTag::context_constructed(tag_num)
        } else {
            BerTag::context_primitive(tag_num)
        };
        Self::encode_tlv(tag_byte.0, value)
    }
}

/// BER decoder utilities
pub struct BerDecoder;

impl BerDecoder {
    pub fn decode_length(data: &[u8]) -> Result<(usize, usize), super::Asn1Error> {
        if data.is_empty() {
            return Err(super::Asn1Error::InsufficientData);
        }
        let first = data[0];
        if first < 0x80 {
            Ok((first as usize, 1))
        } else {
            let num_bytes = (first & 0x7F) as usize;
            if num_bytes == 0 || data.len() < 1 + num_bytes {
                return Err(super::Asn1Error::InvalidLength);
            }
            let mut len = 0usize;
            for i in 0..num_bytes {
                len = (len << 8) | data[1 + i] as usize;
            }
            Ok((len, 1 + num_bytes))
        }
    }

    pub fn decode_tlv(data: &[u8]) -> Result<(u8, &[u8], usize), super::Asn1Error> {
        if data.is_empty() {
            return Err(super::Asn1Error::InsufficientData);
        }
        let tag = data[0];
        let (len, len_bytes) = Self::decode_length(&data[1..])?;
        let total = 1 + len_bytes + len;
        if data.len() < total {
            return Err(super::Asn1Error::UnexpectedEof);
        }
        Ok((tag, &data[1 + len_bytes..1 + len_bytes + len], total))
    }

    pub fn decode_integer(data: &[u8]) -> Result<(i32, usize), super::Asn1Error> {
        let (tag, value, consumed) = Self::decode_tlv(data)?;
        if tag != 0x02 || value.is_empty() {
            return Err(super::Asn1Error::InvalidTag);
        }
        let mut result: i32 = 0;
        for &b in value {
            result = (result << 8) | (b as i32 & 0xFF);
        }
        // Sign extend
        if value[0] & 0x80 != 0 {
            let shift = (4 - value.len()) * 8;
            result = (result << shift) >> shift;
        }
        Ok((result, consumed))
    }

    pub fn decode_octet_string(data: &[u8]) -> Result<(Vec<u8>, usize), super::Asn1Error> {
        let (tag, value, consumed) = Self::decode_tlv(data)?;
        if tag != 0x04 {
            return Err(super::Asn1Error::InvalidTag);
        }
        Ok((value.to_vec(), consumed))
    }

    pub fn decode_oid(data: &[u8]) -> Result<(Vec<u8>, usize), super::Asn1Error> {
        let (tag, value, consumed) = Self::decode_tlv(data)?;
        if tag != 0x06 {
            return Err(super::Asn1Error::InvalidTag);
        }
        Ok((value.to_vec(), consumed))
    }
}
