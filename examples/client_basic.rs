//! Example: DLMS client communication flow
//!
//! Demonstrates a basic DLMS client connection workflow.

use dlms_core::ObisCode;
use dlms_client::DlmsClient;
use dlms_security::{SecuritySuite, HlsContext};

fn main() {
    // Create a DLMS client for HDLC serial communication
    let mut client = DlmsClient::new("192.168.1.100:4059").unwrap();

    // Set up security
    let mut hls = HlsContext::new(SecuritySuite::Suite0);
    println!("Security suite: {:?}", hls.suite());
    println!("Needs authentication: {}", hls.suite().needs_authentication());
    println!("Needs encryption: {}", hls.suite().needs_encryption());

    // In a real scenario, you would:
    // 1. Connect via HDLC/TCP
    // 2. Perform HLS authentication
    // 3. Read COSEM objects (GET request)
    // 4. Write attributes (SET request)
    // 5. Execute methods (ACTION request)

    println!("DLMS client example - connection flow demonstrated");
}
