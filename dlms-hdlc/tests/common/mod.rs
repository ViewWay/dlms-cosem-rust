//! Common test utilities for HDLC testing

/// Create a test HDLC frame from raw bytes (just wraps with flags, no CRC)
pub fn raw_frame(bytes: &[u8]) -> Vec<u8> {
    let mut frame = vec![0x7E];
    frame.extend_from_slice(bytes);
    frame.push(0x7E);
    frame
}

/// Create a valid HDLC frame using dlms-hdlc's build_frame
pub fn build_test_frame(address: u8, control: u8, info: &[u8]) -> Vec<u8> {
    dlms_hdlc::build_frame(address, control, info)
}

/// Parse all frames from raw bytes
pub fn parse_frames(data: &[u8]) -> Vec<Result<dlms_hdlc::HdlcFrame, dlms_hdlc::HdlcError>> {
    let mut parser = dlms_hdlc::HdlcParser::new();
    let mut results = Vec::new();
    for &byte in data {
        if let Some(result) = parser.feed(byte) {
            results.push(result);
        }
    }
    results
}
