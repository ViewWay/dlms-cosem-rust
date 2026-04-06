//! HDLC frame types and fields according to Green Book Ed.9 Chapter 8
//!
//! Implements Frame Format Type 3 as specified in IEC 62056-53

/// HDLC frame type (I/S/U frames)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    /// Information frame: send_seq (N(S)), recv_seq (N(R))
    I { send_seq: u8, recv_seq: u8 },
    /// Supervisory frame: type (0=RR, 1=RNR, 2=REJ, 3=SREJ), recv_seq (N(R))
    S { s_type: u8, recv_seq: u8 },
    /// Unnumbered frame: type (modifier bits), poll/final bit
    U { u_type: u8, poll_final: bool },
}

impl FrameType {
    /// Parse frame type from control byte
    pub fn from_control(control: u8) -> Self {
        if control & 0x01 == 0 {
            // I-frame: bit0=0, N(S) bits 3-1, N(R) bits 7-5, P/F bit 4
            FrameType::I {
                send_seq: (control >> 1) & 0x07,
                recv_seq: (control >> 5) & 0x07,
            }
        } else if control & 0x02 == 0 {
            // S-frame: bits 1-0=01, type bits 4-3, N(R) bits 7-5
            FrameType::S {
                s_type: (control >> 2) & 0x03,
                recv_seq: (control >> 5) & 0x07,
            }
        } else {
            // U-frame: bits 1-0=11, P/F bit 4
            FrameType::U {
                u_type: control & !0x10, // base value without P/F
                poll_final: (control >> 4) & 0x01 == 1,
            }
        }
    }

    /// Convert frame type to control byte
    pub fn to_control(&self) -> u8 {
        match self {
            FrameType::I { send_seq, recv_seq } => {
                // N(R) bits 7-5, P/F bit 4, N(S) bits 3-1, bit0=0
                ((*recv_seq & 0x07) << 5) | ((*send_seq & 0x07) << 1)
            }
            FrameType::S { s_type, recv_seq } => {
                // N(R) bits 7-5, P/F bit 4, S-type bits 3-2, bits 1-0=01
                0x01 | ((*recv_seq & 0x07) << 5) | ((*s_type & 0x03) << 2)
            }
            FrameType::U { u_type, poll_final } => {
                // modifier bits 5-2, P/F bit 4, bits 1-0=11
                0x03 | ((*u_type & 0x0F) << 2) | ((*poll_final as u8) << 4)
            }
        }
    }
}

/// U-frame types (modifier bits)
pub mod u_frame {
    /// Base value with P/F=0: 0x83
    pub const SNRM: u8 = 0x83;  // Set Normal Response Mode
    /// Base value with P/F=0: 0x43
    pub const DISC: u8 = 0x43;  // Disconnect
    /// Base value with P/F=0: 0x63
    pub const UA: u8 = 0x63;    // Unnumbered Acknowledge
    const DM: u8 = 0x0F;        // Disconnected Mode
    const FRMR: u8 = 0x87;      // Frame Reject
    const UI: u8 = 0x03;        // Unnumbered Information

    pub fn is_snrm(control: u8) -> bool {
        (control & 0xEF) == 0x83
    }

    pub fn is_disc(control: u8) -> bool {
        (control & 0xEF) == 0x43
    }

    pub fn is_ua(control: u8) -> bool {
        (control & 0xEF) == 0x63
    }

    pub fn is_dm(control: u8) -> bool {
        (control & 0xEF) == 0x0F
    }

    pub fn is_ui(control: u8) -> bool {
        (control & 0xEF) == 0x03
    }
}

/// S-frame types
pub mod s_frame {
    pub const RR: u8 = 0x00;   // Receiver Ready
    pub const RNR: u8 = 0x01;  // Receiver Not Ready
    pub const REJ: u8 = 0x02;  // Reject
    const SREJ: u8 = 0x03;     // Selective Reject
}

/// HDLC control field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlField {
    pub raw: u8,
}

impl ControlField {
    pub fn from_byte(byte: u8) -> Self {
        Self { raw: byte }
    }

    pub fn frame_type(&self) -> FrameType {
        FrameType::from_control(self.raw)
    }

    pub fn poll_final(&self) -> bool {
        (self.raw >> 4) & 0x01 == 1
    }

    pub fn to_byte(&self) -> u8 {
        self.raw
    }

    /// Create I-frame control field
    pub fn i_frame(send_seq: u8, recv_seq: u8, poll_final: bool) -> Self {
        let raw = ((recv_seq & 0x07) << 5)
            | ((poll_final as u8) << 4)
            | ((send_seq & 0x07) << 1);
        Self { raw }
    }

    /// Create S-frame control field
    pub fn s_frame(s_type: u8, recv_seq: u8, poll_final: bool) -> Self {
        let raw = ((recv_seq & 0x07) << 5)
            | ((poll_final as u8) << 4)
            | ((s_type & 0x03) << 2)
            | 0x01;
        Self { raw }
    }

    /// Create U-frame control field
    /// Create U-frame control field from base value (P/F=0) and poll_final flag
    pub fn u_frame(base: u8, poll_final: bool) -> Self {
        let raw = if poll_final { base | 0x10 } else { base & !0x10 };
        Self { raw }
    }
}

/// HDLC address representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HdlcAddress {
    /// Single-byte address (client or server upper address only)
    OneByte { address: u8 },
    /// Two-byte address (server upper + lower, both 1 byte each)
    TwoByte { upper: u8, lower: u8 },
    /// Four-byte address (server upper + lower, both 2 bytes each)
    FourByte { upper: u16, lower: u16 },
}

impl HdlcAddress {
    /// Create a single-byte address
    pub fn one_byte(address: u8) -> Self {
        HdlcAddress::OneByte { address }
    }

    /// Create a two-byte address
    pub fn two_byte(upper: u8, lower: u8) -> Self {
        HdlcAddress::TwoByte { upper, lower }
    }

    /// Create a four-byte address
    pub fn four_byte(upper: u16, lower: u16) -> Self {
        HdlcAddress::FourByte { upper, lower }
    }

    /// Check if this is a broadcast address
    pub fn is_broadcast(&self) -> bool {
        match self {
            HdlcAddress::OneByte { address } => *address == 0x7F,
            HdlcAddress::TwoByte { upper, lower } => *upper == 0x7F && *lower == 0x7F,
            HdlcAddress::FourByte { upper, lower } => *upper == 0x3FFF && *lower == 0x3FFF,
        }
    }

    /// Get the encoded length in bytes
    pub fn encoded_length(&self) -> usize {
        match self {
            HdlcAddress::OneByte { .. } => 1,
            HdlcAddress::TwoByte { .. } => 2,
            HdlcAddress::FourByte { .. } => 4,
        }
    }

    /// Encode address to bytes (with extension bits)
    pub fn encode(&self) -> Vec<u8> {
        match self {
            HdlcAddress::OneByte { address } => {
                // LSB=1 indicates end of address
                vec![(*address << 1) | 0x01]
            }
            HdlcAddress::TwoByte { upper, lower } => {
                // First byte: LSB=0 (extension), second byte: LSB=1 (end)
                vec![
                    (*upper << 1) & 0xFE,
                    (*lower << 1) | 0x01,
                ]
            }
            HdlcAddress::FourByte { upper, lower } => {
                // 4 bytes with extension bits
                let upper_high = (*upper >> 6) as u8;
                let upper_low = (*upper & 0x3F) as u8;
                let lower_high = (*lower >> 6) as u8;
                let lower_low = (*lower & 0x3F) as u8;
                vec![
                    (upper_high << 1) & 0xFE,
                    (upper_low << 1) & 0xFE,
                    (lower_high << 1) & 0xFE,
                    (lower_low << 1) | 0x01,
                ]
            }
        }
    }

    /// Parse address from bytes, returns (address, bytes_consumed)
    pub fn parse(data: &[u8]) -> Result<(Self, usize), String> {
        if data.is_empty() {
            return Err("Empty address field".to_string());
        }

        // Count how many bytes until LSB=1
        let mut len = 0;
        for (i, &byte) in data.iter().enumerate() {
            if byte & 0x01 == 1 {
                len = i + 1;
                break;
            }
        }

        if len == 0 {
            return Err("No end marker in address field".to_string());
        }

        if len > 4 {
            return Err(format!("Address too long: {} bytes", len));
        }

        let address = match len {
            1 => {
                let addr = data[0] >> 1;
                HdlcAddress::OneByte { address: addr }
            }
            2 => {
                let upper = data[0] >> 1;
                let lower = data[1] >> 1;
                HdlcAddress::TwoByte { upper, lower }
            }
            4 => {
                let upper = ((data[0] as u16) << 7) | ((data[1] as u16) >> 1);
                let lower = ((data[2] as u16) << 7) | ((data[3] as u16) >> 1);
                HdlcAddress::FourByte { upper, lower }
            }
            _ => return Err(format!("Invalid address length: {}", len)),
        };

        Ok((address, len))
    }
}

/// Frame format field (2 bytes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameFormat {
    /// Format type (should be 0xA for Type 3)
    pub format_type: u8,
    /// Segmentation bit
    pub segmented: bool,
    /// Frame length (11 bits, excludes flags)
    pub length: u16,
}

impl FrameFormat {
    const TYPE_3: u8 = 0xA;

    /// Create a new frame format field
    pub fn new(segmented: bool, length: u16) -> Self {
        Self {
            format_type: Self::TYPE_3,
            segmented,
            length: length & 0x7FF, // 11 bits max
        }
    }

    /// Encode to 2 bytes
    pub fn encode(&self) -> [u8; 2] {
        // Format type (4 bits) | Segmentation (1 bit) | Length high 3 bits
        // Length low 8 bits
        let byte1 = (self.format_type << 4)
            | ((self.segmented as u8) << 3)
            | ((self.length >> 8) as u8 & 0x07);
        let byte2 = (self.length & 0xFF) as u8;
        [byte1, byte2]
    }

    /// Parse from 2 bytes
    pub fn parse(data: &[u8; 2]) -> Result<Self, String> {
        let format_type = (data[0] >> 4) & 0x0F;
        if format_type != Self::TYPE_3 {
            return Err(format!("Invalid format type: {:#X}", format_type));
        }
        let segmented = (data[0] & 0x08) != 0;
        let length = (((data[0] & 0x07) as u16) << 8) | (data[1] as u16);
        Ok(Self {
            format_type,
            segmented,
            length,
        })
    }
}

/// Complete HDLC frame (Frame Format Type 3)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlcFrame {
    /// Frame format field
    pub format: FrameFormat,
    /// Destination address
    pub dest_address: HdlcAddress,
    /// Source address
    pub src_address: HdlcAddress,
    /// Control field
    pub control: ControlField,
    /// Header check sequence (CRC-16 of header)
    pub hcs: u16,
    /// Information field (optional)
    pub info: Vec<u8>,
    /// Frame check sequence (CRC-16 of entire frame)
    pub fcs: u16,
}

impl HdlcFrame {
    /// Create a new HDLC frame
    pub fn new(
        segmented: bool,
        dest_address: HdlcAddress,
        src_address: HdlcAddress,
        control: ControlField,
        info: Vec<u8>,
    ) -> Self {
        // Calculate frame length (excludes flags)
        // Format(2) + addresses + Control(1) + HCS(2) + info + FCS(2)
        let length = 2 + dest_address.encoded_length() + src_address.encoded_length()
            + 1 + 2 + info.len() + 2;

        let format = FrameFormat::new(segmented, length as u16);

        Self {
            format,
            dest_address,
            src_address,
            control,
            hcs: 0, // Will be calculated
            info,
            fcs: 0, // Will be calculated
        }
    }

    /// Get frame type
    pub fn frame_type(&self) -> FrameType {
        self.control.frame_type()
    }

    /// Check if this is an I-frame
    pub fn is_i_frame(&self) -> bool {
        matches!(self.frame_type(), FrameType::I { .. })
    }

    /// Check if this is an S-frame
    pub fn is_s_frame(&self) -> bool {
        matches!(self.frame_type(), FrameType::S { .. })
    }

    /// Check if this is a U-frame
    pub fn is_u_frame(&self) -> bool {
        matches!(self.frame_type(), FrameType::U { .. })
    }
}

/// HDLC parameters for negotiation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlcParameters {
    /// Window size (1-7)
    pub window_size: u8,
    /// Maximum information field length
    pub max_info_length: u16,
}

impl Default for HdlcParameters {
    fn default() -> Self {
        Self {
            window_size: 1,
            max_info_length: 128,
        }
    }
}

impl HdlcParameters {
    /// Encode parameters as TLV for SNRM/UA frames
    pub fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();

        // Window size - transmit (tag 0x07)
        result.push(0x07);
        result.push(0x00); // length = 0 means 1 byte follows
        result.push(self.window_size);

        // Max info length - receive (tag 0x06)
        result.push(0x06);
        result.push(0x02); // length = 2
        result.push((self.max_info_length >> 8) as u8);
        result.push((self.max_info_length & 0xFF) as u8);

        result
    }

    /// Parse parameters from info field
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let mut params = Self::default();
        let mut i = 0;

        while i + 1 < data.len() {
            let tag = data[i];
            let len = data[i + 1] as usize;
            i += 2;

            if len == 0 {
                // Single byte value
                if i >= data.len() {
                    break;
                }
                let value = data[i];
                i += 1;

                match tag {
                    0x07 => params.window_size = value,
                    0x08 => params.window_size = value,
                    0x05 => params.max_info_length = value as u16,
                    0x06 => params.max_info_length = value as u16,
                    _ => {}
                }
            } else {
                // Multi-byte value
                if i + len > data.len() {
                    break;
                }

                match tag {
                    0x05 | 0x06 if len >= 2 => {
                        params.max_info_length =
                            ((data[i] as u16) << 8) | (data[i + 1] as u16);
                    }
                    _ => {}
                }
                i += len;
            }
        }

        Ok(params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_type_i() {
        let ft = FrameType::from_control(0x00);
        assert!(matches!(ft, FrameType::I { send_seq: 0, recv_seq: 0 }));
    }

    #[test]
    fn test_frame_type_s() {
        let ft = FrameType::from_control(0x01);
        assert!(matches!(ft, FrameType::S { s_type: 0, recv_seq: 0 }));
    }

    #[test]
    fn test_frame_type_u() {
        let ft = FrameType::from_control(0x93);
        if let FrameType::U { u_type, poll_final } = ft {
            assert_eq!(u_type, u_frame::SNRM); // SNRM modifier bits
            assert!(poll_final);
        } else {
            panic!("Expected U-frame");
        }
    }

    #[test]
    fn test_control_i_frame() {
        let cf = ControlField::i_frame(3, 5, true);
        assert_eq!(cf.raw, 0xB6); // N(R)=5, P=1, N(S)=3
        assert_eq!(cf.poll_final(), true);
    }

    #[test]
    fn test_control_s_frame() {
        let cf = ControlField::s_frame(s_frame::RR, 2, true);
        assert!(cf.raw & 0x01 != 0); // S-frame marker
        assert_eq!(cf.poll_final(), true);
    }

    #[test]
    fn test_control_u_frame_snrm() {
        let cf = ControlField::u_frame(u_frame::SNRM, true); // SNRM modifier
        assert_eq!(cf.raw, 0x93);
        assert!(u_frame::is_snrm(cf.raw));
    }

    #[test]
    fn test_address_one_byte() {
        let addr = HdlcAddress::one_byte(0x10);
        assert_eq!(addr.encode(), vec![0x21]);
        assert_eq!(addr.encoded_length(), 1);
    }

    #[test]
    fn test_address_two_byte() {
        let addr = HdlcAddress::two_byte(0x01, 0x7F);
        assert_eq!(addr.encode(), vec![0x02, 0xFF]);
        assert_eq!(addr.encoded_length(), 2);
    }

    #[test]
    fn test_address_parse_one_byte() {
        let (addr, len) = HdlcAddress::parse(&[0x21]).unwrap();
        assert_eq!(len, 1);
        assert_eq!(addr, HdlcAddress::one_byte(0x10));
    }

    #[test]
    fn test_address_parse_two_byte() {
        let (addr, len) = HdlcAddress::parse(&[0x02, 0xFF]).unwrap();
        assert_eq!(len, 2);
        assert_eq!(addr, HdlcAddress::two_byte(0x01, 0x7F));
    }

    #[test]
    fn test_address_broadcast() {
        let addr = HdlcAddress::one_byte(0x7F);
        assert!(addr.is_broadcast());
    }

    #[test]
    fn test_frame_format() {
        let ff = FrameFormat::new(false, 0x123);
        let encoded = ff.encode();
        assert_eq!(encoded[0] >> 4, 0xA); // Type 3
        assert_eq!(encoded[0] & 0x08, 0); // Not segmented

        let parsed = FrameFormat::parse(&encoded).unwrap();
        assert_eq!(parsed.length, 0x123);
    }

    #[test]
    fn test_frame_format_segmented() {
        let ff = FrameFormat::new(true, 0xFF);
        assert!(ff.segmented);
        let encoded = ff.encode();
        assert!(encoded[0] & 0x08 != 0); // Segmented bit set
    }

    #[test]
    fn test_hdlc_parameters() {
        let params = HdlcParameters {
            window_size: 7,
            max_info_length: 256,
        };
        let encoded = params.encode();
        assert!(!encoded.is_empty());

        let parsed = HdlcParameters::parse(&encoded).unwrap();
        assert_eq!(parsed.window_size, 7);
        assert_eq!(parsed.max_info_length, 256);
    }

    #[test]
    fn test_u_frame_helpers() {
        assert!(u_frame::is_snrm(0x93));
        assert!(u_frame::is_ua(0x73));
        assert!(u_frame::is_disc(0x53));
    }
}
