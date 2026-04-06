//! dlms-hdlc: HDLC framing for DLMS/COSEM (Green Book Ed.9 Chapter 8)
//!
//! Implements HDLC Frame Format Type 3 as specified in IEC 62056-53
//! with proper Format field, HCS/FCS checksums, and multi-byte addresses.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use core::fmt;

mod block_transfer;
mod crc;
mod frame;
mod parser;
mod wport;

pub use block_transfer::{
    Block, BlockTransfer, BlockTransferError, BlockTransferState, TransferProgress,
};
pub use crc::{crc16_hdlc, crc16_hdlc_update};
 pub use frame::{
    s_frame, u_frame, ControlField, FrameFormat, FrameType, HdlcAddress, HdlcFrame, HdlcParameters,
};
pub use parser::{HdlcParser, ParserState};
pub use wport::{
    WPORT_DLMS_COSEM_TCP, WPORT_DLMS_COSEM_UDP,
    WPORT_ALL_STATION, WPORT_CLIENT_MGMT_PROCESS, WPORT_MGMT_LOGICAL_DEVICE,
    WPORT_NO_STATION, WPORT_PUBLIC_CLIENT,
    get_wport_description, is_reserved_wport,
};

/// HDLC constants
pub const HDLC_FLAG: u8 = 0x7E;
pub const HDLC_ESCAPE: u8 = 0x7D;
pub const HDLC_ESCAPE_XOR: u8 = 0x20;
pub const HDLC_MAX_FRAME_SIZE: usize = 2048;

/// Perform HDLC byte stuffing on a frame
pub fn stuff_bytes(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len() + data.len() / 4);
 {
        for &b in data {
            match b {
                HDLC_FLAG | HDLC_ESCAPE => {
                    result.push(HDLC_ESCAPE);
                    result.push(b ^ HDLC_ESCAPE_XOR);
                }
                _ => result.push(b),
            }
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

/// Build a complete HDLC Frame Format Type 3 frame
///
/// Frame structure:
/// - Flag (0x7E)
/// - Frame Format (2 bytes): Type=0xA, Segmentation bit, Length (11 bits)
/// - Destination Address (1-4 bytes)
/// - Source Address (1-4 bytes)
/// - Control (1 byte)
/// - HCS (2 bytes): CRC-16 of Format + Addresses + Control
/// - Information (optional)
/// - FCS (2 bytes): CRC-16 of Addresses + Control + Information
/// - Flag (0x7E)
pub fn build_frame(
    segmented: bool,
    dest_address: &HdlcAddress,
    src_address: &HdlcAddress,
    control: &ControlField,
    info: &[u8],
) -> Vec<u8> {
    // Calculate frame length (excludes flags)
    let frame_len = 2 // Format field
        + dest_address.encoded_length()
        + src_address.encoded_length()
        + 1 // Control
        + 2 // HCS
        + info.len()
        + 2; // FCS

    let format = FrameFormat::new(segmented, frame_len as u16);

    // Build header (Format + Dest + Src + Control)
    let mut header = Vec::new();
    header.extend_from_slice(&format.encode());
    header.extend(dest_address.encode());
    header.extend(src_address.encode());
    header.push(control.to_byte());

    // Calculate HCS (CRC of header)
    let hcs = crc16_hdlc(&header);

    // Build content (Dest + Src + Control + Info) for FCS
    let mut content = Vec::new();
    content.extend(dest_address.encode());
    content.extend(src_address.encode());
    content.push(control.to_byte());
    content.extend_from_slice(info);

    // Calculate FCS (CRC of content)
    let fcs = crc16_hdlc(&content);

    // Build complete frame (before stuffing)
    let mut frame = Vec::new();
    frame.extend_from_slice(&header);
    frame.push((hcs & 0xFF) as u8);
    frame.push((hcs >> 8) as u8);
    if !info.is_empty() {
        frame.extend_from_slice(info);
    }
    frame.push((fcs & 0xFF) as u8);
    frame.push((fcs >> 8) as u8);

    // Add flags and perform stuffing
    let mut result = Vec::with_capacity(frame.len() + 2);
    result.push(HDLC_FLAG);
    result.extend(stuff_bytes(&frame));
    result.push(HDLC_FLAG);

    result
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
    fn test_crc16_known() {
        let crc = crc16_hdlc(&[0x01, 0x02, 0x03, 0x04]);
        assert_ne!(crc, 0xFFFF);
    }

    #[test]
    fn test_crc16_empty() {
        assert_eq!(crc16_hdlc(&[]), 0xFFFF);
    }

    #[test]
    fn test_build_frame_minimal() {
        let dest = HdlcAddress::one_byte(0x03);
        let src = HdlcAddress::one_byte(0x01);
        let ctrl = ControlField::u_frame(u_frame::SNRM, true);
        let frame = build_frame(false, &dest, &src, &ctrl, &[]);
        assert_eq!(frame[0], HDLC_FLAG);
        assert_eq!(*frame.last().unwrap(), HDLC_FLAG);
    }

    #[test]
    fn test_build_and_parse_frame() {
        let info = vec![0xE6, 0xE0, 0x00, 0x01, 0x02];
        let dest = HdlcAddress::one_byte(0x03);
        let src = HdlcAddress::one_byte(0x01);
        let ctrl = ControlField::i_frame(0, 0, false);
        let frame = build_frame(false, &dest, &src, &ctrl, &info);

        let mut parser = HdlcParser::new();
        let mut frames = Vec::new();
        for byte in &frame {
            if let Some(result) = parser.feed(*byte) {
                frames.push(result.unwrap());
            }
        }
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].dest_address, dest);
        assert_eq!(frames[0].info, info);
    }

    #[test]
    fn test_frame_type_detection() {
        // I-frame: control bit 0 = 0
        assert!(matches!(FrameType::from_control(0x00), FrameType::I { .. }));
        // S-frame: control bits 1:0 = 01
        assert!(matches!(FrameType::from_control(0x01), FrameType::S { .. }));
        // U-frame: control bits 1:0 = 11
        assert!(matches!(FrameType::from_control(0x03), FrameType::U { .. }));
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
    fn test_parser_initial_state() {
        let parser = HdlcParser::new();
        assert!(matches!(parser.state(), ParserState::Idle));
    }

    #[test]
    fn test_parser_multiple_frames() {
        let dest = HdlcAddress::one_byte(0x03);
        let src = HdlcAddress::one_byte(0x01);
        let ctrl_ua = ControlField::u_frame(u_frame::UA, true);
        let ctrl_i = ControlField::i_frame(0, 0, false);

        let frame1 = build_frame(false, &dest, &src, &ctrl_ua, &[]);
        let frame2 = build_frame(false, &dest, &src, &ctrl_i, &[0x01]);

        let mut combined = frame1.clone();
        combined.extend_from_slice(&frame2);

        let mut parser = HdlcParser::new();
        let mut count = 0;
        for byte in &combined {
            if parser.feed(*byte).is_some() {
                count += 1;
            }
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_parser_byte_by_byte() {
        let dest = HdlcAddress::one_byte(0x03);
        let src = HdlcAddress::one_byte(0x01);
        let ctrl = ControlField::i_frame(0, 0, false);
        let frame = build_frame(false, &dest, &src, &ctrl, &[0xAA, 0xBB]);

        let mut parser = HdlcParser::new();
        let mut got = false;
        for byte in &frame {
            if parser.feed(*byte).is_some() {
                got = true;
            }
        }
        assert!(got);
    }

    #[test]
    fn test_parser_crc_error() {
        let dest = HdlcAddress::one_byte(0x03);
        let src = HdlcAddress::one_byte(0x01);
        let ctrl = ControlField::i_frame(0, 0, false);
        let mut frame = build_frame(false, &dest, &src, &ctrl, &[0x01, 0x02]);

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
    fn test_parser_reset() {
        let mut parser = HdlcParser::new();
        parser.feed(0x7E);
        parser.feed(0x03);
        parser.reset();
        assert!(matches!(parser.state(), ParserState::Idle));
    }

    #[test]
    fn test_hdlc_error_display() {
        let err = HdlcError::InvalidFrame;
        assert!(!format!("{}", err).is_empty());
    }

    #[test]
    fn test_hdlc_error_crc_display() {
        let err = HdlcError::CrcError {
            expected: 0x1234,
            actual: 0x5678,
        };
        let s = format!("{}", err);
        assert!(s.contains("CRC"));
    }

    #[test]
    fn test_frame_format() {
        let ff = FrameFormat::new(false, 0x123);
        assert_eq!(ff.format_type, 0xA);
        assert!(!ff.segmented);
        assert_eq!(ff.length, 0x123);
    }

    #[test]
    fn test_frame_format_encode_decode() {
        let ff = FrameFormat::new(true, 0xFF);
        let encoded = ff.encode();
        let decoded = FrameFormat::parse(&encoded).unwrap();
        assert!(decoded.segmented);
        assert_eq!(decoded.length, 0xFF);
    }

    #[test]
    fn test_address_encode_decode() {
        let addr = HdlcAddress::one_byte(0x10);
        let encoded = addr.encode();
        assert_eq!(encoded, vec![0x21]);

        let (parsed, _) = HdlcAddress::parse(&encoded).unwrap();
        assert_eq!(parsed, addr);
    }

    #[test]
    fn test_address_two_byte() {
        let addr = HdlcAddress::two_byte(0x01, 0x7F);
        let encoded = addr.encode();
        assert_eq!(addr.encoded_length(), 2);

        let (parsed, _) = HdlcAddress::parse(&encoded).unwrap();
        assert_eq!(parsed, addr);
    }

    #[test]
    fn test_control_field_i_frame() {
        let cf = ControlField::i_frame(3, 5, true);
        if let FrameType::I { send_seq, recv_seq } = cf.frame_type() {
            assert_eq!(send_seq, 3);
            assert_eq!(recv_seq, 5);
        } else {
            panic!("Expected I-frame");
        }
        assert!(cf.poll_final());
    }

    #[test]
    fn test_control_field_s_frame() {
        let cf = ControlField::s_frame(s_frame::RR, 2, true);
        if let FrameType::S { s_type, recv_seq } = cf.frame_type() {
            assert_eq!(s_type, s_frame::RR);
            assert_eq!(recv_seq, 2);
        } else {
            panic!("Expected S-frame");
        }
    }

    #[test]
    fn test_control_field_u_frame() {
        let cf = ControlField::u_frame(u_frame::SNRM, true);
        assert!(u_frame::is_snrm(cf.raw));
        if let FrameType::U { u_type: _, poll_final } = cf.frame_type() {
            assert!(poll_final);
        } else {
            panic!("Expected U-frame");
        }
    }

    #[test]
    fn test_hdlc_parameters() {
        let params = HdlcParameters {
            window_size: 7,
            max_info_length: 256,
        };
        let encoded = params.encode();
        assert!(!encoded.is_empty());

        let decoded = HdlcParameters::parse(&encoded).unwrap();
        assert_eq!(decoded.window_size, 7);
        assert_eq!(decoded.max_info_length, 256);
    }

    #[test]
    fn test_build_frame_with_info() {
        let info = vec![0xDE, 0xAD, 0xBE, 0xEF];
        let dest = HdlcAddress::one_byte(0x03);
        let src = HdlcAddress::one_byte(0x01);
        let ctrl = ControlField::i_frame(0, 0, false);
        let frame = build_frame(false, &dest, &src, &ctrl, &info);

        let mut parser = HdlcParser::new();
        for byte in &frame {
            if let Some(result) = parser.feed(*byte) {
                let parsed = result.unwrap();
                assert_eq!(parsed.info, info);
                return;
            }
        }
        panic!("No frame parsed");
    }

    #[test]
    fn test_frame_format_segmented() {
        let ff = FrameFormat::new(true, 100);
        assert!(ff.segmented);

        let ff2 = FrameFormat::new(false, 100);
        assert!(!ff2.segmented);
    }

    #[test]
    fn test_address_broadcast() {
        let addr = HdlcAddress::one_byte(0x7F);
        assert!(addr.is_broadcast());

        let addr2 = HdlcAddress::one_byte(0x10);
        assert!(!addr2.is_broadcast());
    }

    #[test]
    fn test_parser_default() {
        let parser = HdlcParser::default();
        assert!(matches!(parser.state(), ParserState::Idle));
    }
}
