//! Property-based tests for AXDR encoding/decoding

use proptest::prelude::*;
use dlms_axdr::{encode, decode};
use dlms_core::DlmsData;

proptest! {
    #[test]
    fn u8_roundtrip(value in any::<u8>()) {
        let encoded = encode(&DlmsData::Unsigned(value));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::Unsigned(value));
    }

    #[test]
    fn u16_roundtrip(value in any::<u16>()) {
        let encoded = encode(&DlmsData::LongUnsigned(value));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::LongUnsigned(value));
    }

    #[test]
    fn u32_roundtrip(value in any::<u32>()) {
        let encoded = encode(&DlmsData::DoubleLongUnsigned(value));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::DoubleLongUnsigned(value));
    }

    #[test]
    fn i8_roundtrip(value in any::<i8>()) {
        let encoded = encode(&DlmsData::Integer(value));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::Integer(value));
    }

    #[test]
    fn i16_roundtrip(value in any::<i16>()) {
        let encoded = encode(&DlmsData::Long(value));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::Long(value));
    }

    #[test]
    fn i32_roundtrip(value in any::<i32>()) {
        let encoded = encode(&DlmsData::DoubleLong(value));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::DoubleLong(value));
    }

    #[test]
    fn f32_roundtrip(value in any::<f32>()) {
        let encoded = encode(&DlmsData::Float(value));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::Float(value));
    }

    #[test]
    fn f64_roundtrip(value in any::<f64>()) {
        let encoded = encode(&DlmsData::Double(value));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::Double(value));
    }

    #[test]
    fn bool_roundtrip(value in any::<bool>()) {
        let encoded = encode(&DlmsData::Boolean(value));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::Boolean(value));
    }

    #[test]
    fn simple_octet_string(s in "[a-z]{1,10}") {
        let bytes = s.as_bytes().to_vec();
        let encoded = encode(&DlmsData::OctetString(bytes.clone()));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::OctetString(bytes));
    }
}
