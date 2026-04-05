//! Common test utilities for DLMS/COSEM testing
//!
//! This module provides shared helpers for building test data, APDUs, and other
//! DLMS constructs used across the test suite.

use dlms_core::{DlmsData, ObisCode, CosemDateTime};

/// Build a simple DLMS APDU with a given invoke ID and tag
pub fn build_apdu(invoke_id: u8, tag: u8, data: &[u8]) -> Vec<u8> {
    let mut apdu = Vec::new();
    // APDU header: AssociationInvokeId | Tag | Length
    apdu.push(invoke_id);
    apdu.push(tag);
    apdu.push(data.len() as u8);
    apdu.extend_from_slice(data);
    apdu
}

/// Build a GET-REQUEST APDU for a COSEM object attribute
pub fn build_get_request(invoke_id: u8, obis: &ObisCode, attribute_id: u8) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(obis.bytes());
    data.push(attribute_id);
    data.push(0); // Access selector
    build_apdu(invoke_id, 0xC0, &data)
}

/// Build a SET-REQUEST APDU for a COSEM object attribute
pub fn build_set_request(invoke_id: u8, obis: &ObisCode, attribute_id: u8, value: &DlmsData) -> Vec<u8> {
    use dlms_axdr::{encode};
    let mut data = Vec::new();
    data.extend_from_slice(obis.bytes());
    data.push(attribute_id);
    data.push(0); // Access selector
    data.extend_from_slice(&encode(value));
    build_apdu(invoke_id, 0xC1, &data)
}

/// Build an ACTION-REQUEST APDU
pub fn build_action_request(invoke_id: u8, obis: &ObisCode, method_id: u8) -> Vec<u8> {
    let mut data = Vec::new();
    data.extend_from_slice(obis.bytes());
    data.push(method_id);
    data.push(0); // Method parameter (none)
    build_apdu(invoke_id, 0xC2, &data)
}

/// Create a test OBIS code for logical name 1.0.0.1.0.255
pub fn test_obis_code() -> ObisCode {
    ObisCode::new([1, 0, 0, 1, 0, 255])
}

/// Create a test COSEM date-time (2024-01-15 12:30:45)
pub fn test_datetime() -> CosemDateTime {
    // DLMS datetime: year(2), month, day, weekday, hour, minute, second, hundredths, deviation, status
    CosemDateTime::new([
        0x07, 0xE8, // 2024
        0x01,       // January
        0x0F,       // 15
        0x01,       // Monday
        0x0C,       // 12
        0x1E,       // 30
        0x2D,       // 45
        0x00,       // 0 hundredths
        0x00,       // Deviation 0
        0x00,       // Status OK
    ])
}

/// Test vectors module with known DLMS values
pub mod test_vectors {
    use dlms_core::DlmsData;

    /// Known test vectors for various DLMS data types
    pub struct TestVectors;

    impl TestVectors {
        /// Get all test data values
        pub fn all_test_values() -> Vec<DlmsData> {
            vec![
                DlmsData::None,
                DlmsData::Boolean(true),
                DlmsData::Boolean(false),
                DlmsData::Integer(-128),
                DlmsData::Integer(-1),
                DlmsData::Integer(0),
                DlmsData::Integer(127),
                DlmsData::Unsigned(0),
                DlmsData::Unsigned(1),
                DlmsData::Unsigned(255),
                DlmsData::Long(i16::MIN),
                DlmsData::Long(0),
                DlmsData::Long(i16::MAX),
                DlmsData::LongUnsigned(0),
                DlmsData::LongUnsigned(u16::MAX),
                DlmsData::DoubleLong(i32::MIN),
                DlmsData::DoubleLong(0),
                DlmsData::DoubleLong(i32::MAX),
                DlmsData::DoubleLongUnsigned(0),
                DlmsData::DoubleLongUnsigned(u32::MAX),
                DlmsData::Long64(i64::MIN),
                DlmsData::Long64(0),
                DlmsData::Long64(i64::MAX),
                DlmsData::Long64Unsigned(0),
                DlmsData::Long64Unsigned(u64::MAX),
                DlmsData::Float(0.0),
                DlmsData::Float(3.14),
                DlmsData::Double(0.0),
                DlmsData::Double(3.14159),
                DlmsData::Enum(0),
                DlmsData::Enum(255),
                DlmsData::OctetString(vec![]),
                DlmsData::OctetString(vec![0xDE, 0xAD, 0xBE, 0xEF]),
                DlmsData::VisibleString(String::new()),
                DlmsData::VisibleString("DLMS/COSEM".to_string()),
                DlmsData::Utf8String("测试".to_string()),
                DlmsData::BitString { unused_bits: 4, data: vec![0xF0] },
                DlmsData::Array(vec![
                    DlmsData::Unsigned(1),
                    DlmsData::Unsigned(2),
                    DlmsData::Unsigned(3),
                ]),
                DlmsData::Structure(vec![
                    DlmsData::Unsigned(1),
                    DlmsData::VisibleString("test".to_string()),
                ]),
            ]
        }

        /// Get a specific test data value by index
        pub fn get(index: usize) -> DlmsData {
            let values = Self::all_test_values();
            values[index % values.len()]
        }
    }
}
