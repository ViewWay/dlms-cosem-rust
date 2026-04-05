//! Common test utilities for AXDR testing

/// Perform an AXDR encode/decode roundtrip and verify equality
pub fn roundtrip(data: &dlms_core::DlmsData) -> dlms_core::DlmsData {
    let bytes = dlms_axdr::encode(data);
    dlms_axdr::decode(&bytes).expect("AXDR roundtrip decode failed")
}

/// Assert that an AXDR roundtrip preserves the value
pub fn assert_roundtrip(data: &dlms_core::DlmsData) {
    let decoded = roundtrip(data);
    assert_eq!(*data, decoded, "AXDR roundtrip failed for {:?}", data);
}
