//! HDLC Parameter Negotiation according to Green Book Edition 9
//!
//! Implements parameter negotiation for SNRM/UA frames as specified in
//! Green Book section 8.4.5.3.2.

use alloc::vec::Vec;
use core::fmt;

/// Format identifier for SNRM/UA information field
pub const FORMAT_IDENTIFIER: u8 = 0x81;

/// Group identifier for SNRM/UA information field
pub const GROUP_IDENTIFIER: u8 = 0x80;

/// Parameter IDs according to Green Book Edition 9
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ParameterType {
    /// Maximum information field length - transmit (128-2048)
    MaxInfoFieldLengthTx = 0x05,
    /// Maximum information field length - receive (128-2048)
    MaxInfoFieldLengthRx = 0x06,
    /// Window size - transmit (1-7)
    WindowSizeTx = 0x07,
    /// Window size - receive (1-7)
    WindowSizeRx = 0x08,
}

/// HDLC parameters with separate TX/RX values according to Green Book
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlcParametersGreen {
    /// Maximum information field length for transmission (128-2048)
    pub max_info_length_tx: u16,
    /// Maximum information field length for reception (128-2048)
    pub max_info_length_rx: u16,
    /// Window size for transmission (1-7)
    pub window_size_tx: u8,
    /// Window size for reception (1-7)
    pub window_size_rx: u8,
}

impl Default for HdlcParametersGreen {
    fn default() -> Self {
        Self {
            max_info_length_tx: 128,
            max_info_length_rx: 128,
            window_size_tx: 1,
            window_size_rx: 1,
        }
    }
}

impl HdlcParametersGreen {
    /// Validate parameter ranges
    pub fn validate(&self) -> Result<(), HdlcParameterError> {
        if self.window_size_tx < 1 || self.window_size_tx > 7 {
            return Err(HdlcParameterError::InvalidRange {
                parameter: "WindowSizeTx",
                value: self.window_size_tx as u32,
                min: 1,
                max: 7,
            });
        }
        if self.window_size_rx < 1 || self.window_size_rx > 7 {
            return Err(HdlcParameterError::InvalidRange {
                parameter: "WindowSizeRx",
                value: self.window_size_rx as u32,
                min: 1,
                max: 7,
            });
        }
        if self.max_info_length_tx < 128 || self.max_info_length_tx > 2048 {
            return Err(HdlcParameterError::InvalidRange {
                parameter: "MaxInfoLengthTx",
                value: self.max_info_length_tx as u32,
                min: 128,
                max: 2048,
            });
        }
        if self.max_info_length_rx < 128 || self.max_info_length_rx > 2048 {
            return Err(HdlcParameterError::InvalidRange {
                parameter: "MaxInfoLengthRx",
                value: self.max_info_length_rx as u32,
                min: 128,
                max: 2048,
            });
        }
        Ok(())
    }

    /// Encode parameters as Green Book format
    ///
    /// Format: 0x81, 0x80, group_length, parameters...
    pub fn encode(&self) -> Result<Vec<u8>, HdlcParameterError> {
        self.validate()?;

        let mut params_buf = Vec::new();

        // Encode MaxInfoLengthTx (0x05)
        params_buf.push(ParameterType::MaxInfoFieldLengthTx as u8);
        if self.max_info_length_tx <= 255 {
            params_buf.push(0x01); // length = 1
            params_buf.push(self.max_info_length_tx as u8);
        } else {
            params_buf.push(0x02); // length = 2
            params_buf.push((self.max_info_length_tx >> 8) as u8);
            params_buf.push((self.max_info_length_tx & 0xFF) as u8);
        }

        // Encode MaxInfoLengthRx (0x06)
        params_buf.push(ParameterType::MaxInfoFieldLengthRx as u8);
        if self.max_info_length_rx <= 255 {
            params_buf.push(0x01); // length = 1
            params_buf.push(self.max_info_length_rx as u8);
        } else {
            params_buf.push(0x02); // length = 2
            params_buf.push((self.max_info_length_rx >> 8) as u8);
            params_buf.push((self.max_info_length_rx & 0xFF) as u8);
        }

        // Encode WindowSizeTx (0x07)
        params_buf.push(ParameterType::WindowSizeTx as u8);
        params_buf.push(0x01); // length = 1
        params_buf.push(self.window_size_tx);

        // Encode WindowSizeRx (0x08)
        params_buf.push(ParameterType::WindowSizeRx as u8);
        params_buf.push(0x01); // length = 1
        params_buf.push(self.window_size_rx);

        // Build complete frame with header
        let mut result = Vec::with_capacity(3 + params_buf.len());
        result.push(FORMAT_IDENTIFIER);
        result.push(GROUP_IDENTIFIER);
        result.push(params_buf.len() as u8);
        result.extend(params_buf);

        Ok(result)
    }

    /// Parse parameters from Green Book format
    pub fn parse(data: &[u8]) -> Result<Self, HdlcParameterError> {
        if data.len() < 3 {
            return Err(HdlcParameterError::TooShort {
                expected: 3,
                actual: data.len(),
            });
        }

        // Check format and group identifiers
        if data[0] != FORMAT_IDENTIFIER || data[1] != GROUP_IDENTIFIER {
            return Err(HdlcParameterError::InvalidHeader {
                format_id: data[0],
                group_id: data[1],
            });
        }

        let group_length = data[2] as usize;
        if data.len() < 3 + group_length {
            return Err(HdlcParameterError::TooShort {
                expected: 3 + group_length,
                actual: data.len(),
            });
        }

        let mut params = Self::default();
        let mut i = 3;

        while i + 1 < data.len() {
            let tag = data[i];
            let length = data[i + 1] as usize;
            i += 2;

            if i + length > data.len() {
                return Err(HdlcParameterError::ValueOverflow);
            }

            let value_data = &data[i..i + length];
            i += length;

            match tag {
                0x05 => {
                    // MaxInfoLengthTx
                    params.max_info_length_tx = if length == 1 {
                        value_data[0] as u16
                    } else if length >= 2 {
                        ((value_data[0] as u16) << 8) | (value_data[1] as u16)
                    } else {
                        continue;
                    };
                }
                0x06 => {
                    // MaxInfoLengthRx
                    params.max_info_length_rx = if length == 1 {
                        value_data[0] as u16
                    } else if length >= 2 {
                        ((value_data[0] as u16) << 8) | (value_data[1] as u16)
                    } else {
                        continue;
                    };
                }
                0x07 => {
                    // WindowSizeTx
                    if length >= 1 {
                        params.window_size_tx = value_data[0];
                    }
                }
                0x08 => {
                    // WindowSizeRx
                    if length >= 1 {
                        params.window_size_rx = value_data[0];
                    }
                }
                _ => {
                    // Unknown parameter, skip
                }
            }
        }

        params.validate()?;
        Ok(params)
    }

    /// Negotiate parameters between client and server
    ///
    /// Note: Client TX parameters correspond to server RX and vice versa
    pub fn negotiate(client: &Self, server: &Self) -> Self {
        Self {
            // Client TX with server RX
            max_info_length_tx: client.max_info_length_tx.min(server.max_info_length_rx),
            window_size_tx: client.window_size_tx.min(server.window_size_rx),
            // Client RX with server TX
            max_info_length_rx: client.max_info_length_rx.min(server.max_info_length_tx),
            window_size_rx: client.window_size_rx.min(server.window_size_tx),
        }
    }
}

/// HDLC parameter errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HdlcParameterError {
    /// Invalid header bytes
    InvalidHeader { format_id: u8, group_id: u8 },
    /// Parameter value out of range
    InvalidRange {
        parameter: &'static str,
        value: u32,
        min: u32,
        max: u32,
    },
    /// Data too short
    TooShort { expected: usize, actual: usize },
    /// Value extends beyond data
    ValueOverflow,
}

impl fmt::Display for HdlcParameterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHeader { format_id, group_id } => {
                write!(
                    f,
                    "Invalid header: got {:02x} {:02x}, expected {:02x} {:02x}",
                    format_id, group_id, FORMAT_IDENTIFIER, GROUP_IDENTIFIER
                )
            }
            Self::InvalidRange {
                parameter,
                value,
                min,
                max,
            } => {
                write!(
                    f,
                    "{} value {} out of range [{}, {}]",
                    parameter, value, min, max
                )
            }
            Self::TooShort { expected, actual } => {
                write!(f, "Data too short: expected {} bytes, got {}", expected, actual)
            }
            Self::ValueOverflow => write!(f, "Parameter value extends beyond data"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for HdlcParameterError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_type_constants() {
        assert_eq!(ParameterType::MaxInfoFieldLengthTx as u8, 0x05);
        assert_eq!(ParameterType::MaxInfoFieldLengthRx as u8, 0x06);
        assert_eq!(ParameterType::WindowSizeTx as u8, 0x07);
        assert_eq!(ParameterType::WindowSizeRx as u8, 0x08);
    }

    #[test]
    fn test_default_parameters() {
        let params = HdlcParametersGreen::default();
        assert_eq!(params.max_info_length_tx, 128);
        assert_eq!(params.max_info_length_rx, 128);
        assert_eq!(params.window_size_tx, 1);
        assert_eq!(params.window_size_rx, 1);
    }

    #[test]
    fn test_validation_valid() {
        let params = HdlcParametersGreen {
            max_info_length_tx: 512,
            max_info_length_rx: 1024,
            window_size_tx: 3,
            window_size_rx: 5,
        };
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_validation_invalid_window_size() {
        let mut params = HdlcParametersGreen::default();
        params.window_size_tx = 0;
        assert!(params.validate().is_err());

        params.window_size_tx = 8;
        assert!(params.validate().is_err());
    }

    #[test]
    fn test_validation_invalid_max_info_length() {
        let mut params = HdlcParametersGreen::default();
        params.max_info_length_tx = 127;
        assert!(params.validate().is_err());

        params.max_info_length_tx = 2049;
        assert!(params.validate().is_err());
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let original = HdlcParametersGreen {
            max_info_length_tx: 512,
            max_info_length_rx: 1024,
            window_size_tx: 3,
            window_size_rx: 5,
        };

        let encoded = original.encode().unwrap();
        let decoded = HdlcParametersGreen::parse(&encoded).unwrap();

        assert_eq!(decoded.max_info_length_tx, original.max_info_length_tx);
        assert_eq!(decoded.max_info_length_rx, original.max_info_length_rx);
        assert_eq!(decoded.window_size_tx, original.window_size_tx);
        assert_eq!(decoded.window_size_rx, original.window_size_rx);
    }

    #[test]
    fn test_encode_large_values() {
        let params = HdlcParametersGreen {
            max_info_length_tx: 2048,
            max_info_length_rx: 1024,
            window_size_tx: 7,
            window_size_rx: 7,
        };

        let encoded = params.encode().unwrap();
        let decoded = HdlcParametersGreen::parse(&encoded).unwrap();

        assert_eq!(decoded.max_info_length_tx, 2048);
        assert_eq!(decoded.max_info_length_rx, 1024);
    }

    #[test]
    fn test_encode_header() {
        let params = HdlcParametersGreen::default();
        let encoded = params.encode().unwrap();

        assert_eq!(encoded[0], FORMAT_IDENTIFIER);
        assert_eq!(encoded[1], GROUP_IDENTIFIER);
        assert!(encoded.len() > 3);
    }

    #[test]
    fn test_negotiate() {
        let client = HdlcParametersGreen {
            max_info_length_tx: 1024,
            max_info_length_rx: 2048,
            window_size_tx: 5,
            window_size_rx: 7,
        };

        let server = HdlcParametersGreen {
            max_info_length_tx: 512,
            max_info_length_rx: 512,
            window_size_tx: 7,
            window_size_rx: 3,
        };

        let negotiated = HdlcParametersGreen::negotiate(&client, &server);

        // Client TX (1024) with server RX (512) -> 512
        assert_eq!(negotiated.max_info_length_tx, 512);
        // Client TX window (5) with server RX window (3) -> 3
        assert_eq!(negotiated.window_size_tx, 3);
        // Client RX (2048) with server TX (512) -> 512
        assert_eq!(negotiated.max_info_length_rx, 512);
        // Client RX window (7) with server TX window (7) -> 7
        assert_eq!(negotiated.window_size_rx, 7);
    }

    #[test]
    fn test_green_book_example() {
        // Test against Green Book example with default values
        let params = HdlcParametersGreen {
            max_info_length_tx: 128,
            max_info_length_rx: 128,
            window_size_tx: 1,
            window_size_rx: 1,
        };

        let encoded = params.encode().unwrap();

        // Check header
        assert_eq!(encoded[0], 0x81);
        assert_eq!(encoded[1], 0x80);

        // Parse and verify
        let decoded = HdlcParametersGreen::parse(&encoded).unwrap();
        assert_eq!(decoded.max_info_length_tx, 128);
    }
}
