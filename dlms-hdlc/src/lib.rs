//! dlms-hdlc: HDLC framing for DLMS/COSEM (IEC 62056-53)
//!
//! Implements HDLC frame format, CRC-16, byte stuffing, I/S/U frames,
//! window mechanism, and stream-based frame parsing.

#![cfg_attr(not(feature = "std"), no_std)]

// no_std support: feature gate for no_std


use core::fmt;

mod crc;
mod frame;
mod parser;

pub use crc::{crc16_hdlc, crc16_hdlc_update};
pub use frame::{HdlcFrame, FrameType, ControlField, AddressField};
pub use parser::{HdlcParser, ParserState};

/// HDLC constants
pub const HDLC_FLAG: u8 = 0x7E;
pub const HDLC_ESCAPE: u8 = 0x7D;
pub const HDLC_ESCAPE_XOR: u8 = 0x20;
pub const HDLC_MAX_FRAME_SIZE: usize = 2048;

/// Perform HDLC byte stuffing on a frame (FCS must be included before calling)
pub fn stuff_bytes(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len() + data.len() / 4);
    for &b in data {
        match b {
            HDLC_FLAG | HDLC_ESCAPE => {
                result.push(HDLC_ESCAPE);
                result.push(b ^ HDLC_ESCAPE_XOR);
            }
            _ => result.push(b),
        }
    }
    result
}

/// Perform HDLC byte un-stuffing
pub fn unstuff_bytes(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len());
    let mut escaping = false;
    for &b in data {
        if escaping {
            result.push(b ^ HDLC_ESCAPE_XOR);
            escaping = false;
        } else if b == HDLC_ESCAPE {
            escaping = true;
        } else {
            result.push(b);
        }
    }
    result
}

/// Build a complete HDLC frame: Flag + Address + Control + Info + FCS + Flag
pub fn build_frame(address: u8, control: u8, info: &[u8]) -> Vec<u8> {
    let mut content = Vec::with_capacity(2 + info.len() + 2);
    content.push(address);
    content.push(control);
    content.extend_from_slice(info);

    let fcs = crc16_hdlc(&content);
    content.push((fcs & 0xFF) as u8);
    content.push((fcs >> 8) as u8);

    let mut frame = Vec::with_capacity(1 + content.len() * 2 + 1);
    frame.push(HDLC_FLAG);
    frame.extend(stuff_bytes(&content));
    frame.push(HDLC_FLAG);
    frame
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HdlcError {
    InvalidFrame,
    CrcError { expected: u16, actual: u16 },
    FrameTooLong,
    ParserError(String),
}

impl fmt::Display for HdlcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HdlcError::InvalidFrame => write!(f, "Invalid HDLC frame"),
            HdlcError::CrcError { expected, actual } => {
                write!(f, "CRC error: expected {expected:#06x}, got {actual:#06x}")
            }
            HdlcError::FrameTooLong => write!(f, "Frame exceeds maximum size"),
            HdlcError::ParserError(msg) => write!(f, "Parser error: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HdlcError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stuff_bytes() {
        assert_eq!(stuff_bytes(&[0x7E]), vec![0x7D, 0x5E]);
        assert_eq!(stuff_bytes(&[0x7D]), vec![0x7D, 0x5D]);
        assert_eq!(stuff_bytes(&[0x41]), vec![0x41]);
    }

    #[test]
    fn test_stuff_unstuff_roundtrip() {
        let data = vec![0x00, 0x7E, 0x7D, 0x41, 0x7E, 0xFF];
        let stuffed = stuff_bytes(&data);
        let unstuffed = unstuff_bytes(&stuffed);
        assert_eq!(data, unstuffed);
    }

    #[test]
    fn test_unstuff_bytes() {
        assert_eq!(unstuff_bytes(&[0x7D, 0x5E]), vec![0x7E]);
        assert_eq!(unstuff_bytes(&[0x7D, 0x5D]), vec![0x7D]);
    }

    #[test]
    fn test_unstuff_no_escape() {
        assert_eq!(unstuff_bytes(&[0x41, 0x42]), vec![0x41, 0x42]);
    }

    #[test]
    fn test_crc16_known() {
        // CRC-16/X.25 test
        let crc = crc16_hdlc(&[0x01, 0x02, 0x03, 0x04]);
        assert_ne!(crc, 0xFFFF);
        // Verify roundtrip: CRC of data should be the same
        assert_eq!(crc16_hdlc(&[0x01, 0x02, 0x03, 0x04]), crc);
    }

    #[test]
    fn test_crc16_empty() {
        assert_eq!(crc16_hdlc(&[]), 0xFFFF);
    }

    #[test]
    fn test_crc16_single() {
        let crc = crc16_hdlc(&[0x00]);
        assert_ne!(crc, 0xFFFF);
    }

    #[test]
    fn test_build_frame_minimal() {
        let frame = build_frame(0x03, 0xA0, &[]);
        assert_eq!(frame[0], HDLC_FLAG);
        assert_eq!(*frame.last().unwrap(), HDLC_FLAG);
    }

    #[test]
    fn test_build_frame_with_info() {
        let frame = build_frame(0x03, 0x10, &[0xE6, 0xE0, 0x00]);
        assert_eq!(frame[0], HDLC_FLAG);
        assert!(frame.len() > 6);
    }

    #[test]
    fn test_build_and_parse_frame() {
        let info = vec![0xE6, 0xE0, 0x00, 0x01, 0x02];
        let frame = build_frame(0x03, 0x10, &info);
        let mut parser = HdlcParser::new();
        let mut frames = Vec::new();
        for byte in &frame {
            if let Some(result) = parser.feed(*byte) {
                frames.push(result.unwrap());
            }
        }
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].address.value(), 0x03);
        assert_eq!(frames[0].info, info);
    }

    #[test]
    fn test_frame_type_detection() {
        // I-frame: control bit 0 = 0
        assert!(matches!(FrameType::from_control(0x00), FrameType::I { .. }));
        assert!(matches!(FrameType::from_control(0x10), FrameType::I { .. }));
        // S-frame: control = 0x01
        assert!(matches!(FrameType::from_control(0x01), FrameType::S { .. }));
        // U-frame: control = 0x03
        assert!(matches!(FrameType::from_control(0x03), FrameType::U { .. }));
    }

    #[test]
    fn test_control_field_i_frame() {
        let cf = ControlField::from_byte(0x26);
        // 0x26 = 0b00100110: bit0=0 (I), N(S)=0b001=1, N(R)=0b011=3
        // Actually let's use a cleaner value
        let cf2 = ControlField::from_byte(0x00);
        assert!(matches!(cf2.frame_type(), FrameType::I { send_seq: 0, recv_seq: 0 }));
    }

    #[test]
    fn test_control_field_s_frame() {
        let cf = ControlField::from_byte(0x95);
        assert!(matches!(cf.frame_type(), FrameType::S { .. }));
    }

    #[test]
    fn test_control_field_u_frame() {
        let cf = ControlField::from_byte(0x73);
        assert!(matches!(cf.frame_type(), FrameType::U { .. }));
    }

    #[test]
    fn test_address_field() {
        let addr = AddressField::from_byte(0x03);
        assert_eq!(addr.value(), 0x03);
        assert!(!addr.is_broadcast());
    }

    #[test]
    fn test_address_broadcast() {
        let addr = AddressField::from_byte(0x81);
        assert!(addr.is_broadcast());
    }

    #[test]
    fn test_parser_initial_state() {
        let parser = HdlcParser::new();
        assert!(matches!(parser.state(), ParserState::Idle));
    }

    #[test]
    fn test_parser_multiple_frames() {
        let frame1 = build_frame(0x03, 0x73, &[]); // SNRM
        let frame2 = build_frame(0x03, 0x10, &[0x01]);
        let mut combined = frame1.clone();
        combined.extend_from_slice(&frame2);
        let mut parser = HdlcParser::new();
        let mut count = 0;
        for byte in &combined {
            if let Some(result) = parser.feed(*byte) {
                count += 1;
            }
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_parser_byte_by_byte() {
        let frame = build_frame(0x03, 0x10, &[0xAA, 0xBB]);
        let mut parser = HdlcParser::new();
        let mut got = false;
        for byte in &frame {
            if let Some(result) = parser.feed(*byte) {
                got = true;
            }
        }
        assert!(got);
    }

    #[test]
    fn test_parser_crc_error() {
        let mut frame = build_frame(0x03, 0x10, &[0x01, 0x02]);
        // Corrupt a byte in the middle
        let mid = frame.len() / 2;
        frame[mid] ^= 0xFF;
        let mut parser = HdlcParser::new();
        for byte in &frame {
            if let Some(result) = parser.feed(*byte) {
                assert!(result.is_err());
            }
        }
    }

    #[test]
    fn test_hdlc_flag_constant() {
        assert_eq!(HDLC_FLAG, 0x7E);
    }

    #[test]
    fn test_hdlc_escape_constant() {
        assert_eq!(HDLC_ESCAPE, 0x7D);
    }

    #[test]
    fn test_crc16_update() {
        let mut crc = 0xFFFF;
        crc = crc16_hdlc_update(crc, 0x01);
        crc = crc16_hdlc_update(crc, 0x02);
        assert_eq!(crc, crc16_hdlc(&[0x01, 0x02]));
    }

    #[test]
    fn test_stuff_bytes_empty() {
        assert!(stuff_bytes(&[]).is_empty());
    }

    #[test]
    fn test_unstuff_bytes_empty() {
        assert!(unstuff_bytes(&[]).is_empty());
    }

    #[test]
    fn test_stuff_bytes_no_special() {
        let data = vec![0x01, 0x02, 0x41, 0x42];
        assert_eq!(stuff_bytes(&data), data);
    }

    #[test]
    fn test_build_frame_content() {
        let frame = build_frame(0x03, 0xA0, &[]);
        // Flag + Address + Control + FCS(2) + Flag = 6 minimum
        assert!(frame.len() >= 6);
    }

    #[test]
    fn test_frame_equality() {
        let f1 = HdlcFrame {
            address: AddressField::from_byte(0x03),
            control: ControlField::from_byte(0x10),
            info: vec![0x01, 0x02],
        };
        let f2 = HdlcFrame {
            address: AddressField::from_byte(0x03),
            control: ControlField::from_byte(0x10),
            info: vec![0x01, 0x02],
        };
        assert_eq!(f1, f2);
    }

    #[test]
    fn test_control_field_send_seq() {
        // I-frame: N(S)=3, N(R)=0, P=0
        // N(S) in bits 3-1: 0b011, bit0=0 → 0b0110 = 0x06
        let cf = ControlField::from_byte(0x06);
        if let FrameType::I { send_seq, .. } = cf.frame_type() {
            assert_eq!(send_seq, 3);
        } else {
            panic!("Expected I-frame");
        }
    }

    #[test]
    fn test_control_field_recv_seq() {
        // I-frame: N(S)=0, N(R)=1, P=0
        // N(R)=1 in bits 7-5: 0b001_0_0000 = 0x20
        let cf = ControlField::from_byte(0x20);
        if let FrameType::I { recv_seq, .. } = cf.frame_type() {
            assert_eq!(recv_seq, 1);
        } else {
            panic!("Expected I-frame");
        }
    }

    #[test]
    fn test_address_equality() {
        let a = AddressField::from_byte(0x03);
        let b = AddressField::from_byte(0x03);
        assert_eq!(a, b);
    }

    #[test]
    fn test_address_inequality() {
        let a = AddressField::from_byte(0x03);
        let b = AddressField::from_byte(0x10);
        assert_ne!(a, b);
    }

    #[test]
    fn test_parser_state_debug() {
        let state = ParserState::Idle;
        assert_eq!(format!("{:?}", state), "Idle");
    }

    #[test]
    fn test_hdlc_error_display() {
        let err = HdlcError::InvalidFrame;
        assert!(!format!("{err}").is_empty());
    }

    #[test]
    fn test_hdlc_error_crc_display() {
        let err = HdlcError::CrcError { expected: 0x1234, actual: 0x5678 };
        let s = format!("{err}");
        assert!(s.contains("CRC"));
    }

    #[test]
    fn test_parser_reset() {
        let mut parser = HdlcParser::new();
        parser.feed(0x7E);
        parser.feed(0x03);
        parser.reset();
        assert!(matches!(parser.state(), ParserState::Idle));
    }

    #[test]
    fn test_s_frame_rr() {
        let cf = ControlField::from_byte(0x01); // RR, N(R)=0
        if let FrameType::S { s_type, recv_seq } = cf.frame_type() {
            assert_eq!(recv_seq, 0);
            assert_eq!(s_type, 0);
        } else {
            panic!("Expected S-frame");
        }
    }

    #[test]
    fn test_u_frame_snrm() {
        // SNRM in DLMS: 0x93
        let cf = ControlField::from_byte(0x93);
        if let FrameType::U { u_type, poll_final } = cf.frame_type() {
            // modifier extraction from DLMS HDLC
            assert_eq!(u_type, (0x93 >> 2) & 0x07);
            assert!(poll_final);
        } else {
            panic!("Expected U-frame");
        }
    }

    #[test]
    fn test_u_frame_ua() {
        // UA in DLMS: 0x73
        let cf = ControlField::from_byte(0x73);
        if let FrameType::U { u_type, poll_final } = cf.frame_type() {
            assert_eq!(u_type, (0x73 >> 2) & 0x07);
            assert!(poll_final);
        } else {
            panic!("Expected U-frame");
        }
    }

    #[test]
    fn test_build_frame_info_preserved() {
        let info = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let frame = build_frame(0x03, 0x10, &info);
        let mut parser = HdlcParser::new();
        for byte in &frame {
            if let Some(result) = parser.feed(*byte) {
                assert_eq!(result.unwrap().info, info);
                return;
            }
        }
        panic!("No frame parsed");
    }

    #[test]
    fn test_crc16_sequential() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05];
        let crc = crc16_hdlc(&data);
        assert_eq!(crc, crc16_hdlc(&data)); // deterministic
    }

    #[test]
    fn test_stuff_bytes_all_special() {
        let data = vec![0x7E, 0x7D, 0x7E, 0x7D];
        let stuffed = stuff_bytes(&data);
        let unstuffed = unstuff_bytes(&stuffed);
        assert_eq!(data, unstuffed);
    }

    #[test]
    fn test_control_field_poll_final() {
        // I-frame with poll: bit 4 set
        // N(S)=1: bits 3-1 = 001, bit0=0 → lower byte = 0b0010 = 0x02, with P=1: 0x12
        let cf = ControlField::from_byte(0x12);
        assert!(cf.poll());
        // Without poll: 0x02
        let cf2 = ControlField::from_byte(0x02);
        assert!(!cf2.poll());
    }

    #[test]
    fn test_address_from_to_bytes() {
        let addr = AddressField::from_byte(0x03);
        assert_eq!(addr.to_byte(), 0x03);
    }
}
