//! Property-based tests for ASN.1 BER encoding/decoding
//!
//! These tests use proptest to verify roundtrip properties:
//! - Encoding then decoding should return the original value
//! - Properties hold for arbitrary valid inputs

#[cfg(test)]
mod tests {
    use crate::{BerDecoder, BerEncoder};
    use proptest::prelude::*;

    // ============= Length Encoding/Decoding =============

    proptest! {
        #[test]
        fn prop_length_roundtrip(len in 0usize..65535) {
            let encoded = BerEncoder::encode_length(len);
            let (decoded, consumed) = BerDecoder::decode_length(&encoded)
                .expect("decode_length should succeed");
            prop_assert_eq!(decoded, len);
            prop_assert_eq!(consumed, encoded.len());
        }

        #[test]
        fn prop_length_deterministic(len in 0usize..10000) {
            let e1 = BerEncoder::encode_length(len);
            let e2 = BerEncoder::encode_length(len);
            prop_assert_eq!(e1, e2);
        }
    }

    // ============= Integer Encoding/Decoding =============

    proptest! {
        #[test]
        fn prop_integer_roundtrip(value in -1000000i32..1000000) {
            let encoded = BerEncoder::encode_integer(value);
            let (decoded, consumed) = BerDecoder::decode_integer(&encoded)
                .expect("decode_integer should succeed");
            prop_assert_eq!(decoded, value);
            prop_assert_eq!(consumed, encoded.len());
        }

        #[test]
        fn prop_integer_deterministic(value in any::<i32>()) {
            let e1 = BerEncoder::encode_integer(value);
            let e2 = BerEncoder::encode_integer(value);
            prop_assert_eq!(e1, e2);
        }

        #[test]
        fn prop_integer_different_values_different_encoding(v1 in any::<i32>(), v2 in any::<i32>()) {
            prop_assume!(v1 != v2);
            let e1 = BerEncoder::encode_integer(v1);
            let e2 = BerEncoder::encode_integer(v2);
            prop_assert_ne!(e1, e2);
        }
    }

    // ============= Octet String Encoding/Decoding =============

    proptest! {
        #[test]
        fn prop_octet_string_roundtrip(data in proptest::collection::vec(any::<u8>(), 0..256)) {
            let encoded = BerEncoder::encode_octet_string(&data);
            let (decoded, consumed) = BerDecoder::decode_octet_string(&encoded)
                .expect("decode_octet_string should succeed");
            prop_assert_eq!(decoded, data);
            prop_assert_eq!(consumed, encoded.len());
        }

        #[test]
        fn prop_octet_string_deterministic(data in proptest::collection::vec(any::<u8>(), 0..100)) {
            let e1 = BerEncoder::encode_octet_string(&data);
            let e2 = BerEncoder::encode_octet_string(&data);
            prop_assert_eq!(e1, e2);
        }
    }

    // ============= OID Encoding/Decoding =============

    proptest! {
        #[test]
        fn prop_oid_roundtrip(oid in proptest::collection::vec(any::<u8>(), 1..64)) {
            let encoded = BerEncoder::encode_oid(&oid);
            let (decoded, consumed) = BerDecoder::decode_oid(&encoded)
                .expect("decode_oid should succeed");
            prop_assert_eq!(decoded, oid);
            prop_assert_eq!(consumed, encoded.len());
        }
    }

    // ============= Sequence Encoding =============

    proptest! {
        #[test]
        fn prop_sequence_contains_content(content in proptest::collection::vec(any::<u8>(), 0..256)) {
            let encoded = BerEncoder::encode_sequence(&content);
            // First byte should be 0x30 (SEQUENCE tag)
            prop_assert_eq!(encoded[0], 0x30);
            // The content should be present somewhere in the encoding
            if !content.is_empty() {
                let content_start = encoded.len() - content.len();
                prop_assert_eq!(&encoded[content_start..], &content[..]);
            }
        }
    }

    // ============= TLV Encoding/Decoding =============

    proptest! {
        #[test]
        fn prop_tlv_roundtrip(tag in any::<u8>(), value in proptest::collection::vec(any::<u8>(), 0..256)) {
            let encoded = BerEncoder::encode_tlv(tag, &value);
            let (decoded_tag, decoded_value, consumed) = BerDecoder::decode_tlv(&encoded)
                .expect("decode_tlv should succeed");
            prop_assert_eq!(decoded_tag, tag);
            prop_assert_eq!(decoded_value, value);
            prop_assert_eq!(consumed, encoded.len());
        }
    }

    // ============= Edge Cases =============

    #[test]
    fn prop_empty_octet_string_roundtrip() {
        let data: Vec<u8> = vec![];
        let encoded = BerEncoder::encode_octet_string(&data);
        let (decoded, _) =
            BerDecoder::decode_octet_string(&encoded).expect("empty octet string should decode");
        assert_eq!(decoded, data);
    }

    #[test]
    fn prop_zero_integer_roundtrip() {
        let value: i32 = 0;
        let encoded = BerEncoder::encode_integer(value);
        let (decoded, _) = BerDecoder::decode_integer(&encoded).expect("zero should decode");
        assert_eq!(decoded, value);
    }

    #[test]
    fn prop_max_integer_roundtrip() {
        let value: i32 = i32::MAX;
        let encoded = BerEncoder::encode_integer(value);
        let (decoded, _) = BerDecoder::decode_integer(&encoded).expect("max integer should decode");
        assert_eq!(decoded, value);
    }

    #[test]
    fn prop_min_integer_roundtrip() {
        let value: i32 = i32::MIN;
        let encoded = BerEncoder::encode_integer(value);
        let (decoded, _) = BerDecoder::decode_integer(&encoded).expect("min integer should decode");
        assert_eq!(decoded, value);
    }
}
