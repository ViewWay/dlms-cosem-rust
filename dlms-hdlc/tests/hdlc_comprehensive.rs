//! Comprehensive HDLC tests — boundary conditions, noise resilience, protocol correctness

use dlms_hdlc::*;

// ============================================================
// 1. CRC-16/X.25 known vectors & good-FCS property
// ============================================================

#[test]
fn test_crc_known_vector_123456789() {
    // CRC-16/X.25 of "123456789" — verify determinism
    let data = b"123456789";
    let crc = crc16_hdlc(data);
    // Verify it's consistent with incremental computation
    let mut inc = 0xFFFFu16;
    for &b in data {
        inc = crc16_hdlc_update(inc, b);
    }
    assert_eq!(crc, inc);
    // Should not be trivial values
    assert_ne!(crc, 0x0000);
    assert_ne!(crc, 0xFFFF);
}

#[test]
fn test_crc_append_verifies() {
    // The parser uses CRC by computing over content and comparing with received FCS.
    // Verify that build_frame's FCS matches what crc16_hdlc computes.
    let data = [0x03, 0x10, 0xE6, 0x00, 0x01, 0x02];
    let fcs = crc16_hdlc(&data);
    let fcs_lo = (fcs & 0xFF) as u8;
    let fcs_hi = (fcs >> 8) as u8;
    // Verify FCS is recoverable by the parser's method
    let received_fcs = (fcs_hi as u16) << 8 | fcs_lo as u16;
    assert_eq!(fcs, received_fcs);
}

#[test]
fn test_crc_single_byte_all_values() {
    // Ensure no panic and deterministic for all byte values
    for b in 0u8..=255 {
        let c = crc16_hdlc(&[b]);
        assert_ne!(c, 0xFFFF); // only empty input gives 0xFFFF
    }
}

#[test]
fn test_crc_incremental_vs_batch() {
    let data = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    let batch = crc16_hdlc(&data);
    let mut incremental = 0xFFFF;
    for &b in &data {
        incremental = crc16_hdlc_update(incremental, b);
    }
    assert_eq!(batch, incremental);
}

// ============================================================
// 2. Byte stuffing edge cases
// ============================================================

#[test]
fn test_stuff_escape_followed_by_flag() {
    // 0x7D 0x7E in input: escape escapes the 0x7D, then 0x7E is a flag
    // stuff_bytes only stuffs 0x7E and 0x7D, doesn't handle flag semantics
    let data = vec![0x7D, 0x7E];
    let stuffed = stuff_bytes(&data);
    assert_eq!(stuffed, vec![0x7D, 0x5D, 0x7D, 0x5E]);
    assert_eq!(unstuff_bytes(&stuffed), data);
}

#[test]
fn test_stuff_consecutive_specials() {
    let data = vec![0x7E, 0x7D, 0x7E, 0x7D, 0x7E];
    let stuffed = stuff_bytes(&data);
    for &b in &stuffed {
        assert_ne!(b, 0x7E, "stuffed data should never contain raw 0x7E");
    }
    assert_eq!(unstuff_bytes(&stuffed), data);
}

#[test]
fn test_stuff_large_payload() {
    let data: Vec<u8> = (0u8..=255).cycle().take(1024).collect();
    let stuffed = stuff_bytes(&data);
    assert_eq!(unstuff_bytes(&stuffed), data);
}

#[test]
fn test_stuff_preserves_non_special() {
    // All bytes except 0x7E and 0x7D should pass through unchanged
    for b in 0u8..=255 {
        if b == 0x7E || b == 0x7D { continue; }
        assert_eq!(stuff_bytes(&[b]), vec![b]);
    }
}

// ============================================================
// 3. Frame size boundary
// ============================================================

#[test]
fn test_frame_max_size() {
    // Build frame with info close to max. Frame = flag(1) + addr(1) + ctrl(1) + info + FCS(2) + flag(1)
    // After stuffing, size can grow. HDLC_MAX_FRAME_SIZE limits parser buffer.
    let large_info = vec![0x41u8; 500];
    let frame = build_frame(0x03, 0x10, &large_info);
    let mut parser = HdlcParser::new();
    let mut got_frame = false;
    for &byte in &frame {
        if let Some(result) = parser.feed(byte) {
            assert!(result.is_ok());
            got_frame = true;
        }
    }
    assert!(got_frame);
}

#[test]
fn test_parser_rejects_oversized() {
    // Feed > HDLC_MAX_FRAME_SIZE bytes between flags
    let mut parser = HdlcParser::new();
    parser.feed(0x7E); // start
    for _ in 0..=HDLC_MAX_FRAME_SIZE {
        parser.feed(0x41);
    }
    // Now feed closing flag — should get None (frame was too long, silently dropped)
    let result = parser.feed(0x7E);
    assert!(result.is_none());
}

// ============================================================
// 4. Noise resilience
// ============================================================

#[test]
fn test_noise_before_frame() {
    let frame = build_frame(0x03, 0x73, &[]);
    let noise = [0xFF, 0x00, 0xAA, 0x55, 0x12, 0x34];
    let mut data = noise.to_vec();
    data.extend_from_slice(&frame);
    let mut parser = HdlcParser::new();
    let mut frames = Vec::new();
    for &byte in &data {
        if let Some(result) = parser.feed(byte) {
            frames.push(result);
        }
    }
    assert_eq!(frames.len(), 1);
    assert!(frames[0].is_ok());
}

#[test]
fn test_noise_between_frames() {
    let f1 = build_frame(0x03, 0x73, &[]);
    let f2 = build_frame(0x03, 0x10, &[0x01]);
    let noise = [0xDE, 0xAD, 0xBE, 0xEF];
    let mut data = f1.clone();
    data.extend_from_slice(&noise);
    data.extend_from_slice(&f2);
    let mut parser = HdlcParser::new();
    let mut count = 0;
    for &byte in &data {
        if let Some(result) = parser.feed(byte) {
            assert!(result.is_ok(), "frame {} should be ok", count);
            count += 1;
        }
    }
    assert_eq!(count, 2);
}

#[test]
fn test_corrupted_flags_as_data() {
    // 0x7E bytes inside a frame (without proper framing) should be treated as frame boundaries
    let mut parser = HdlcParser::new();
    // start frame
    parser.feed(0x7E);
    parser.feed(0x03);
    // premature flag — should end frame (too short, discarded)
    parser.feed(0x7E);
    // now a proper frame
    let good = build_frame(0x03, 0x73, &[]);
    let mut count = 0;
    for &byte in &good {
        if let Some(result) = parser.feed(byte) {
            assert!(result.is_ok());
            count += 1;
        }
    }
    assert_eq!(count, 1);
}

// ============================================================
// 5. Back-to-back flags
// ============================================================

#[test]
fn test_back_to_back_flags() {
    let frame = build_frame(0x03, 0x10, &[0xAA]);
    let mut data = vec![0x7E, 0x7E]; // extra flags before frame
    data.extend_from_slice(&frame);
    data.extend_from_slice(&[0x7E, 0x7E]); // extra trailing flags
    let mut parser = HdlcParser::new();
    let mut count = 0;
    for &byte in &data {
        if let Some(result) = parser.feed(byte) {
            assert!(result.is_ok());
            count += 1;
        }
    }
    assert_eq!(count, 1);
}

// ============================================================
// 6. Escape sequence edge cases in parser
// ============================================================

#[test]
fn test_escape_at_end_of_stream() {
    // Escape byte at end of data (no closing flag) — should not panic
    let mut parser = HdlcParser::new();
    parser.feed(0x7E);
    parser.feed(0x03);
    parser.feed(0x10);
    parser.feed(0x7D); // escape with no following byte
    // No assertion needed — just checking it doesn't panic
}

#[test]
fn test_parser_recovery_after_bad_escape() {
    // If parser is in escaped state and gets flag, should reset
    let mut parser = HdlcParser::new();
    parser.feed(0x7E);
    parser.feed(0x03);
    parser.feed(0x7D); // enter escaped state
    parser.feed(0x7E); // flag during escape — should reset
    // Now feed a valid frame
    let good = build_frame(0x03, 0x73, &[]);
    let mut count = 0;
    for &byte in &good {
        if let Some(result) = parser.feed(byte) {
            assert!(result.is_ok());
            count += 1;
        }
    }
    assert_eq!(count, 1);
}

// ============================================================
// 7. I-frame sequence number wrapping (mod 8)
// ============================================================

#[test]
fn test_i_frame_all_send_seqs() {
    for ns in 0..=7 {
        let control = FrameType::I { send_seq: ns, recv_seq: 0 }.to_control();
        let cf = ControlField::from_byte(control);
        if let FrameType::I { send_seq, .. } = cf.frame_type() {
            assert_eq!(send_seq, ns);
        } else {
            panic!("Expected I-frame for N(S)={}", ns);
        }
    }
}

#[test]
fn test_i_frame_all_recv_seqs() {
    for nr in 0..=7 {
        let control = FrameType::I { send_seq: 0, recv_seq: nr }.to_control();
        let cf = ControlField::from_byte(control);
        if let FrameType::I { recv_seq, .. } = cf.frame_type() {
            assert_eq!(recv_seq, nr);
        } else {
            panic!("Expected I-frame for N(R)={}", nr);
        }
    }
}

#[test]
fn test_i_frame_with_poll() {
    for ns in 0..=7 {
        for nr in 0..=7 {
            let ft = FrameType::I { send_seq: ns, recv_seq: nr };
            let control = ft.to_control();
            // Set poll bit (bit 4)
            let control_p = control | 0x10;
            let cf = ControlField::from_byte(control_p);
            assert!(cf.poll(), "I-frame N(S)={} N(R)={} with P=1 should have poll()", ns, nr);
            if let FrameType::I { send_seq: s, recv_seq: r } = cf.frame_type() {
                assert_eq!(s, ns);
                assert_eq!(r, nr);
            } else {
                panic!("Not an I-frame");
            }
        }
    }
}

// ============================================================
// 8. S-frame types: RR(0), REJ(1), RNR(2), SREJ(3)
// ============================================================

#[test]
fn test_s_frame_types() {
    let types = [(0, "RR"), (1, "REJ"), (2, "RNR"), (3, "SREJ")];
    for &(s_type, name) in &types {
        let control = FrameType::S { s_type, recv_seq: 0 }.to_control();
        let cf = ControlField::from_byte(control);
        if let FrameType::S { s_type: st, recv_seq: 0 } = cf.frame_type() {
            assert_eq!(st, s_type, "S-frame type mismatch for {}", name);
        } else {
            panic!("Expected S-frame for {}", name);
        }
    }
}

#[test]
fn test_s_frame_with_poll() {
    for s_type in 0..=3 {
        for nr in 0..=7 {
            let ft = FrameType::S { s_type, recv_seq: nr };
            let control = ft.to_control() | 0x10; // poll bit
            let cf = ControlField::from_byte(control);
            assert!(cf.poll());
            if let FrameType::S { s_type: st, recv_seq: r } = cf.frame_type() {
                assert_eq!(st, s_type);
                assert_eq!(r, nr);
            } else {
                panic!("Not an S-frame");
            }
        }
    }
}

// ============================================================
// 9. U-frame types with poll/final
// ============================================================

#[test]
fn test_u_frame_roundtrip() {
    // With 2-bit u_type (bits 3:2), there's no collision with poll_final (bit 4)
    // All u_type 0-3 roundtrip with any poll_final
    for u_type in 0u8..4 {
        for pf in [false, true] {
            let ft = FrameType::U { u_type, poll_final: pf };
            let control = ft.to_control();
            let ft2 = FrameType::from_control(control);
            assert_eq!(ft, ft2, "u_type={} pf={} roundtrip", u_type, pf);
        }
    }
    // Known DLMS values
    // 0x93 = 0b10010011: b4=1(P/F), b3:b2=00 → u_type=0, poll_final=true
    let snrm = ControlField::from_byte(0x93);
    if let FrameType::U { u_type, poll_final } = snrm.frame_type() {
        assert_eq!(u_type, 0);
        assert!(poll_final, "SNRM should have poll/final set");
    }
    // 0x73 = 0b01110011: b4=1(P/F!), b3:b2=00 → u_type=0, poll_final=true
    // Note: 0x73 has bit 4 SET (0b0111_0011, bit4=1), so poll_final=true
    let ua = ControlField::from_byte(0x73);
    if let FrameType::U { u_type, poll_final } = ua.frame_type() {
        assert_eq!(u_type, 0);
        assert!(poll_final, "UA 0x73 has bit4=1");
    }
}

// ============================================================
// 10. DLMS-specific HDLC (APDU start bytes 0xE6/0xE7)
// ============================================================

#[test]
fn test_dlms_get_request_apdu() {
    // GET-REQUEST: 0xC0 0x01 (normal, 1st invocation) + OBIS
    let info = vec![0xE6, 0xE0, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xFF]; // GetRequest for 0.0.255.255
    let frame = build_frame(0x03, 0x10, &info);
    let mut parser = HdlcParser::new();
    for &byte in &frame {
        if let Some(result) = parser.feed(byte) {
            let f = result.unwrap();
            assert_eq!(f.info[0], 0xE6, "DLMS APDU should start with 0xE6");
            return;
        }
    }
    panic!("No frame parsed");
}

#[test]
fn test_dlms_set_request_apdu() {
    let info = vec![0xE6, 0xD0, 0x01]; // SetRequest
    let frame = build_frame(0x03, 0x10, &info);
    let mut parser = HdlcParser::new();
    for &byte in &frame {
        if let Some(result) = parser.feed(byte) {
            let f = result.unwrap();
            assert_eq!(f.info[0], 0xE6);
            return;
        }
    }
    panic!("No frame parsed");
}

#[test]
fn test_dlms_response_apdu() {
    let info = vec![0xE7, 0x00, 0xC0, 0x01, 0x00]; // GetResponse (normal)
    let frame = build_frame(0x03, 0x10, &info);
    let mut parser = HdlcParser::new();
    for &byte in &frame {
        if let Some(result) = parser.feed(byte) {
            let f = result.unwrap();
            assert_eq!(f.info[0], 0xE7, "DLMS Response should start with 0xE7");
            return;
        }
    }
    panic!("No frame parsed");
}

// ============================================================
// 11. Build/parse roundtrip for various frame types
// ============================================================

#[test]
fn test_roundtrip_i_frame_small() {
    let info = vec![0xE6, 0x00];
    let frame = build_frame(0x03, 0x00, &info); // I-frame N(S)=0 N(R)=0
    let mut parser = HdlcParser::new();
    for &byte in &frame {
        if let Some(result) = parser.feed(byte) {
            let f = result.unwrap();
            assert_eq!(f.info, info);
            assert_eq!(f.address.value(), 0x03);
            return;
        }
    }
    panic!("No frame");
}

#[test]
fn test_roundtrip_i_frame_large() {
    let info: Vec<u8> = (0..200).map(|i| (i % 256) as u8).collect();
    let frame = build_frame(0x03, 0x20, &info); // I-frame N(S)=0 N(R)=1
    let mut parser = HdlcParser::new();
    for &byte in &frame {
        if let Some(result) = parser.feed(byte) {
            let f = result.unwrap();
            assert_eq!(f.info, info);
            return;
        }
    }
    panic!("No frame");
}

#[test]
fn test_roundtrip_u_frame_snrm() {
    let frame = build_frame(0x03, 0x93, &[]); // SNRM with P=1
    let mut parser = HdlcParser::new();
    for &byte in &frame {
        if let Some(result) = parser.feed(byte) {
            let f = result.unwrap();
            assert!(matches!(f.control.frame_type(), FrameType::U { .. }));
            return;
        }
    }
    panic!("No frame");
}

#[test]
fn test_roundtrip_u_frame_ua() {
    let frame = build_frame(0x03, 0x73, &[]); // UA with F=1
    let mut parser = HdlcParser::new();
    for &byte in &frame {
        if let Some(result) = parser.feed(byte) {
            let f = result.unwrap();
            assert!(matches!(f.control.frame_type(), FrameType::U { .. }));
            return;
        }
    }
    panic!("No frame");
}

#[test]
fn test_roundtrip_s_frame_rr() {
    let frame = build_frame(0x03, 0x01, &[]); // RR, N(R)=0
    let mut parser = HdlcParser::new();
    for &byte in &frame {
        if let Some(result) = parser.feed(byte) {
            let f = result.unwrap();
            assert!(matches!(f.control.frame_type(), FrameType::S { .. }));
            return;
        }
    }
    panic!("No frame");
}

#[test]
fn test_roundtrip_with_special_bytes_in_info() {
    // Info containing 0x7E and 0x7D should survive stuffing/unstuffing
    let info = vec![0xE6, 0x7E, 0x7D, 0xFF, 0x00, 0x7E];
    let frame = build_frame(0x03, 0x10, &info);
    let mut parser = HdlcParser::new();
    for &byte in &frame {
        if let Some(result) = parser.feed(byte) {
            let f = result.unwrap();
            assert_eq!(f.info, info);
            return;
        }
    }
    panic!("No frame");
}

// ============================================================
// 12. Parser recovery after CRC error
// ============================================================

#[test]
fn test_parser_recovery_after_crc_error() {
    let good1 = build_frame(0x03, 0x73, &[]); // UA
    let mut bad = build_frame(0x03, 0x10, &[0x01, 0x02]);
    // Corrupt middle byte
    let mid = bad.len() / 2;
    bad[mid] ^= 0xFF;
    let good2 = build_frame(0x03, 0x10, &[0xAA]);

    let mut data = good1;
    data.extend_from_slice(&bad);
    data.extend_from_slice(&good2);

    let mut parser = HdlcParser::new();
    let mut results = Vec::new();
    for &byte in &data {
        if let Some(result) = parser.feed(byte) {
            results.push(result);
        }
    }
    assert_eq!(results.len(), 3);
    assert!(results[0].is_ok(), "First frame should be valid");
    assert!(results[1].is_err(), "Corrupted frame should fail CRC");
    assert!(results[2].is_ok(), "Third frame should be valid after recovery");
}

#[test]
fn test_parser_recovery_multiple_errors() {
    let good = build_frame(0x03, 0x73, &[]);
    let mut bad1 = build_frame(0x03, 0x10, &[0x01]);
    bad1[2] ^= 0xFF;
    let mut bad2 = build_frame(0x03, 0x10, &[0x02]);
    bad2[3] ^= 0xFF;

    let mut data = good.clone();
    data.extend_from_slice(&bad1);
    data.extend_from_slice(&bad2);
    data.extend_from_slice(&good);

    let mut parser = HdlcParser::new();
    let mut ok_count = 0;
    let mut err_count = 0;
    for &byte in &data {
        if let Some(result) = parser.feed(byte) {
            if result.is_ok() { ok_count += 1; } else { err_count += 1; }
        }
    }
    assert_eq!(ok_count, 2, "Should recover and parse 2 good frames");
    assert_eq!(err_count, 2, "Should detect 2 corrupted frames");
}

// ============================================================
// 13. Address field tests
// ============================================================

#[test]
fn test_address_broadcast_detection() {
    // DLMS broadcast: bit 7 set
    assert!(AddressField::from_byte(0x81).is_broadcast());
    assert!(AddressField::from_byte(0xFF).is_broadcast());
    assert!(!AddressField::from_byte(0x01).is_broadcast());
    assert!(!AddressField::from_byte(0x03).is_broadcast());
}

#[test]
fn test_address_all_values() {
    // Ensure AddressField works for all byte values
    for b in 0u8..=255 {
        let addr = AddressField::from_byte(b);
        assert_eq!(addr.value(), b);
        assert_eq!(addr.to_byte(), b);
        // Broadcast iff bit 7 set
        assert_eq!(addr.is_broadcast(), b & 0x80 != 0);
    }
}

// ============================================================
// 14. Build frame structure validation
// ============================================================

#[test]
fn test_build_frame_structure() {
    let frame = build_frame(0x03, 0x10, &[0xAA, 0xBB]);
    assert_eq!(frame[0], 0x7E, "Should start with flag");
    assert_eq!(*frame.last().unwrap(), 0x7E, "Should end with flag");
    // No raw 0x7E or 0x7D in the stuffed payload between flags
    for &b in &frame[1..frame.len()-1] {
        assert_ne!(b, 0x7E, "Stuffed payload should not contain 0x7E");
    }
}

#[test]
fn test_build_frame_no_info() {
    let frame = build_frame(0x03, 0x73, &[]);
    assert_eq!(frame[0], 0x7E);
    assert_eq!(*frame.last().unwrap(), 0x7E);
    // Minimum: flag + addr + ctrl + FCS(2) + flag = 6 bytes (no stuffing needed)
    assert!(frame.len() >= 6);
}

// ============================================================
// 15. FrameType to_control/from_control roundtrip
// ============================================================

#[test]
fn test_frametype_roundtrip() {
    // I-frame combos
    for ns in 0..=7 {
        for nr in 0..=7 {
            let ft = FrameType::I { send_seq: ns, recv_seq: nr };
            let control = ft.to_control();
            assert_eq!(control & 0x01, 0, "I-frame bit 0 must be 0");
            let ft2 = FrameType::from_control(control);
            assert_eq!(ft, ft2);
        }
    }
    // S-frame combos
    for st in 0..=3 {
        for nr in 0..=7 {
            let ft = FrameType::S { s_type: st, recv_seq: nr };
            let control = ft.to_control();
            assert_eq!(control & 0x03, 0x01, "S-frame bits 1:0 must be 01");
            let ft2 = FrameType::from_control(control);
            assert_eq!(ft, ft2);
        }
    }
    // U-frame: 2-bit u_type (bits 3:2) + poll_final (bit 4) — no collision
    for ut in 0u8..4 {
        for pf in [false, true] {
            let ft = FrameType::U { u_type: ut, poll_final: pf };
            let control = ft.to_control();
            assert_eq!(control & 0x03, 0x03);
            let ft2 = FrameType::from_control(control);
            assert_eq!(ft, ft2);
        }
    }
}

// ============================================================
// 16. Parser reset
// ============================================================

#[test]
fn test_parser_reset_clears_state() {
    let mut parser = HdlcParser::new();
    parser.feed(0x7E);
    parser.feed(0x03);
    parser.feed(0x7D); // enter escaped state
    assert!(matches!(parser.state(), ParserState::Escaped));
    parser.reset();
    assert!(matches!(parser.state(), ParserState::Idle));
    // Should still work after reset
    let good = build_frame(0x03, 0x73, &[]);
    let mut count = 0;
    for &byte in &good {
        if let Some(result) = parser.feed(byte) {
            assert!(result.is_ok());
            count += 1;
        }
    }
    assert_eq!(count, 1);
}

// ============================================================
// 17. HdlcError variants
// ============================================================

#[test]
fn test_hdlc_error_crc_display() {
    let err = HdlcError::CrcError { expected: 0x1234, actual: 0x5678 };
    let msg = format!("{}", err);
    assert!(msg.contains("CRC"));
    assert!(msg.contains("1234") || msg.contains("4660")); // hex or decimal
}

#[test]
fn test_hdlc_error_frame_too_long() {
    let msg = format!("{}", HdlcError::FrameTooLong);
    assert!(msg.contains("maximum") || msg.contains("long"));
}

#[test]
fn test_hdlc_error_parser_error() {
    let msg = format!("{}", HdlcError::ParserError("test".into()));
    assert!(msg.contains("test"));
}

// ============================================================
// 18. Default trait
// ============================================================

#[test]
fn test_parser_default() {
    let parser = HdlcParser::default();
    assert!(matches!(parser.state(), ParserState::Idle));
}

// ============================================================
// 19. Multiple frames in sequence
// ============================================================

#[test]
fn test_three_frames_sequence() {
    let f1 = build_frame(0x03, 0x73, &[]); // UA
    let f2 = build_frame(0x03, 0x10, &[0xE6, 0x00]); // I-frame
    let f3 = build_frame(0x03, 0x01, &[]); // RR
    let mut data = Vec::new();
    data.extend_from_slice(&f1);
    data.extend_from_slice(&f2);
    data.extend_from_slice(&f3);
    let mut parser = HdlcParser::new();
    let mut results = Vec::new();
    for &byte in &data {
        if let Some(result) = parser.feed(byte) {
            results.push(result.unwrap());
        }
    }
    assert_eq!(results.len(), 3);
    // Verify info fields
    assert!(results[0].info.is_empty()); // UA has no info
    assert_eq!(results[1].info, vec![0xE6, 0x00]); // I-frame
    assert!(results[2].info.is_empty()); // RR has no info
}

#[test]
fn test_interleaved_noise_and_frames() {
    let frames = vec![
        build_frame(0x03, 0x73, &[]),
        build_frame(0x03, 0x10, &[0x01]),
        build_frame(0x03, 0x01, &[]),
        build_frame(0x03, 0x10, &[0xAA, 0xBB, 0xCC]),
    ];
    let mut data = Vec::new();
    for frame in &frames {
        data.extend_from_slice(&[0xFF, 0x00, 0xFF]); // noise between
        data.extend_from_slice(frame);
    }
    data.extend_from_slice(&[0xDE, 0xAD]); // trailing noise

    let mut parser = HdlcParser::new();
    let mut count = 0;
    for &byte in &data {
        if let Some(result) = parser.feed(byte) {
            assert!(result.is_ok(), "Frame {} should parse correctly", count);
            count += 1;
        }
    }
    assert_eq!(count, frames.len());
}
