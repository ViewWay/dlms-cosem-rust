//! Stream-based HDLC frame parser

// no_std support
use std::vec::Vec;

use crate::{crc16_hdlc, HDLC_FLAG, HDLC_MAX_FRAME_SIZE, HdlcError, HdlcFrame};
use crate::frame::{AddressField, ControlField};

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
    /// Some(Err) on CRC error, None otherwise.
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
                        if self.buffer.len() < 4 {
                            // Need at least: address(1) + control(1) + FCS(2)
                            self.state = ParserState::Idle;
                            self.buffer.clear();
                            return None;
                        }
                        let frame = self.parse_buffer();
                        self.state = ParserState::Idle;
                        self.buffer.clear();
                        Some(frame)
                    }
                }
            }
            _ => {
                if matches!(self.state, ParserState::Idle) {
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

    fn parse_buffer(&self) -> Result<HdlcFrame, HdlcError> {
        if self.buffer.len() < 4 {
            return Err(HdlcError::InvalidFrame);
        }

        let len = self.buffer.len();
        let content = &self.buffer[..len - 2];
        let fcs_bytes = &self.buffer[len - 2..];

        let computed_fcs = crc16_hdlc(content);
        let received_fcs = (fcs_bytes[1] as u16) << 8 | fcs_bytes[0] as u16;

        if computed_fcs != received_fcs {
            return Err(HdlcError::CrcError {
                expected: computed_fcs,
                actual: received_fcs,
            });
        }

        if content.len() < 2 {
            return Err(HdlcError::InvalidFrame);
        }

        let address = AddressField::from_byte(content[0]);
        let control = ControlField::from_byte(content[1]);
        let info = if content.len() > 2 {
            content[2..].to_vec()
        } else {
            Vec::new()
        };

        Ok(HdlcFrame { address, control, info })
    }
}

impl Default for HdlcParser {
    fn default() -> Self {
        Self::new()
    }
}
