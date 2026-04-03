//! HDLC frame types and fields

/// HDLC frame type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    I { send_seq: u8, recv_seq: u8 },
    S { s_type: u8, recv_seq: u8 },
    U { u_type: u8, poll_final: bool },
}

impl FrameType {
    pub fn from_control(control: u8) -> Self {
        if control & 0x01 == 0 {
            // I-frame
            FrameType::I {
                send_seq: (control >> 1) & 0x07,
                recv_seq: (control >> 5) & 0x07,
            }
        } else if control & 0x02 == 0 {
            // S-frame
            FrameType::S {
                s_type: (control >> 2) & 0x03,
                recv_seq: (control >> 5) & 0x07,
            }
        } else {
            // U-frame
            FrameType::U {
                u_type: (control >> 2) & 0x07,
                poll_final: (control >> 4) & 0x01 == 1,
            }
        }
    }

    pub fn to_control(&self) -> u8 {
        match self {
            FrameType::I { send_seq, recv_seq } => {
                0x00 | ((*send_seq & 0x07) << 1) | ((*recv_seq & 0x07) << 5)
            }
            FrameType::S { s_type, recv_seq } => {
                0x01 | ((*s_type & 0x03) << 2) | ((*recv_seq & 0x07) << 5)
            }
            FrameType::U { u_type, poll_final } => {
                0x03 | ((*u_type & 0x07) << 2) | ((*poll_final as u8) << 4)
            }
        }
    }
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

    pub fn poll(&self) -> bool {
        (self.raw >> 4) & 0x01 == 1
    }

    pub fn to_byte(&self) -> u8 {
        self.raw
    }
}

/// HDLC address field
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddressField {
    pub raw: u8,
}

impl AddressField {
    pub fn from_byte(byte: u8) -> Self {
        Self { raw: byte }
    }

    pub fn value(&self) -> u8 {
        self.raw
    }

    pub fn is_broadcast(&self) -> bool {
        self.raw & 0x80 != 0
    }

    pub fn to_byte(&self) -> u8 {
        self.raw
    }
}

/// Parsed HDLC frame
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HdlcFrame {
    pub address: AddressField,
    pub control: ControlField,
    pub info: Vec<u8>,
}
