//! Stream-based HDLC frame parser (Green Book Ed.9 Chapter 8)
//!
//! Parses HDLC Frame Format Type 3 frames with proper Format field and HCS

// no_std support
use alloc::vec::Vec;

use crate::crc::crc16_hdlc;
use crate::frame::{ControlField, FrameFormat, HdlcAddress, HdlcFrame};
use crate::{HdlcError, HDLC_FLAG, HDLC_MAX_FRAME_SIZE};

/// Parser state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParserState {
    Idle,
    InFrame,
    Escaped,
}

/// Stream-based HDLC frame parser
pub struct HdlcParser {
    state: ParserState,
    buffer: Vec<u8>,
}

impl HdlcParser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Idle,
            buffer: Vec::with_capacity(HDLC_MAX_FRAME_SIZE),
        }
    }

    /// Feed a byte into the parser. Returns Some(Ok(frame)) when a complete frame is parsed,
    /// Some(Err) on CRC/parse error, None otherwise.
    pub fn feed(&mut self, byte: u8) -> Option<Result<HdlcFrame, HdlcError>> {
        match byte {
            HDLC_FLAG => {
                match self.state {
                    ParserState::Idle => {
                        self.state = ParserState::InFrame;
                        self.buffer.clear();
                        None
                    }
                    ParserState::InFrame | ParserState::Escaped => {
                        // Try to parse frame
                        let frame = self.parse_buffer();
                        self.state = ParserState::Idle;
                        self.buffer.clear();
                        Some(frame)
                    }
                }
            }
            _ => {
                if self.state == ParserState::Idle {
                    return None;
                }

                if self.buffer.len() >= HDLC_MAX_FRAME_SIZE {
                    self.state = ParserState::Idle;
                    self.buffer.clear();
                    return None;
                }

                if byte == 0x7D {
                    self.state = ParserState::Escaped;
                } else if matches!(self.state, ParserState::Escaped) {
                    self.buffer.push(byte ^ 0x20);
                    self.state = ParserState::InFrame;
                } else {
                    self.buffer.push(byte);
                }
                None
            }
        }
    }

    pub fn state(&self) -> ParserState {
        self.state
    }

    pub fn reset(&mut self) {
        self.state = ParserState::Idle;
        self.buffer.clear();
    }

    /// Parse the buffer into an HdlcFrame
    fn parse_buffer(&self) -> Result<HdlcFrame, HdlcError> {
        if self.buffer.len() < 7 {
            return Err(HdlcError::InvalidFrame);
        }

        // Parse format field (first 2 bytes)
        let format = FrameFormat::parse(&[self.buffer[0], self.buffer[1]])
            .map_err(|e| HdlcError::ParserError(e.to_string()))?;

        // Parse destination address
        let (dest_address, _offset) = HdlcAddress::parse(&self.buffer[2..])
            .map_err(|e| HdlcError::ParserError(e.to_string()))?;

        // Parse source address
        let dest_len = dest_address.encoded_length();
    let src_start = 2 + dest_len;
        if src_start >= self.buffer.len() {
            return Err(HdlcError::InvalidFrame);
        }

        let (src_address, _) = HdlcAddress::parse(&self.buffer[src_start..])
            .map_err(|e| HdlcError::ParserError(e.to_string()))?;

        let addr_end = src_start + src_address.encoded_length();

        // Parse control field
        if addr_end >= self.buffer.len() {
            return Err(HdlcError::InvalidFrame);
        }
        let control = ControlField::from_byte(self.buffer[addr_end]);

        // Calculate header end (Format + Dest + Src + Control)
        let header_end = addr_end + 1;

        // Extract HCS (2 bytes after header)
        let hcs_offset = header_end;
        if hcs_offset + 2 > self.buffer.len() {
            return Err(HdlcError::InvalidFrame);
        }
        let hcs_bytes = &self.buffer[hcs_offset..hcs_offset + 2];
        let received_hcs = (hcs_bytes[1] as u16) << 8 | hcs_bytes[0] as u16;

        // Verify HCS (CRC of Format + Dest + Src + Control)
        let header = &self.buffer[..header_end];
        let computed_hcs = crc16_hdlc(header);
        if computed_hcs != received_hcs {
            return Err(HdlcError::CrcError {
                expected: computed_hcs,
                actual: received_hcs,
            });
        }

        // Extract FCS (last 2 bytes)
        let info_end = self.buffer.len() - 2;
        let fcs_bytes = &self.buffer[info_end..];
        let received_fcs = (fcs_bytes[1] as u16) << 8 | fcs_bytes[0] as u16;

        // Extract information field (between HCS and FCS)
        let info_start = hcs_offset + 2;
        let info = if info_start < info_end {
            self.buffer[info_start..info_end].to_vec()
        } else {
            Vec::new()
        };

        // Verify FCS (CRC of Dest + Src + Control + Info)
        let content = &self.buffer[2..info_end];
        let computed_fcs = crc16_hdlc(content);
        if computed_fcs != received_fcs {
            return Err(HdlcError::CrcError {
                expected: computed_fcs,
                actual: received_fcs,
            });
        }

        Ok(HdlcFrame {
            format,
            dest_address,
            src_address,
            control,
            hcs: received_hcs,
            info,
            fcs: received_fcs,
        })
    }
}

impl Default for HdlcParser {
    fn default() -> Self {
        Self::new()
    }
}
