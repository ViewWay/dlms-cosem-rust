//! AXDR encoder

use std::vec::Vec;
use dlms_core::DlmsData;
use super::encode_length;

pub struct AxdvEncoder {
    buffer: Vec<u8>,
}

impl AxdvEncoder {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn encode(&mut self, data: &DlmsData) {
        self.buffer.push(data.tag());
        match data {
            DlmsData::None => {
                self.buffer.push(0x00);
            }
            DlmsData::Boolean(b) => {
                self.buffer.push(0x01);
                self.buffer.push(if *b { 0x01 } else { 0x00 });
            }
            DlmsData::BitString { unused_bits, data } => {
                self.buffer.extend(encode_length(data.len() + 1));
                self.buffer.push(*unused_bits);
                self.buffer.extend_from_slice(data);
            }
            DlmsData::DoubleLong(v) => {
                self.buffer.push(0x04);
                self.buffer.extend_from_slice(&v.to_be_bytes());
            }
            DlmsData::DoubleLongUnsigned(v) => {
                self.buffer.push(0x04);
                self.buffer.extend_from_slice(&v.to_be_bytes());
            }
            DlmsData::OctetString(v) => {
                self.buffer.extend(encode_length(v.len()));
                self.buffer.extend_from_slice(v);
            }
            DlmsData::VisibleString(v) => {
                self.buffer.extend(encode_length(v.len()));
                self.buffer.extend_from_slice(v.as_bytes());
            }
            DlmsData::Utf8String(v) => {
                self.buffer.extend(encode_length(v.len()));
                self.buffer.extend_from_slice(v.as_bytes());
            }
            DlmsData::Bcd(v) => {
                self.buffer.extend(encode_length(v.len()));
                self.buffer.extend_from_slice(v);
            }
            DlmsData::Integer(v) => {
                let bytes = v.to_be_bytes();
                let len = Self::trim_leading_bytes(&bytes);
                self.buffer.push(len as u8);
                self.buffer.extend_from_slice(&bytes[bytes.len() - len..]);
            }
            DlmsData::Long(v) => {
                let bytes = v.to_be_bytes();
                self.buffer.push(0x02);
                self.buffer.extend_from_slice(&bytes);
            }
            DlmsData::Unsigned(v) => {
                let bytes = v.to_be_bytes();
                let len = Self::trim_leading_bytes(&bytes);
                self.buffer.push(len as u8);
                self.buffer.extend_from_slice(&bytes[bytes.len() - len..]);
            }
            DlmsData::LongUnsigned(v) => {
                let bytes = v.to_be_bytes();
                self.buffer.push(0x02);
                self.buffer.extend_from_slice(&bytes);
            }
            DlmsData::Long64(v) => {
                let bytes = v.to_be_bytes();
                self.buffer.push(0x08);
                self.buffer.extend_from_slice(&bytes);
            }
            DlmsData::Long64Unsigned(v) => {
                let bytes = v.to_be_bytes();
                self.buffer.push(0x08);
                self.buffer.extend_from_slice(&bytes);
            }
            DlmsData::Float(v) => {
                self.buffer.push(0x04);
                self.buffer.extend_from_slice(&v.to_be_bytes());
            }
            DlmsData::Double(v) => {
                self.buffer.push(0x08);
                self.buffer.extend_from_slice(&v.to_be_bytes());
            }
            DlmsData::DateTime(dt) => {
                self.buffer.push(0x0C);
                self.buffer.extend_from_slice(dt);
            }
            DlmsData::Date(d) => {
                self.buffer.push(0x05);
                self.buffer.extend_from_slice(d);
            }
            DlmsData::Time(t) => {
                self.buffer.push(0x04);
                self.buffer.extend_from_slice(t);
            }
            DlmsData::Array(items) => {
                self.buffer.extend(encode_length(items.len()));
                for item in items {
                    self.encode(item);
                }
            }
            DlmsData::Structure(items) => {
                self.buffer.extend(encode_length(items.len()));
                for item in items {
                    self.encode(item);
                }
            }
            DlmsData::CompactArray { header, data } => {
                let mut inner = Vec::new();
                inner.extend_from_slice(header);
                for item in data {
                    inner.push(item.tag());
                    match item {
                        DlmsData::OctetString(v) => {
                            inner.extend(encode_length(v.len()));
                            inner.extend_from_slice(v);
                        }
                        DlmsData::DoubleLong(v) => {
                            inner.push(0x04);
                            inner.extend_from_slice(&v.to_be_bytes());
                        }
                        DlmsData::LongUnsigned(v) => {
                            inner.push(0x02);
                            inner.extend_from_slice(&v.to_be_bytes());
                        }
                        _ => {
                            // Fallback: encode normally
                            let mut sub_enc = AxdvEncoder::new();
                            sub_enc.encode(item);
                            inner.extend_from_slice(&sub_enc.buffer[1..]); // skip tag
                        }
                    }
                }
                self.buffer.extend(encode_length(inner.len()));
                self.buffer.extend(inner);
            }
            DlmsData::Enum(v) => {
                self.buffer.push(0x01);
                self.buffer.push(*v);
            }
            DlmsData::CompactArrayDefinition(items) => {
                self.buffer.extend(encode_length(items.len()));
                for item in items {
                    self.encode(item);
                }
            }
        }
    }

    pub fn finish(self) -> Vec<u8> {
        self.buffer
    }

    fn trim_leading_bytes(bytes: &[u8]) -> usize {
        let mut i = 0;
        while i < bytes.len() - 1 && bytes[i] == 0 {
            // For signed types, check if the remaining MSB would lose sign info
            if i + 1 < bytes.len() && (bytes[i + 1] & 0x80) != 0 {
                break;
            }
            i += 1;
        }
        bytes.len() - i
    }
}

impl Default for AxdvEncoder {
    fn default() -> Self {
        Self::new()
    }
}
