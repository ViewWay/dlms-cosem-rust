//! Property-based tests for AXDR encoding/decoding

use dlms_axdr::{decode, encode};
use dlms_core::DlmsData;
use proptest::prelude::*;

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

    // === New property tests ===

    #[test]
    fn enum_roundtrip(value in 0u8..=255u8) {
        let encoded = encode(&DlmsData::Enum(value));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::Enum(value));
    }

    #[test]
    fn visible_string_roundtrip(s in "[A-Za-z0-9 ]{0,50}") {
        let encoded = encode(&DlmsData::VisibleString(s.clone()));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::VisibleString(s));
    }

    #[test]
    fn utf8_string_roundtrip(s in ".{0,50}") {
        let encoded = encode(&DlmsData::Utf8String(s.clone()));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::Utf8String(s));
    }

    #[test]
    fn none_roundtrip(_v in any::<()>()) {
        let encoded = encode(&DlmsData::None);
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::None);
    }

    #[test]
    fn binary_octet_string(bytes in proptest::collection::vec(any::<u8>(), 0..200)) {
        let encoded = encode(&DlmsData::OctetString(bytes.clone()));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::OctetString(bytes));
    }

    #[test]
    fn small_array_roundtrip(vals in proptest::collection::vec(any::<u8>(), 0..10)) {
        let items: Vec<DlmsData> = vals.iter().map(|&v| DlmsData::Unsigned(v)).collect();
        let encoded = encode(&DlmsData::Array(items.clone()));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::Array(items));
    }

    #[test]
    fn small_structure_roundtrip(vals in proptest::collection::vec(any::<i32>(), 0..5)) {
        let items: Vec<DlmsData> = vals.iter().map(|&v| DlmsData::DoubleLong(v)).collect();
        let encoded = encode(&DlmsData::Structure(items.clone()));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::Structure(items));
    }

    #[test]
    fn array_boundary_sizes(len in 0usize..5) {
        // Generate arrays of exactly len elements
        let items: Vec<DlmsData> = (0..len).map(|i| DlmsData::Unsigned(i as u8)).collect();
        let encoded = encode(&DlmsData::Array(items.clone()));
        let decoded = decode(&encoded).unwrap();
        prop_assert_eq!(decoded, DlmsData::Array(items));
    }
}

// === Boundary value tests (non-proptest, but testing specific boundaries) ===

#[test]
fn test_octet_string_empty() {
    let encoded = encode(&DlmsData::OctetString(vec![]));
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, DlmsData::OctetString(vec![]));
}

#[test]
fn test_octet_string_all_zeros() {
    let data = vec![0u8; 200];
    let encoded = encode(&DlmsData::OctetString(data.clone()));
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, DlmsData::OctetString(data));
}

#[test]
fn test_octet_string_all_ff() {
    let data = vec![0xFFu8; 200];
    let encoded = encode(&DlmsData::OctetString(data.clone()));
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, DlmsData::OctetString(data));
}

#[test]
fn test_octet_string_length_127() {
    // Boundary: single-byte length max
    let data = vec![0xABu8; 127];
    let encoded = encode(&DlmsData::OctetString(data.clone()));
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, DlmsData::OctetString(data));
}

#[test]
fn test_octet_string_length_128() {
    // Boundary: first two-byte length value
    let data = vec![0xABu8; 128];
    let encoded = encode(&DlmsData::OctetString(data.clone()));
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, DlmsData::OctetString(data));
}

#[test]
fn test_octet_string_length_255() {
    let data = vec![0xCDu8; 255];
    let encoded = encode(&DlmsData::OctetString(data.clone()));
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, DlmsData::OctetString(data));
}

#[test]
fn test_octet_string_length_256() {
    let data = vec![0xEFu8; 256];
    let encoded = encode(&DlmsData::OctetString(data.clone()));
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, DlmsData::OctetString(data));
}

#[test]
fn test_octet_string_length_65535() {
    let data = vec![0x42u8; 65535];
    let encoded = encode(&DlmsData::OctetString(data.clone()));
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, DlmsData::OctetString(data));
}

#[test]
fn test_u16_boundary_values() {
    for &val in &[0u16, 127, 128, 255, 256, 65535] {
        let encoded = encode(&DlmsData::LongUnsigned(val));
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, DlmsData::LongUnsigned(val));
    }
}

#[test]
fn test_u32_boundary_values() {
    for &val in &[0u32, 127, 128, 255, 256, 65535, 65536, u32::MAX] {
        let encoded = encode(&DlmsData::DoubleLongUnsigned(val));
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, DlmsData::DoubleLongUnsigned(val));
    }
}

#[test]
fn test_i32_boundary_values() {
    for &val in &[0i32, -1, 1, i8::MIN as i32, i8::MAX as i32, i16::MIN as i32, i16::MAX as i32, i32::MIN, i32::MAX] {
        let encoded = encode(&DlmsData::DoubleLong(val));
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, DlmsData::DoubleLong(val));
    }
}

#[test]
fn test_array_size_100() {
    let items: Vec<DlmsData> = (0..100).map(|i| DlmsData::Unsigned(i as u8)).collect();
    let encoded = encode(&DlmsData::Array(items.clone()));
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, DlmsData::Array(items));
}

#[test]
fn test_nested_structure_depth_3() {
    let inner = DlmsData::Structure(vec![DlmsData::Unsigned(1), DlmsData::Unsigned(2)]);
    let middle = DlmsData::Structure(vec![inner.clone(), DlmsData::Unsigned(3)]);
    let outer = DlmsData::Structure(vec![middle.clone(), DlmsData::Unsigned(4)]);

    let encoded = encode(&outer);
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, outer);
}

#[test]
fn test_empty_array() {
    let encoded = encode(&DlmsData::Array(vec![]));
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, DlmsData::Array(vec![]));
}

#[test]
fn test_empty_structure() {
    let encoded = encode(&DlmsData::Structure(vec![]));
    let decoded = decode(&encoded).unwrap();
    assert_eq!(decoded, DlmsData::Structure(vec![]));
}

#[test]
fn test_decode_empty_input_fails() {
    assert!(decode(&[]).is_err());
}

#[test]
fn test_decode_single_byte_fails() {
    // Tag only, no data
    assert!(decode(&[0x11]).is_err());
}
