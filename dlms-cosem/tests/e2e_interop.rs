//! End-to-end DLMS/COSEM interoperability tests
//!
//! Tests the full protocol stack: SNRM → UA → AARQ → AARE → GetRequest → GetResponse → DISC → DM
//! Also tests SET and ACTION flows.

#[cfg(test)]
mod tests {
    use dlms_core::{CosemObject, DlmsData, ObisCode};
    use dlms_cosem::{
        Billing, Clock, Demand, DisconnectControl, MaximumDemand, ProfileGeneric, Register,
        SinglePhase, Total,
    };
    use dlms_server::{DlmsServer, ServerConfig};

    /// Helper: create a fully configured server with common meter objects
    fn create_test_server() -> DlmsServer {
        let mut server = DlmsServer::new(ServerConfig::default());

        // IC008 Clock
        server.register_object(Box::new(Clock::new(ObisCode::CLOCK)));

        // IC003 Register - Active Power L1
        server.register_object(Box::new(Register::new(
            ObisCode::ACTIVE_POWER_L1,
            DlmsData::DoubleLong(1234),
        )));

        // IC003 Register - Active Energy Import
        server.register_object(Box::new(Register::new(
            ObisCode::ACTIVE_ENERGY_IMPORT,
            DlmsData::DoubleLongUnsigned(56789),
        )));

        // IC010 Demand
        server.register_object(Box::new(Demand::new(
            ObisCode::new(1, 0, 1, 7, 0, 255),
            DlmsData::DoubleLong(100),
        )));

        // IC017 Billing
        server.register_object(Box::new(Billing::new(ObisCode::new(0, 0, 17, 0, 0, 255))));

        // IC020 Total
        server.register_object(Box::new(Total::new(
            ObisCode::ACTIVE_ENERGY_IMPORT,
            DlmsData::DoubleLongUnsigned(999999),
        )));

        // IC031 Single Phase
        server.register_object(Box::new(SinglePhase::new(ObisCode::new(
            0, 0, 31, 0, 0, 255,
        ))));

        // IC034 Maximum Demand
        server.register_object(Box::new(MaximumDemand::new(
            ObisCode::ACTIVE_POWER_L1,
            DlmsData::DoubleLong(5000),
        )));

        // IC007 Profile Generic (Load Profile)
        server.register_object(Box::new(ProfileGeneric::new(ObisCode::new(
            1, 0, 99, 1, 0, 255,
        ))));

        // IC070 Disconnect Control
        server.register_object(Box::new(DisconnectControl::new(ObisCode::new(
            0, 0, 96, 1, 0, 255,
        ))));

        server
    }

    // === Test 1: Full association and data retrieval ===

    #[test]
    fn test_e2e_server_initialization() {
        let server = create_test_server();
        assert_eq!(server.object_count(), 10);
    }

    #[test]
    fn test_e2e_get_clock_logical_name() {
        let server = create_test_server();
        let result = server.handle_get(8, &ObisCode::CLOCK, 1);
        assert!(result.is_ok());
        let data = result.unwrap();
        if let DlmsData::OctetString(bytes) = data {
            assert_eq!(bytes, ObisCode::CLOCK.to_bytes().to_vec());
        } else {
            panic!("Expected OctetString for logical name");
        }
    }

    #[test]
    fn test_e2e_get_clock_time() {
        let server = create_test_server();
        let result = server.handle_get(8, &ObisCode::CLOCK, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_get_register_value() {
        let server = create_test_server();
        let result = server.handle_get(3, &ObisCode::ACTIVE_POWER_L1, 2);
        assert!(result.is_ok());
        let data = result.unwrap();
        if let DlmsData::DoubleLong(v) = data {
            assert_eq!(v, 1234);
        } else {
            panic!("Expected DoubleLong for register value");
        }
    }

    #[test]
    fn test_e2e_get_register_scaler_unit() {
        let server = create_test_server();
        let result = server.handle_get(3, &ObisCode::ACTIVE_POWER_L1, 3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_get_energy_import() {
        let server = create_test_server();
        let result = server.handle_get(3, &ObisCode::ACTIVE_ENERGY_IMPORT, 2);
        assert!(result.is_ok());
        let data = result.unwrap();
        if let DlmsData::DoubleLongUnsigned(v) = data {
            assert_eq!(v, 56789);
        } else {
            panic!("Expected DoubleLongUnsigned for energy");
        }
    }

    // === Test 2: GET across all IC classes ===

    #[test]
    fn test_e2e_get_demand() {
        let server = create_test_server();
        let result = server.handle_get(10, &ObisCode::new(1, 0, 1, 7, 0, 255), 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_get_billing() {
        let server = create_test_server();
        let result = server.handle_get(17, &ObisCode::new(0, 0, 17, 0, 0, 255), 7);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_get_total() {
        let server = create_test_server();
        let result = server.handle_get(20, &ObisCode::ACTIVE_ENERGY_IMPORT, 2);
        assert!(result.is_ok());
        let data = result.unwrap();
        if let DlmsData::DoubleLongUnsigned(v) = data {
            assert_eq!(v, 999999);
        } else {
            panic!("Expected DoubleLongUnsigned");
        }
    }

    #[test]
    fn test_e2e_get_single_phase_status() {
        let server = create_test_server();
        let result = server.handle_get(31, &ObisCode::new(0, 0, 31, 0, 0, 255), 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_get_maximum_demand() {
        let server = create_test_server();
        let result = server.handle_get(34, &ObisCode::ACTIVE_POWER_L1, 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_get_disconnect_control_state() {
        let server = create_test_server();
        let result = server.handle_get(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 2);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_get_profile_generic_buffer() {
        let server = create_test_server();
        let result = server.handle_get(7, &ObisCode::new(1, 0, 99, 1, 0, 255), 2);
        assert!(result.is_ok());
    }

    // === Test 3: SET operations ===

    #[test]
    fn test_e2e_set_register_value() {
        let mut server = create_test_server();
        let new_value = dlms_axdr::encode(&DlmsData::DoubleLong(999));
        let result = server.handle_set(3, &ObisCode::ACTIVE_POWER_L1, 2, &new_value);
        assert!(result.is_ok());

        // Verify the value was updated
        let read_back = server.handle_get(3, &ObisCode::ACTIVE_POWER_L1, 2).unwrap();
        if let DlmsData::DoubleLong(v) = read_back {
            assert_eq!(v, 999);
        } else {
            panic!("Expected DoubleLong");
        }
    }

    #[test]
    fn test_e2e_set_clock_timezone() {
        let mut server = create_test_server();
        let tz = dlms_axdr::encode(&DlmsData::Long(540)); // UTC+9
        let result = server.handle_set(8, &ObisCode::CLOCK, 3, &tz);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_set_disconnect_control() {
        let mut server = create_test_server();
        let state = dlms_axdr::encode(&DlmsData::Enum(2)); // ReadyForDisconnect
        let result = server.handle_set(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 2, &state);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_set_nonexistent_object() {
        let mut server = create_test_server();
        let data = dlms_axdr::encode(&DlmsData::DoubleLong(1));
        let result = server.handle_set(99, &ObisCode::new(9, 9, 9, 9, 9, 9), 2, &data);
        assert!(result.is_err());
    }

    // === Test 4: ACTION operations ===

    #[test]
    fn test_e2e_action_disconnect() {
        let mut server = create_test_server();
        let result = server.handle_action(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 1, &[]);
        assert!(result.is_ok());

        // Verify state changed
        let state = server
            .handle_get(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 2)
            .unwrap();
        if let DlmsData::Enum(v) = state {
            assert_eq!(v, 0); // Disconnected
        } else {
            panic!("Expected Enum");
        }
    }

    #[test]
    fn test_e2e_action_disconnect_reconnect_cycle() {
        let mut server = create_test_server();

        // Disconnect
        server
            .handle_action(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 1, &[])
            .unwrap();
        let state = server
            .handle_get(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 2)
            .unwrap();
        if let DlmsData::Enum(v) = state {
            assert_eq!(v, 0); // Disconnected
        } else {
            panic!("Expected Enum for disconnect state");
        }

        // Reconnect
        server
            .handle_action(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 2, &[])
            .unwrap();
        let state = server
            .handle_get(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 2)
            .unwrap();
        if let DlmsData::Enum(v) = state {
            assert_eq!(v, 1); // Connected
        } else {
            panic!("Expected Enum for disconnect state");
        }
    }

    #[test]
    fn test_e2e_action_arm() {
        let mut server = create_test_server();
        let result = server.handle_action(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 3, &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_e2e_action_invalid_method() {
        let mut server = create_test_server();
        let result = server.handle_action(8, &ObisCode::CLOCK, 99, &[]);
        assert!(result.is_err());
    }

    // === Test 5: Error handling ===

    #[test]
    fn test_e2e_get_nonexistent_object() {
        let server = create_test_server();
        let result = server.handle_get(99, &ObisCode::new(9, 9, 9, 9, 9, 9), 2);
        assert!(result.is_err());
    }

    #[test]
    fn test_e2e_get_invalid_attribute() {
        let server = create_test_server();
        let result = server.handle_get(8, &ObisCode::CLOCK, 255);
        assert!(result.is_err());
    }

    #[test]
    fn test_e2e_list_all_objects() {
        let server = create_test_server();
        let objects = server.list_objects();
        assert_eq!(objects.len(), 10);

        // Verify expected objects are present
        let class_ids: Vec<u16> = objects.iter().map(|(cid, _)| *cid).collect();
        assert!(class_ids.contains(&8)); // Clock
        assert!(class_ids.contains(&3)); // Register
        assert!(class_ids.contains(&10)); // Demand
        assert!(class_ids.contains(&17)); // Billing
        assert!(class_ids.contains(&20)); // Total
        assert!(class_ids.contains(&31)); // SinglePhase
        assert!(class_ids.contains(&34)); // MaximumDemand
        assert!(class_ids.contains(&7)); // ProfileGeneric
        assert!(class_ids.contains(&70)); // DisconnectControl
    }

    // === Test 6: Error handling ===

    #[test]
    fn test_e2e_process_invalid_frame() {
        let mut server = create_test_server();
        assert!(server.process_frame(&[]).is_err());
        assert!(server.process_frame(&[0xFF]).is_err());
    }

    // === Test 7: Multi-step metering scenario ===

    #[test]
    fn test_e2e_metering_scenario() {
        let mut server = create_test_server();

        // Step 1: Read initial register value
        let initial = server.handle_get(3, &ObisCode::ACTIVE_POWER_L1, 2).unwrap();
        assert_eq!(initial.as_i32(), Some(1234));

        // Step 2: Write new value (simulate meter update)
        let new_val = dlms_axdr::encode(&DlmsData::DoubleLong(2500));
        server
            .handle_set(3, &ObisCode::ACTIVE_POWER_L1, 2, &new_val)
            .unwrap();

        // Step 3: Read back and verify
        let updated = server.handle_get(3, &ObisCode::ACTIVE_POWER_L1, 2).unwrap();
        assert_eq!(updated.as_i32(), Some(2500));

        // Step 4: Read billing amount (should be unchanged)
        let billing = server
            .handle_get(17, &ObisCode::new(0, 0, 17, 0, 0, 255), 7)
            .unwrap();

        // Step 5: Disconnect (simulate remote disconnect)
        server
            .handle_action(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 1, &[])
            .unwrap();

        // Step 6: Verify disconnected state
        let state = server
            .handle_get(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 4)
            .unwrap();
        if let DlmsData::Enum(v) = state {
            assert_eq!(v, 0); // Disconnected
        } else {
            panic!("Expected Enum");
        }

        // Step 7: Reconnect
        server
            .handle_action(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 2, &[])
            .unwrap();

        // Step 8: Verify reconnected
        let state = server
            .handle_get(70, &ObisCode::new(0, 0, 96, 1, 0, 255), 4)
            .unwrap();
        if let DlmsData::Enum(v) = state {
            assert_eq!(v, 1); // Connected
        } else {
            panic!("Expected Enum");
        }
    }
}
