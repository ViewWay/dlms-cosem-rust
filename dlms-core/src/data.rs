//! DlmsData - All DLMS data type enumerations per IEC 62056-53

// no_std support
use std::vec::Vec;
// no_std support
use std::string::String;

/// DLMS data types as defined in IEC 62056-53 (Blue Book)
#[derive(Clone, Debug, PartialEq)]
pub enum DlmsData {
    /// No data / null
    None,
    /// Boolean
    Boolean(bool),
    /// Bit string
    BitString { unused_bits: u8, data: Vec<u8> },
    /// Double long signed (i32)
    DoubleLong(i32),
    /// Double long unsigned (u32)
    DoubleLongUnsigned(u32),
    /// Octet string (byte array)
    OctetString(Vec<u8>),
    /// Visible string (UTF-8-like)
    VisibleString(String),
    /// UTF-8 string
    Utf8String(String),
    /// BCD
    Bcd(Vec<u8>),
    /// Integer (i8)
    Integer(i8),
    /// Long (i16)
    Long(i16),
    /// Unsigned (u8)
    Unsigned(u8),
    /// Long unsigned (u16)
    LongUnsigned(u16),
    /// Long64 (i64)
    Long64(i64),
    /// Long64 unsigned (u64)
    Long64Unsigned(u64),
    /// Float (f32)
    Float(f32),
    /// Double (f64)
    Double(f64),
    /// Date-time (12 bytes)
    DateTime([u8; 12]),
    /// Date (5 bytes)
    Date([u8; 5]),
    /// Time (4 bytes)
    Time([u8; 4]),
    /// Array of DlmsData
    Array(Vec<DlmsData>),
    /// Structure (ordered collection)
    Structure(Vec<DlmsData>),
    /// Compact array
    CompactArray {
        header: Vec<u8>,
        data: Vec<DlmsData>,
    },
    /// Enum
    Enum(u8),
    /// Compact array definition
    CompactArrayDefinition(Vec<DlmsData>),
}

impl DlmsData {
    /// Get the tag byte for this data type
    pub fn tag(&self) -> u8 {
        match self {
            DlmsData::None => 0x00,
            DlmsData::Boolean(_) => 0x03,
            DlmsData::BitString { .. } => 0x04,
            DlmsData::DoubleLong(_) => 0x05,
            DlmsData::DoubleLongUnsigned(_) => 0x06,
            DlmsData::OctetString(_) => 0x09,
            DlmsData::VisibleString(_) => 0x0A,
            DlmsData::Utf8String(_) => 0x0C,
            DlmsData::Bcd(_) => 0x0D,
            DlmsData::Integer(_) => 0x0F,
            DlmsData::Long(_) => 0x10,
            DlmsData::Unsigned(_) => 0x11,
            DlmsData::LongUnsigned(_) => 0x12,
            DlmsData::Long64(_) => 0x16,
            DlmsData::Long64Unsigned(_) => 0x17,
            DlmsData::Float(_) => 0x18,
            DlmsData::Double(_) => 0x19,
            DlmsData::DateTime(_) => 0x1A,
            DlmsData::Date(_) => 0x1B,
            DlmsData::Time(_) => 0x1C,
            DlmsData::Array(_) => 0x01,
            DlmsData::Structure(_) => 0x02,
            DlmsData::CompactArray { .. } => 0x1F,
            DlmsData::Enum(_) => 0x07,
            DlmsData::CompactArrayDefinition(_) => 0x1E,
        }
    }

    /// Decode a tag byte to data type
    pub fn from_tag(tag: u8) -> Result<DlmsDataType, DlmsDataError> {
        match tag {
            0x00 => Ok(DlmsDataType::None),
            0x01 => Ok(DlmsDataType::Array),
            0x02 => Ok(DlmsDataType::Structure),
            0x03 => Ok(DlmsDataType::Boolean),
            0x04 => Ok(DlmsDataType::BitString),
            0x05 => Ok(DlmsDataType::DoubleLong),
            0x06 => Ok(DlmsDataType::DoubleLongUnsigned),
            0x07 => Ok(DlmsDataType::Enum),
            0x09 => Ok(DlmsDataType::OctetString),
            0x0A => Ok(DlmsDataType::VisibleString),
            0x0C => Ok(DlmsDataType::Utf8String),
            0x0D => Ok(DlmsDataType::Bcd),
            0x0F => Ok(DlmsDataType::Integer),
            0x10 => Ok(DlmsDataType::Long),
            0x11 => Ok(DlmsDataType::Unsigned),
            0x12 => Ok(DlmsDataType::LongUnsigned),
            0x16 => Ok(DlmsDataType::Long64),
            0x17 => Ok(DlmsDataType::Long64Unsigned),
            0x18 => Ok(DlmsDataType::Float),
            0x19 => Ok(DlmsDataType::Double),
            0x1A => Ok(DlmsDataType::DateTime),
            0x1B => Ok(DlmsDataType::Date),
            0x1C => Ok(DlmsDataType::Time),
            0x1E => Ok(DlmsDataType::CompactArrayDefinition),
            0x1F => Ok(DlmsDataType::CompactArray),
            _ => Err(DlmsDataError::UnknownTag(tag)),
        }
    }

    /// Try to get as bool
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DlmsData::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Try to get as u8
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            DlmsData::Unsigned(v) => Some(*v),
            _ => None,
        }
    }

    /// Try to get as i32
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            DlmsData::DoubleLong(v) => Some(*v),
            DlmsData::Integer(v) => Some(*v as i32),
            DlmsData::Long(v) => Some(*v as i32),
            _ => None,
        }
    }

    /// Try to get as f64
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DlmsData::Double(v) => Some(*v),
            DlmsData::Float(v) => Some(*v as f64),
            DlmsData::DoubleLong(v) => Some(*v as f64),
            DlmsData::Long(v) => Some(*v as f64),
            DlmsData::Integer(v) => Some(*v as f64),
            DlmsData::Unsigned(v) => Some(*v as f64),
            DlmsData::LongUnsigned(v) => Some(*v as f64),
            DlmsData::DoubleLongUnsigned(v) => Some(*v as f64),
            _ => None,
        }
    }

    /// Try to get as string
    pub fn as_str(&self) -> Option<&str> {
        match self {
            DlmsData::VisibleString(s) => Some(s),
            DlmsData::Utf8String(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as octet string
    pub fn as_octet_string(&self) -> Option<&[u8]> {
        match self {
            DlmsData::OctetString(v) => Some(v),
            _ => None,
        }
    }

    /// Try to get as array
    pub fn as_array(&self) -> Option<&[DlmsData]> {
        match self {
            DlmsData::Array(v) => Some(v),
            _ => None,
        }
    }

    /// Try to get as structure
    pub fn as_structure(&self) -> Option<&[DlmsData]> {
        match self {
            DlmsData::Structure(v) => Some(v),
            _ => None,
        }
    }
}

/// DLMS data type identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DlmsDataType {
    None,
    Array,
    Structure,
    Boolean,
    BitString,
    DoubleLong,
    DoubleLongUnsigned,
    Enum,
    OctetString,
    VisibleString,
    Utf8String,
    Bcd,
    Integer,
    Long,
    Unsigned,
    LongUnsigned,
    Long64,
    Long64Unsigned,
    Float,
    Double,
    DateTime,
    Date,
    Time,
    CompactArrayDefinition,
    CompactArray,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DlmsDataError {
    UnknownTag(u8),
    InsufficientData,
    InvalidData,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_none_tag() {
        assert_eq!(DlmsData::None.tag(), 0x00);
    }

    #[test]
    fn test_boolean_tag() {
        assert_eq!(DlmsData::Boolean(true).tag(), 0x03);
    }

    #[test]
    fn test_double_long_tag() {
        assert_eq!(DlmsData::DoubleLong(42).tag(), 0x05);
    }

    #[test]
    fn test_octet_string_tag() {
        assert_eq!(DlmsData::OctetString(vec![1, 2, 3]).tag(), 0x09);
    }

    #[test]
    fn test_visible_string_tag() {
        assert_eq!(DlmsData::VisibleString("hello".into()).tag(), 0x0A);
    }

    #[test]
    fn test_array_tag() {
        assert_eq!(DlmsData::Array(vec![]).tag(), 0x01);
    }

    #[test]
    fn test_structure_tag() {
        assert_eq!(DlmsData::Structure(vec![]).tag(), 0x02);
    }

    #[test]
    fn test_from_tag_valid() {
        assert!(DlmsData::from_tag(0x03).is_ok());
    }

    #[test]
    fn test_from_tag_invalid() {
        assert!(DlmsData::from_tag(0xFE).is_err());
    }

    #[test]
    fn test_as_bool() {
        assert_eq!(DlmsData::Boolean(true).as_bool(), Some(true));
        assert_eq!(DlmsData::None.as_bool(), None);
    }

    #[test]
    fn test_as_u8() {
        assert_eq!(DlmsData::Unsigned(42).as_u8(), Some(42));
        assert_eq!(DlmsData::None.as_u8(), None);
    }

    #[test]
    fn test_as_i32() {
        assert_eq!(DlmsData::DoubleLong(-100).as_i32(), Some(-100));
        assert_eq!(DlmsData::Integer(-1).as_i32(), Some(-1));
    }

    #[test]
    fn test_as_f64() {
        let val = DlmsData::Float(3.14);
        assert!((val.as_f64().unwrap() - 3.14).abs() < 0.01);
    }

    #[test]
    fn test_as_str() {
        assert_eq!(DlmsData::VisibleString("hello".into()).as_str(), Some("hello"));
    }

    #[test]
    fn test_as_octet_string() {
        let data = DlmsData::OctetString(vec![1, 2, 3]);
        assert_eq!(data.as_octet_string(), Some(&[1, 2, 3][..]));
    }

    #[test]
    fn test_as_array() {
        let arr = DlmsData::Array(vec![DlmsData::None, DlmsData::Boolean(true)]);
        assert_eq!(arr.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_as_structure() {
        let s = DlmsData::Structure(vec![DlmsData::None]);
        assert_eq!(s.as_structure().unwrap().len(), 1);
    }

    #[test]
    fn test_clone() {
        let d = DlmsData::OctetString(vec![1, 2, 3]);
        let d2 = d.clone();
        assert_eq!(d, d2);
    }

    #[test]
    fn test_enum_data() {
        let e = DlmsData::Enum(1);
        assert_eq!(e.tag(), 0x07);
    }

    #[test]
    fn test_all_tags_unique() {
        let tags = [
            DlmsData::None.tag(),
            DlmsData::Boolean(true).tag(),
            DlmsData::BitString { unused_bits: 0, data: vec![] }.tag(),
            DlmsData::DoubleLong(0).tag(),
            DlmsData::DoubleLongUnsigned(0).tag(),
            DlmsData::OctetString(vec![]).tag(),
            DlmsData::VisibleString(String::new()).tag(),
            DlmsData::Utf8String(String::new()).tag(),
            DlmsData::Bcd(vec![]).tag(),
            DlmsData::Integer(0).tag(),
            DlmsData::Long(0).tag(),
            DlmsData::Unsigned(0).tag(),
            DlmsData::LongUnsigned(0).tag(),
            DlmsData::Long64(0).tag(),
            DlmsData::Long64Unsigned(0).tag(),
            DlmsData::Float(0.0).tag(),
            DlmsData::Double(0.0).tag(),
            DlmsData::DateTime([0; 12]).tag(),
            DlmsData::Date([0; 5]).tag(),
            DlmsData::Time([0; 4]).tag(),
            DlmsData::Array(vec![]).tag(),
            DlmsData::Structure(vec![]).tag(),
            DlmsData::CompactArray { header: vec![], data: vec![] }.tag(),
            DlmsData::Enum(0).tag(),
            DlmsData::CompactArrayDefinition(vec![]).tag(),
        ];
        let unique: std::collections::HashSet<u8> = tags.iter().copied().collect();
        assert_eq!(unique.len(), tags.len());
    }
}
