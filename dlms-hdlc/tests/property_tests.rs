//! Property-based tests using proptest

use dlms_hdlc::*;
use proptest::prelude::*;

// ============================================================
// CRC-16/X.25 properties
// ============================================================

proptest! {
    #[test]
    fn crc_is_deterministic(data in prop::collection::vec(any::<u8>(), 0..100)) {
        let crc1 = crc16_hdlc(&data);
        let crc2 = crc16_hdlc(&data);
        prop_assert_eq!(crc1, crc2);
    }

    #[test]
    fn crc_incremental_matches_batch(data in prop::collection::vec(any::<u8>(), 0..100)) {
        let batch = crc16_hdlc(&data);
        let mut incremental = 0xFFFF;
        for b in &data {
            incremental = crc16_hdlc_update(incremental, *b);
        }
        prop_assert_eq!(batch, incremental);
    }

    #[test]
    fn crc_nontrivial_for_nonempty(data in prop::collection::vec(any::<u8>(), 1..50)) {
        let crc = crc16_hdlc(&data);
        prop_assert_ne!(crc, 0x0000);
        prop_assert_ne!(crc, 0xFFFF);
    }
}

// ============================================================
// Byte stuffing properties
// ============================================================

proptest! {
    #[test]
    fn stuff_roundtrip(data in prop::collection::vec(any::<u8>(), 0..500)) {
        let stuffed = stuff_bytes(&data);
        let unstuffed = unstuff_bytes(&stuffed);
        prop_assert_eq!(&data[..], &unstuffed[..data.len()]);
    }

    #[test]
    fn stuffed_needs_no_flag_or_escape(bytes in prop::collection::vec(0u8..0x7D, 0..20)) {
        let stuffed = stuff_bytes(&bytes);
        prop_assert_eq!(stuffed, bytes);
    }

    #[test]
    fn stuffed_contains_no_raw_flags(data in prop::collection::vec(any::<u8>(), 0..200)) {
        let stuffed = stuff_bytes(&data);
        for b in &stuffed {
            prop_assert_ne!(*b, HDLC_FLAG, "Stuffed data should never contain 0x7E");
        }
    }
}

// ============================================================
// Frame build/parse roundtrip
// ============================================================

proptest! {
    #[test]
    fn frame_build_parse_roundtrip(address in any::<u8>(), control in any::<u8>(),
                                      info in prop::collection::vec(any::<u8>(), 0..100)) {
        let frame = build_frame_simple(address, control, &info);
        let mut parser = HdlcParser::new();
        let mut got_frame = false;
        for byte in &frame {
            if let Some(result) = parser.feed(*byte) {
                prop_assert!(result.is_ok());
                let f = result.unwrap();
                // Check dest_address matches (new API uses dest_address instead of address)
                if let HdlcAddress::OneByte { address: addr } = f.dest_address {
                    prop_assert_eq!(addr, address);
                }
                prop_assert_eq!(f.control.to_byte(), control);
                prop_assert_eq!(f.info.len(), info.len());
                if f.info.len() == info.len() && !info.is_empty() {
                    prop_assert_eq!(&f.info[..], &info[..]);
                }
                got_frame = true;
            }
        }
        prop_assert!(got_frame, "Frame should have been parsed");
    }
}

// ============================================================
// AddressField properties (updated for HdlcAddress API)
// ============================================================

proptest! {
    #[test]
    fn address_roundtrip(byte in any::<u8>()) {
        let addr = HdlcAddress::one_byte(byte);
        let encoded = addr.encode();
        let (parsed, _) = HdlcAddress::parse(&encoded).unwrap();
        prop_assert_eq!(parsed, addr);
    }

    #[test]
    fn broadcast_bit_isolated(byte in any::<u8>()) {
        let addr = HdlcAddress::one_byte(byte);
        // Broadcast if byte == 0x7F (127)
        let is_broadcast = byte == 0x7F;
        prop_assert_eq!(addr.is_broadcast(), is_broadcast);
    }
}

// ============================================================
// ControlField properties
// ============================================================

proptest! {
    #[test]
    fn control_roundtrip(byte in any::<u8>()) {
        let cf = ControlField::from_byte(byte);
        prop_assert_eq!(cf.to_byte(), byte);
    }
}
