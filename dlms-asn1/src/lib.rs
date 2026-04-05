//! dlms-asn1: ASN.1 BER/DER encoding for DLMS/COSEM ACSE layer
//!
//! Implements AARQ (Association Request), AARE (Association Response),
//! RLRQ (Release Request), RLRE (Release Response) as defined in IEC 62056-53.

#![cfg_attr(not(feature = "std"), no_std)]

// no_std support: feature gate for no_std

use core::fmt;

mod aare;
mod aarq;
mod ber;
mod rlre;
mod rlrq;

#[cfg(test)]
mod property_tests;

pub use aare::AareApdu;
pub use aarq::AarqApdu;
pub use ber::{BerDecoder, BerEncoder, BerTag};
pub use rlre::RlreApdu;
pub use rlrq::RlrqApdu;

/// Application context names
pub mod app_context {
    pub const ACSE_DLMS: &[u8] = &[0x60, 0x85, 0x74, 0x05, 0x08, 0x01, 0x01];
    pub const ACSE_DLMS_NO_CIPHER: &[u8] = &[0x60, 0x85, 0x74, 0x05, 0x08, 0x02, 0x01];
    pub const ACSE_DLMS_AES128: &[u8] = &[0x60, 0x85, 0x74, 0x05, 0x08, 0x02, 0x02];
    pub const ACSE_DLMS_AES128_GMAC: &[u8] = &[0x60, 0x85, 0x74, 0x05, 0x08, 0x02, 0x03];
    pub const ACSE_DLMS_AES128_GCM: &[u8] = &[0x60, 0x85, 0x74, 0x05, 0x08, 0x02, 0x04];
    pub const ACSE_DLMS_SM4_GMAC: &[u8] = &[0x60, 0x85, 0x74, 0x05, 0x08, 0x02, 0x05];
    pub const ACSE_DLMS_ECDSA: &[u8] = &[0x60, 0x85, 0x74, 0x05, 0x08, 0x02, 0x06];
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Asn1Error {
    InvalidTag,
    InvalidLength,
    InsufficientData,
    UnexpectedEof,
    InvalidData(String),
}

#[cfg(feature = "std")]
impl std::error::Error for Asn1Error {}

impl fmt::Display for Asn1Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Asn1Error::InvalidTag => write!(f, "Invalid ASN.1 tag"),
            Asn1Error::InvalidLength => write!(f, "Invalid length"),
            Asn1Error::InsufficientData => write!(f, "Insufficient data"),
            Asn1Error::UnexpectedEof => write!(f, "Unexpected end of data"),
            Asn1Error::InvalidData(msg) => write!(f, "Invalid data: {msg}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ber_tag_class() {
        let tag = BerTag::new(0x60);
        assert_eq!(tag.class(), BerTag::APPLICATION);
    }

    #[test]
    fn test_ber_tag_number() {
        let tag = BerTag::new(0x02);
        assert_eq!(tag.number(), 2);
    }

    #[test]
    fn test_ber_tag_constructed() {
        let tag = BerTag::new(0x60); // Application, constructed
        assert!(tag.is_constructed());
    }

    #[test]
    fn test_ber_tag_primitive() {
        let tag = BerTag::new(0x02); // Integer, primitive
        assert!(!tag.is_constructed());
    }

    #[test]
    fn test_ber_encode_length() {
        assert_eq!(BerEncoder::encode_length(0), vec![0x00]);
        assert_eq!(BerEncoder::encode_length(127), vec![0x7F]);
        assert_eq!(BerEncoder::encode_length(128), vec![0x81, 0x80]);
        assert_eq!(BerEncoder::encode_length(256), vec![0x82, 0x01, 0x00]);
    }

    #[test]
    fn test_ber_decode_length() {
        let (len, consumed) = BerDecoder::decode_length(&[0x00]).unwrap();
        assert_eq!(len, 0);
        assert_eq!(consumed, 1);
        let (len, consumed) = BerDecoder::decode_length(&[0x81, 0x80]).unwrap();
        assert_eq!(len, 128);
        assert_eq!(consumed, 2);
    }

    #[test]
    fn test_ber_encode_integer() {
        let bytes = BerEncoder::encode_integer(42);
        assert_eq!(bytes, vec![0x02, 0x01, 0x2A]);
    }

    #[test]
    fn test_ber_decode_integer() {
        let (val, consumed) = BerDecoder::decode_integer(&[0x02, 0x01, 0x2A]).unwrap();
        assert_eq!(val, 42);
        assert_eq!(consumed, 3);
    }

    #[test]
    fn test_ber_encode_octet_string() {
        let bytes = BerEncoder::encode_octet_string(&[0x01, 0x02, 0x03]);
        assert_eq!(bytes, vec![0x04, 0x03, 0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_ber_decode_octet_string() {
        let (val, consumed) =
            BerDecoder::decode_octet_string(&[0x04, 0x03, 0x01, 0x02, 0x03]).unwrap();
        assert_eq!(val, vec![0x01, 0x02, 0x03]);
        assert_eq!(consumed, 5);
    }

    #[test]
    fn test_ber_encode_sequence() {
        let bytes = BerEncoder::encode_sequence(&[0x02, 0x01, 0x2A]);
        assert_eq!(bytes[0], 0x30);
    }

    #[test]
    fn test_aarq_basic() {
        let aarq = AarqApdu {
            protocol_version: 1,
            application_context_name: app_context::ACSE_DLMS_NO_CIPHER.to_vec(),
            called_ap_title: None,
            calling_ap_title: None,
            mechanism_name: None,
            user_information: None,
        };
        let bytes = aarq.encode();
        assert!(!bytes.is_empty());
        assert_eq!(bytes[0], 0x60); // Application 0, constructed
    }

    #[test]
    fn test_aare_basic() {
        let aare = AareApdu {
            result: 0, // accepted
            result_source_diagnostic: None,
            application_context_name: app_context::ACSE_DLMS_NO_CIPHER.to_vec(),
            responding_ap_title: None,
            mechanism_name: None,
            user_information: None,
        };
        let bytes = aare.encode();
        assert!(!bytes.is_empty());
        assert_eq!(bytes[0], 0x61); // Application 1, constructed
    }

    #[test]
    fn test_rlrq_basic() {
        let rlrq = RlrqApdu { reason: 0 };
        let bytes = rlrq.encode();
        assert!(!bytes.is_empty());
        assert_eq!(bytes[0], 0x62); // Application 2, constructed
    }

    #[test]
    fn test_rlre_basic() {
        let rlre = RlreApdu { result: 0 };
        let bytes = rlre.encode();
        assert!(!bytes.is_empty());
        assert_eq!(bytes[0], 0x63); // Application 3, constructed
    }

    #[test]
    fn test_aarq_decode() {
        let aarq = AarqApdu {
            protocol_version: 1,
            application_context_name: app_context::ACSE_DLMS_NO_CIPHER.to_vec(),
            called_ap_title: None,
            calling_ap_title: None,
            mechanism_name: None,
            user_information: None,
        };
        let bytes = aarq.encode();
        let decoded = AarqApdu::decode(&bytes).unwrap();
        assert_eq!(decoded.protocol_version, 1);
    }

    #[test]
    fn test_aare_decode() {
        let aare = AareApdu {
            result: 0,
            result_source_diagnostic: None,
            application_context_name: app_context::ACSE_DLMS_NO_CIPHER.to_vec(),
            responding_ap_title: None,
            mechanism_name: None,
            user_information: None,
        };
        let bytes = aare.encode();
        let decoded = AareApdu::decode(&bytes).unwrap();
        assert_eq!(decoded.result, 0);
    }

    #[test]
    fn test_asn1_error_display() {
        let err = Asn1Error::InvalidTag;
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_ber_encode_empty_sequence() {
        let bytes = BerEncoder::encode_sequence(&[]);
        assert_eq!(bytes, vec![0x30, 0x00]);
    }

    #[test]
    fn test_ber_encode_oid() {
        let oid = vec![0x60, 0x85, 0x74, 0x05, 0x08];
        let bytes = BerEncoder::encode_oid(&oid);
        assert_eq!(bytes[0], 0x06);
    }

    #[test]
    fn test_ber_decode_oid() {
        let encoded = BerEncoder::encode_oid(&[0x60, 0x85, 0x74, 0x05, 0x08]);
        let (val, _) = BerDecoder::decode_oid(&encoded).unwrap();
        assert_eq!(val, vec![0x60, 0x85, 0x74, 0x05, 0x08]);
    }

    #[test]
    fn test_app_context_constants() {
        assert!(!app_context::ACSE_DLMS.is_empty());
        assert!(!app_context::ACSE_DLMS_NO_CIPHER.is_empty());
        assert!(!app_context::ACSE_DLMS_AES128.is_empty());
    }

    #[test]
    fn test_ber_tag_equality() {
        let a = BerTag::new(0x60);
        let b = BerTag::new(0x60);
        assert_eq!(a, b);
    }

    #[test]
    fn test_ber_tag_from_context() {
        let tag = BerTag::context_constructed(0);
        assert_eq!(tag.0, 0xA0);
    }
}
