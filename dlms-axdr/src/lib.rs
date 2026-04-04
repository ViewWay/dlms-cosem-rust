//! dlms-axdr: AXDR (ASN.1 XER Defined Rules) encoding/decoding for DLMS/COSEM
//!
//! Implements AXDR encoding for all DLMS data types as specified in IEC 62056-53.

#![cfg_attr(not(feature = "std"), no_std)]

// no_std support: feature gate for no_std

use dlms_core::DlmsData;
use std::vec::Vec;

mod decoder;
mod encoder;
mod length;

pub use decoder::AxdrDecoder;
pub use encoder::AxdvEncoder;
pub use length::{decode_length, encode_length};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AxdrError {
    InsufficientData,
    InvalidTag,
    InvalidLength,
    InvalidData,
    Overflow,
    UnsupportedType,
}

#[cfg(feature = "std")]
impl std::error::Error for AxdrError {}

impl core::fmt::Display for AxdrError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AxdrError::InsufficientData => write!(f, "Insufficient data"),
            AxdrError::InvalidTag => write!(f, "Invalid tag"),
            AxdrError::InvalidLength => write!(f, "Invalid length"),
            AxdrError::InvalidData => write!(f, "Invalid data"),
            AxdrError::Overflow => write!(f, "Value overflow"),
            AxdrError::UnsupportedType => write!(f, "Unsupported type"),
        }
    }
}

/// Convenience: encode a DlmsData value to AXDR bytes
pub fn encode(data: &DlmsData) -> Vec<u8> {
    let mut enc = AxdvEncoder::new();
    enc.encode(data);
    enc.finish()
}

/// Convenience: decode AXDR bytes to a DlmsData value
pub fn decode(bytes: &[u8]) -> Result<DlmsData, AxdrError> {
    let mut dec = AxdrDecoder::new(bytes);
    dec.decode()
}

#[cfg(test)]
mod tests {
    use super::*;
    use dlms_core::DlmsData;

    #[test]
    fn test_encode_none() {
        let data = DlmsData::None;
        let bytes = encode(&data);
        assert_eq!(bytes, vec![0x00, 0x00]);
    }

    #[test]
    fn test_decode_none() {
        let data = decode(&[0x00, 0x00]).unwrap();
        assert_eq!(data, DlmsData::None);
    }

    #[test]
    fn test_encode_boolean() {
        assert_eq!(encode(&DlmsData::Boolean(true)), vec![0x03, 0x01, 0x01]);
        assert_eq!(encode(&DlmsData::Boolean(false)), vec![0x03, 0x01, 0x00]);
    }

    #[test]
    fn test_decode_boolean() {
        assert_eq!(
            decode(&[0x03, 0x01, 0x01]).unwrap(),
            DlmsData::Boolean(true)
        );
        assert_eq!(
            decode(&[0x03, 0x01, 0x00]).unwrap(),
            DlmsData::Boolean(false)
        );
    }

    #[test]
    fn test_encode_integer() {
        let data = DlmsData::Integer(42);
        let bytes = encode(&data);
        assert_eq!(bytes, vec![0x0F, 0x01, 42]);
    }

    #[test]
    fn test_decode_integer() {
        assert_eq!(decode(&[0x0F, 0x01, 42]).unwrap(), DlmsData::Integer(42));
    }

    #[test]
    fn test_encode_integer_negative() {
        let data = DlmsData::Integer(-1);
        let bytes = encode(&data);
        assert_eq!(bytes, vec![0x0F, 0x01, 0xFF]);
    }

    #[test]
    fn test_decode_integer_negative() {
        assert_eq!(decode(&[0x0F, 0x01, 0xFF]).unwrap(), DlmsData::Integer(-1));
    }

    #[test]
    fn test_encode_unsigned() {
        assert_eq!(encode(&DlmsData::Unsigned(200)), vec![0x11, 0x01, 200]);
    }

    #[test]
    fn test_decode_unsigned() {
        assert_eq!(decode(&[0x11, 0x01, 200]).unwrap(), DlmsData::Unsigned(200));
    }

    #[test]
    fn test_encode_long() {
        let data = DlmsData::Long(1000);
        let bytes = encode(&data);
        assert_eq!(bytes, vec![0x10, 0x02, 0x03, 0xE8]);
    }

    #[test]
    fn test_decode_long() {
        assert_eq!(
            decode(&[0x10, 0x02, 0x03, 0xE8]).unwrap(),
            DlmsData::Long(1000)
        );
    }

    #[test]
    fn test_encode_long_unsigned() {
        let data = DlmsData::LongUnsigned(60000);
        let bytes = encode(&data);
        assert_eq!(bytes, vec![0x12, 0x02, 0xEA, 0x60]);
    }

    #[test]
    fn test_decode_long_unsigned() {
        assert_eq!(
            decode(&[0x12, 0x02, 0xEA, 0x60]).unwrap(),
            DlmsData::LongUnsigned(60000)
        );
    }

    #[test]
    fn test_encode_double_long() {
        let data = DlmsData::DoubleLong(100000);
        let bytes = encode(&data);
        assert_eq!(bytes, vec![0x05, 0x04, 0x00, 0x01, 0x86, 0xA0]);
    }

    #[test]
    fn test_decode_double_long() {
        assert_eq!(
            decode(&[0x05, 0x04, 0x00, 0x01, 0x86, 0xA0]).unwrap(),
            DlmsData::DoubleLong(100000)
        );
    }

    #[test]
    fn test_encode_double_long_unsigned() {
        let data = DlmsData::DoubleLongUnsigned(0xFFFFFFFF);
        let bytes = encode(&data);
        assert_eq!(bytes, vec![0x06, 0x04, 0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn test_decode_double_long_unsigned() {
        assert_eq!(
            decode(&[0x06, 0x04, 0xFF, 0xFF, 0xFF, 0xFF]).unwrap(),
            DlmsData::DoubleLongUnsigned(0xFFFFFFFF)
        );
    }

    #[test]
    fn test_encode_octet_string() {
        let data = DlmsData::OctetString(vec![0x01, 0x02, 0x03]);
        let bytes = encode(&data);
        assert_eq!(bytes, vec![0x09, 0x03, 0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_decode_octet_string() {
        assert_eq!(
            decode(&[0x09, 0x03, 0x01, 0x02, 0x03]).unwrap(),
            DlmsData::OctetString(vec![0x01, 0x02, 0x03])
        );
    }

    #[test]
    fn test_encode_empty_octet_string() {
        let data = DlmsData::OctetString(vec![]);
        let bytes = encode(&data);
        assert_eq!(bytes, vec![0x09, 0x00]);
    }

    #[test]
    fn test_decode_empty_octet_string() {
        assert_eq!(
            decode(&[0x09, 0x00]).unwrap(),
            DlmsData::OctetString(vec![])
        );
    }

    #[test]
    fn test_encode_visible_string() {
        let data = DlmsData::VisibleString("hello".into());
        let bytes = encode(&data);
        assert_eq!(bytes, vec![0x0A, 0x05, b'h', b'e', b'l', b'l', b'o']);
    }

    #[test]
    fn test_decode_visible_string() {
        assert_eq!(
            decode(&[0x0A, 0x05, b'h', b'e', b'l', b'l', b'o']).unwrap(),
            DlmsData::VisibleString("hello".into())
        );
    }

    #[test]
    fn test_encode_array() {
        let data = DlmsData::Array(vec![
            DlmsData::Unsigned(1),
            DlmsData::Unsigned(2),
            DlmsData::Unsigned(3),
        ]);
        let bytes = encode(&data);
        assert_eq!(bytes[0], 0x01); // Array tag
        assert_eq!(bytes[1], 0x03); // Count
    }

    #[test]
    fn test_decode_array() {
        let data = decode(&[
            0x01, 0x02, // Array with 2 elements
            0x11, 0x01, 0x0A, // Unsigned(10)
            0x11, 0x01, 0x14, // Unsigned(20)
        ])
        .unwrap();
        assert_eq!(data.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_encode_structure() {
        let data = DlmsData::Structure(vec![
            DlmsData::Unsigned(1),
            DlmsData::VisibleString("test".into()),
        ]);
        let bytes = encode(&data);
        assert_eq!(bytes[0], 0x02); // Structure tag
    }

    #[test]
    fn test_decode_structure() {
        let data = decode(&[
            0x02, 0x02, // Structure with 2 elements
            0x11, 0x01, 0x01, // Unsigned(1)
            0x0A, 0x04, b't', b'e', b's', b't', // VisibleString("test")
        ])
        .unwrap();
        assert_eq!(data.as_structure().unwrap().len(), 2);
    }

    #[test]
    fn test_encode_enum() {
        let data = DlmsData::Enum(5);
        let bytes = encode(&data);
        assert_eq!(bytes, vec![0x07, 0x01, 0x05]);
    }

    #[test]
    fn test_decode_enum() {
        assert_eq!(decode(&[0x07, 0x01, 0x05]).unwrap(), DlmsData::Enum(5));
    }

    #[test]
    fn test_encode_float() {
        let data = DlmsData::Float(3.14);
        let bytes = encode(&data);
        assert_eq!(bytes[0], 0x18);
        assert_eq!(bytes[1], 0x04); // 4 bytes
    }

    #[test]
    fn test_decode_float() {
        let val = DlmsData::Float(3.14);
        let bytes = encode(&val);
        let decoded = decode(&bytes).unwrap();
        if let DlmsData::Float(f) = decoded {
            assert!((f - 3.14f32).abs() < 0.01);
        } else {
            panic!("Expected Float");
        }
    }

    #[test]
    fn test_encode_double() {
        let data = DlmsData::Double(3.14159);
        let bytes = encode(&data);
        assert_eq!(bytes[0], 0x19);
        assert_eq!(bytes[1], 0x08); // 8 bytes
    }

    #[test]
    fn test_decode_double() {
        let val = DlmsData::Double(3.14159);
        let bytes = encode(&val);
        let decoded = decode(&bytes).unwrap();
        if let DlmsData::Double(f) = decoded {
            assert!((f - 3.14159f64).abs() < 0.00001);
        } else {
            panic!("Expected Double");
        }
    }

    #[test]
    fn test_encode_long64() {
        let data = DlmsData::Long64(i64::MAX);
        let bytes = encode(&data);
        assert_eq!(bytes[0], 0x16);
        assert_eq!(bytes[1], 0x08);
    }

    #[test]
    fn test_decode_long64() {
        let data = DlmsData::Long64(i64::MIN);
        let bytes = encode(&data);
        let decoded = decode(&bytes).unwrap();
        assert_eq!(decoded, DlmsData::Long64(i64::MIN));
    }

    #[test]
    fn test_encode_long64_unsigned() {
        let data = DlmsData::Long64Unsigned(u64::MAX);
        let bytes = encode(&data);
        assert_eq!(bytes[0], 0x17);
        assert_eq!(bytes[1], 0x08);
    }

    #[test]
    fn test_decode_long64_unsigned() {
        let data = DlmsData::Long64Unsigned(u64::MAX);
        let bytes = encode(&data);
        let decoded = decode(&bytes).unwrap();
        assert_eq!(decoded, DlmsData::Long64Unsigned(u64::MAX));
    }

    #[test]
    fn test_encode_bitstring() {
        let data = DlmsData::BitString {
            unused_bits: 4,
            data: vec![0xF0],
        };
        let bytes = encode(&data);
        assert_eq!(bytes[0], 0x04);
    }

    #[test]
    fn test_decode_bitstring() {
        let data = decode(&[0x04, 0x02, 0x04, 0xF0]).unwrap();
        assert_eq!(
            data,
            DlmsData::BitString {
                unused_bits: 4,
                data: vec![0xF0]
            }
        );
    }

    #[test]
    fn test_encode_date_time() {
        let data = DlmsData::DateTime([
            0x07, 0xE8, 0x06, 0x0F, 0x06, 0x0A, 0x1E, 0x2D, 0x32, 0x01, 0xE0, 0x00,
        ]);
        let bytes = encode(&data);
        assert_eq!(bytes[0], 0x1A);
        assert_eq!(bytes[1], 0x0C);
    }

    #[test]
    fn test_decode_date_time() {
        let bytes = [
            0x1A, 0x0C, 0x07, 0xE8, 0x06, 0x0F, 0x06, 0x0A, 0x1E, 0x2D, 0x32, 0x01, 0xE0, 0x00,
        ];
        let data = decode(&bytes).unwrap();
        assert_eq!(data.tag(), 0x1A);
    }

    #[test]
    fn test_roundtrip_all_types() {
        let values = vec![
            DlmsData::None,
            DlmsData::Boolean(true),
            DlmsData::Boolean(false),
            DlmsData::Integer(-128),
            DlmsData::Integer(127),
            DlmsData::Unsigned(0),
            DlmsData::Unsigned(255),
            DlmsData::Long(i16::MIN),
            DlmsData::Long(i16::MAX),
            DlmsData::LongUnsigned(u16::MIN),
            DlmsData::LongUnsigned(u16::MAX),
            DlmsData::DoubleLong(i32::MIN),
            DlmsData::DoubleLong(i32::MAX),
            DlmsData::DoubleLongUnsigned(u32::MIN),
            DlmsData::DoubleLongUnsigned(u32::MAX),
            DlmsData::Long64(i64::MIN),
            DlmsData::Long64(i64::MAX),
            DlmsData::Long64Unsigned(u64::MIN),
            DlmsData::Long64Unsigned(u64::MAX),
            DlmsData::Enum(0),
            DlmsData::Enum(255),
            DlmsData::OctetString(vec![]),
            DlmsData::OctetString(vec![0xDE, 0xAD]),
            DlmsData::VisibleString(String::new()),
            DlmsData::VisibleString("DLMS".into()),
            DlmsData::Array(vec![]),
            DlmsData::Structure(vec![]),
        ];
        for v in &values {
            let bytes = encode(v);
            let decoded = decode(&bytes).unwrap();
            assert_eq!(*v, decoded, "Roundtrip failed for {:?}", v);
        }
    }

    #[test]
    fn test_decode_insufficient_data() {
        let result = decode(&[0x0F]); // Integer with no length/data
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_empty_input() {
        let result = decode(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_invalid_tag() {
        let result = decode(&[0xFE, 0x01, 0x00]);
        assert!(result.is_err());
    }

    #[test]
    fn test_length_encoding() {
        assert_eq!(encode_length(0), vec![0x00]);
        assert_eq!(encode_length(127), vec![0x7F]);
        assert_eq!(encode_length(128), vec![0x81, 0x80]);
        assert_eq!(encode_length(255), vec![0x81, 0xFF]);
        assert_eq!(encode_length(256), vec![0x82, 0x01, 0x00]);
    }

    #[test]
    fn test_length_decoding() {
        assert_eq!(decode_length(&[0x00], &mut 0).unwrap().0, 0);
        assert_eq!(decode_length(&[0x7F], &mut 0).unwrap().0, 127);
        assert_eq!(decode_length(&[0x81, 0x80], &mut 0).unwrap().0, 128);
        assert_eq!(decode_length(&[0x82, 0x01, 0x00], &mut 0).unwrap().0, 256);
    }

    #[test]
    fn test_encoder_new() {
        let enc = AxdvEncoder::new();
        assert!(enc.is_empty());
    }

    #[test]
    fn test_decoder_remaining() {
        let dec = AxdrDecoder::new(&[0x00, 0x00, 0x11, 0x01, 0x05]);
        assert_eq!(dec.remaining(), 5);
    }

    #[test]
    fn test_error_display() {
        let err = AxdrError::InsufficientData;
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_encode_utf8_string() {
        let data = DlmsData::Utf8String("测试".into());
        let bytes = encode(&data);
        assert_eq!(bytes[0], 0x0C);
        let decoded = decode(&bytes).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_encode_compact_array() {
        let data = DlmsData::CompactArray {
            header: vec![0x01, 0x02],
            data: vec![DlmsData::Unsigned(42)],
        };
        let bytes = encode(&data);
        assert_eq!(bytes[0], 0x1F);
    }

    #[test]
    fn test_nested_array() {
        let data = DlmsData::Array(vec![
            DlmsData::Array(vec![DlmsData::Unsigned(1), DlmsData::Unsigned(2)]),
            DlmsData::Array(vec![DlmsData::Unsigned(3)]),
        ]);
        let bytes = encode(&data);
        let decoded = decode(&bytes).unwrap();
        assert_eq!(data, decoded);
    }
}
