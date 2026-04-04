//! dlms-cosem: COSEM Interface Classes for DLMS/COSEM
//!
//! Implements core IC (Interface Class) objects:
//! - IC001 Data, IC003 Register, IC004 ExtendedRegister, IC005 DemandRegister
//! - IC007 Profile Generic
//! - IC008 Clock, IC010 Demand, IC011 SpecialDayTable
//! - IC017 Billing, IC020 Total, IC031 Single Phase, IC034 Maximum Demand
//! - IC018 TariffPlan, IC021 WeekProfile, IC022 DayProfile
//! - IC070 DisconnectControl, SecuritySetup
//! - IC100 LP Setup, IC101 RS485 Setup, IC102 Infrared Setup, IC106 NB-IoT Setup, IC107 LoRaWAN Setup

#![cfg_attr(not(feature = "std"), no_std)]

mod account;
mod activity_calendar;
mod application_context;
mod auto_connect;
mod billing;
mod clock;
mod clock_control;
mod data;
mod day_profile;
mod demand;
mod demand_register;
mod direct_disconnect;
mod disconnect_control;
mod event_log;
mod extended_register;
mod gprs_modem_setup;
mod iec_hdlc_setup;
mod image_transfer;
mod infrared_setup;
mod ipv4_tcp_setup;
mod ipv4_udp_setup;
mod ipv6_setup;
mod ipv6_tcp_setup;
mod lift_management;
mod load_profile;
mod local_display;
mod lorawan_setup;
mod lp_setup;
mod maximum_demand;
mod mbus_master;
mod mbus_master_port_setup;
mod mbus_slave_setup;
mod modem_configuration;
mod nbiot_setup;
mod profile_generic;
mod push_setup;
mod register;
mod rs485_setup;
mod sap_assignment;
mod scheduled_activity;
mod security_setup;
mod serial_port;
mod single_phase;
mod single_phase_mq;
mod special_day_table;
mod status_diag;
mod supply_disabling;
mod tariff_plan;
mod tariff_schedule;
mod tcp_udp_setup;
mod total;
mod ups;
mod value_display;
mod week_profile;

pub use account::Account;
pub use activity_calendar::ActivityCalendar;
pub use application_context::ApplicationContext;
pub use auto_connect::AutoConnect;
pub use billing::Billing;
pub use clock::Clock;
pub use clock_control::ClockControl;
pub use data::Data;
pub use day_profile::DayProfile;
pub use demand::Demand;
pub use demand_register::DemandRegister;
pub use direct_disconnect::DirectDisconnect;
pub use disconnect_control::{DisconnectControl, DisconnectState};
pub use event_log::EventLog;
pub use extended_register::ExtendedRegister;
pub use gprs_modem_setup::GprsModemSetup;
pub use iec_hdlc_setup::IecHdlcSetup;
pub use image_transfer::{ImageTransfer, ImageTransferStatus};
pub use infrared_setup::InfraredSetup;
pub use ipv4_tcp_setup::Ipv4TcpSetup;
pub use ipv4_udp_setup::Ipv4UdpSetup;
pub use ipv6_setup::Ipv6Setup;
pub use ipv6_tcp_setup::Ipv6TcpSetup;
pub use lift_management::LiftManagement;
pub use load_profile::LoadProfile;
pub use local_display::LocalDisplay;
pub use lorawan_setup::LorawanSetup;
pub use lp_setup::LpSetup;
pub use maximum_demand::MaximumDemand;
pub use mbus_master::MbusMaster;
pub use mbus_master_port_setup::MbusMasterPortSetup;
pub use mbus_slave_setup::MbusSlaveSetup;
pub use modem_configuration::ModemConfiguration;
pub use nbiot_setup::NbiotSetup;
pub use profile_generic::ProfileGeneric;
pub use push_setup::PushSetup;
pub use register::Register;
pub use rs485_setup::Rs485Setup;
pub use sap_assignment::SapAssignment;
pub use scheduled_activity::ScheduledActivity;
pub use security_setup::SecuritySetup;
pub use serial_port::SerialPort;
pub use single_phase::SinglePhase;
pub use single_phase_mq::SinglePhaseMq;
pub use special_day_table::SpecialDayTable;
pub use status_diag::StatusDiag;
pub use supply_disabling::SupplyDisabling;
pub use tariff_plan::TariffPlan;
pub use tariff_schedule::TariffSchedule;
pub use tcp_udp_setup::TcpUdpSetup;
pub use total::Total;
pub use ups::Ups;
pub use value_display::ValueDisplay;
pub use week_profile::WeekProfile;

/// COSEM Interface Class IDs
pub mod class_id {
    pub const DATA: u16 = 1;
    pub const REGISTER: u16 = 3;
    pub const EXTENDED_REGISTER: u16 = 4;
    pub const DEMAND_REGISTER: u16 = 5;
    pub const PROFILE_GENERIC: u16 = 7;
    pub const CLOCK: u16 = 8;
    pub const DEMAND: u16 = 10;
    pub const SPECIAL_DAY_TABLE: u16 = 11;
    pub const BILLING: u16 = 17;
    pub const TARIFF_PLAN: u16 = 18;
    pub const TARIFF_TABLE: u16 = 19;
    pub const TOTAL: u16 = 20;
    pub const WEEK_PROFILE: u16 = 21;
    pub const DAY_PROFILE: u16 = 22;
    pub const SINGLE_PHASE: u16 = 31;
    pub const MAXIMUM_DEMAND: u16 = 34;
    pub const DISCONNECT_CONTROL: u16 = 70;
    pub const SECURITY_SETUP: u16 = 70;
    pub const LP_SETUP: u16 = 100;
    pub const RS485_SETUP: u16 = 101;
    pub const INFRARED_SETUP: u16 = 102;
    pub const NBIOT_SETUP: u16 = 106;
    pub const LORAWAN_SETUP: u16 = 107;
    // New IC classes
    pub const LOAD_PROFILE: u16 = 4;
    pub const SUPPLY_DISABLING: u16 = 6;
    pub const LOCAL_DISPLAY: u16 = 9;
    pub const SAP_ASSIGNMENT: u16 = 13;
    pub const SCHEDULED_ACTIVITY: u16 = 14;
    pub const ACCOUNT: u16 = 16;
    pub const TARIFF_SCHEDULE: u16 = 19;
    pub const IEC_HDLC_SETUP: u16 = 23;
    pub const STATUS_DIAG: u16 = 27;
    pub const CLOCK_CONTROL: u16 = 28;
    pub const EVENT_LOG: u16 = 35;
    pub const SINGLE_PHASE_MQ: u16 = 36;
    pub const MODEM_CONFIGURATION: u16 = 42;
    pub const TCP_UDP_SETUP: u16 = 43;
    pub const IPV4_UDP_SETUP: u16 = 44;
    pub const IPV6_SETUP: u16 = 45;
    pub const IPV4_TCP_SETUP: u16 = 46;
    pub const IPV6_TCP_SETUP: u16 = 47;
    pub const MBUS_SLAVE_SETUP: u16 = 56;
    pub const MBUS_MASTER_PORT_SETUP: u16 = 57;
    pub const MBUS_MASTER: u16 = 58;
    pub const SERIAL_PORT: u16 = 60;
    pub const UPS: u16 = 61;
    pub const AUTO_CONNECT: u16 = 62;
    pub const DIRECT_DISCONNECT: u16 = 63;
    pub const GPRS_MODEM_SETUP: u16 = 69;
    pub const IMAGE_TRANSFER: u16 = 71;
    pub const LIFT_MANAGEMENT: u16 = 72;
    pub const APPLICATION_CONTEXT: u16 = 88;
}

#[cfg(test)]
mod tests {
    use super::*;
    use dlms_core::{CosemObject, DlmsData, ObisCode};

    #[test]
    fn test_data_class() {
        let obj = Data::new(ObisCode::new(1, 0, 0, 9, 0, 255), DlmsData::None);
        assert_eq!(obj.class_id(), 1);
    }

    #[test]
    fn test_register_class() {
        let obj = Register::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(1000));
        assert_eq!(obj.class_id(), 3);
    }

    #[test]
    fn test_clock_class() {
        let obj = Clock::new(ObisCode::CLOCK);
        assert_eq!(obj.class_id(), 8);
    }

    #[test]
    fn test_profile_generic_class() {
        let obj = ProfileGeneric::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        assert_eq!(obj.class_id(), 7);
    }

    #[test]
    fn test_demand_class() {
        let obj = Demand::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        assert_eq!(obj.class_id(), 10);
    }

    #[test]
    fn test_special_day_table_class() {
        let obj = SpecialDayTable::new(ObisCode::new(0, 0, 11, 0, 0, 255));
        assert_eq!(obj.class_id(), 11);
    }

    #[test]
    fn test_billing_class() {
        let obj = Billing::new(ObisCode::new(0, 0, 17, 0, 0, 255));
        assert_eq!(obj.class_id(), 17);
    }

    #[test]
    fn test_total_class() {
        let obj = Total::new(
            ObisCode::ACTIVE_ENERGY_IMPORT,
            DlmsData::DoubleLongUnsigned(0),
        );
        assert_eq!(obj.class_id(), 20);
    }

    #[test]
    fn test_single_phase_class() {
        let obj = SinglePhase::new(ObisCode::new(0, 0, 31, 0, 0, 255));
        assert_eq!(obj.class_id(), 31);
    }

    #[test]
    fn test_maximum_demand_class() {
        let obj = MaximumDemand::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(0));
        assert_eq!(obj.class_id(), 34);
    }

    #[test]
    fn test_tariff_plan_class() {
        let obj = TariffPlan::new(ObisCode::new(0, 0, 18, 0, 0, 255));
        assert_eq!(obj.class_id(), 18);
    }

    #[test]
    fn test_week_profile_class() {
        let obj = WeekProfile::new(ObisCode::new(0, 0, 21, 0, 0, 255));
        assert_eq!(obj.class_id(), 21);
    }

    #[test]
    fn test_day_profile_class() {
        let obj = DayProfile::new(ObisCode::new(0, 0, 22, 0, 0, 255));
        assert_eq!(obj.class_id(), 22);
    }

    #[test]
    fn test_disconnect_control_class() {
        let obj = DisconnectControl::new(ObisCode::new(0, 0, 96, 1, 0, 255));
        assert_eq!(obj.class_id(), 70);
    }

    #[test]
    fn test_class_id_constants() {
        assert_eq!(class_id::DATA, 1);
        assert_eq!(class_id::REGISTER, 3);
        assert_eq!(class_id::CLOCK, 8);
        assert_eq!(class_id::DEMAND, 10);
        assert_eq!(class_id::BILLING, 17);
        assert_eq!(class_id::TOTAL, 20);
        assert_eq!(class_id::SINGLE_PHASE, 31);
        assert_eq!(class_id::MAXIMUM_DEMAND, 34);
        assert_eq!(class_id::PROFILE_GENERIC, 7);
    }
}
