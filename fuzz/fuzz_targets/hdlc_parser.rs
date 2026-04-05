//! Fuzz target for HDLC parser — should never panic on random input

#![no_main]
use libfuzzer_sys::fuzz_target;
use dlms_hdlc::*;

fuzz_target!(|bytes: &[u8]| {
    // Parser should never panic on any input
    let mut parser = HdlcParser::new();
    for &byte in bytes {
        // Feed byte by byte - should not panic
        let _ = parser.feed(*byte);
    }
    // We don't assert anything — the goal is crash-free execution
    // The fuzzer will discover panic/crash/timeout issues
});
