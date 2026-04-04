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
mod account_bluebook;
mod active_power_import;
mod activity_calendar;
mod actuator;
mod actuator_setup;
mod application_context;
mod arbitrator;
mod auto_connect;
mod billing;
mod charge;
mod clock;
mod clock_control;
mod cluster;
mod compact_data;
mod cosem_data_protection;
mod credit;
mod data;
mod day_profile;
mod demand;
mod demand_register;
mod direct_disconnect;
mod disconnect_control;
mod event_log;
mod extended_register;
mod gas_flow;
mod gas_valve;
mod gprs_modem_setup;
mod han_setup;
mod heat_cost_allocator;
mod iec_hdlc_setup;
mod image_transfer;
mod infrared_setup;
mod ipv4_tcp_setup;
mod ipv4_udp_setup;
mod ipv6_setup;
mod ipv6_tcp_setup;
mod lift_management;
mod limiter;
mod load_profile;
mod load_profile_switch;
mod local_display;
mod lorawan_setup;
mod lp_setup;
mod mac_address_setup;
mod maximum_demand;
mod mbus_master;
mod mbus_master_port_setup;
mod mbus_slave_setup;
mod measurement_data_monitoring;
mod modem_configuration;
mod multiplier;
mod multiplier_setup;
mod nbiot_setup;
mod ntp_setup;
mod parameter_monitor;
mod profile_generic;
mod push_setup;
mod quality_of_service;
mod register;
mod register_activation;
mod register_monitor;
mod route;
mod route_setup;
mod rs485_setup;
mod sap_assignment;
mod scheduled_activity;
mod security_setup;
mod security_setup_bluebook;
mod sensor;
mod sensor_manager;
mod sensor_setup;
mod serial_port;
mod single_action_schedule;
mod single_phase;
mod single_phase_export;
mod single_phase_import;
mod single_phase_mq;
mod special_day_table;
mod status_diag;
mod supply_disabling;
mod tariff_plan;
mod tariff_schedule;
mod tcp_udp_setup;
mod total;
mod transport;
mod ups;
mod utility_sub_schedule;
mod value_display;
mod water_flow;
mod week_profile;

pub use account::Account;
pub use account_bluebook::AccountBluebook;
pub use active_power_import::ActivePowerImport;
pub use activity_calendar::ActivityCalendar;
pub use actuator::{Actuator, ActuatorState};
pub use actuator_setup::ActuatorSetup;
pub use application_context::ApplicationContext;
pub use arbitrator::Arbitrator;
pub use auto_connect::AutoConnect;
pub use billing::Billing;
pub use charge::Charge;
pub use clock::Clock;
pub use clock_control::ClockControl;
pub use cluster::Cluster;
pub use compact_data::CompactData;
pub use cosem_data_protection::CosemDataProtection;
pub use credit::Credit;
pub use data::Data;
pub use day_profile::DayProfile;
pub use demand::Demand;
pub use demand_register::DemandRegister;
pub use direct_disconnect::DirectDisconnect;
pub use disconnect_control::{DisconnectControl, DisconnectState};
pub use event_log::EventLog;
pub use extended_register::ExtendedRegister;
pub use gas_flow::GasFlow;
pub use gas_valve::{GasValve, GasValveState};
pub use gprs_modem_setup::GprsModemSetup;
pub use han_setup::HanSetup;
pub use heat_cost_allocator::HeatCostAllocator;
pub use iec_hdlc_setup::IecHdlcSetup;
pub use image_transfer::{ImageTransfer, ImageTransferStatus};
pub use infrared_setup::InfraredSetup;
pub use ipv4_tcp_setup::Ipv4TcpSetup;
pub use ipv4_udp_setup::Ipv4UdpSetup;
pub use ipv6_setup::Ipv6Setup;
pub use ipv6_tcp_setup::Ipv6TcpSetup;
pub use lift_management::LiftManagement;
pub use limiter::Limiter;
pub use load_profile::LoadProfile;
pub use load_profile_switch::UtilityTables;
pub use local_display::LocalDisplay;
pub use lorawan_setup::LorawanSetup;
pub use lp_setup::LpSetup;
pub use mac_address_setup::MacAddressSetup;
pub use maximum_demand::MaximumDemand;
pub use mbus_master::MbusMaster;
pub use mbus_master_port_setup::MbusMasterPortSetup;
pub use mbus_slave_setup::MbusSlaveSetup;
pub use measurement_data_monitoring::MeasurementDataMonitoring;
pub use modem_configuration::ModemConfiguration;
pub use multiplier::Multiplier;
pub use multiplier_setup::MultiplierSetup;
pub use nbiot_setup::NbiotSetup;
pub use ntp_setup::NtpSetup;
pub use parameter_monitor::ParameterMonitor;
pub use profile_generic::ProfileGeneric;
pub use push_setup::PushSetup;
pub use quality_of_service::{QosEntry, QualityOfService};
pub use register::Register;
pub use register_activation::{RegisterActivation, RegisterAssignment};
pub use register_monitor::{RegisterMonitor, Thresholds};
pub use route::Route;
pub use route_setup::RouteSetup;
pub use rs485_setup::Rs485Setup;
pub use sap_assignment::SapAssignment;
pub use scheduled_activity::ScheduledActivity;
pub use security_setup::SecuritySetup;
pub use security_setup_bluebook::SecuritySetupBluebook;
pub use sensor::Sensor;
pub use sensor_manager::{SensorEntry, SensorManager};
pub use sensor_setup::SensorSetup;
pub use serial_port::SerialPort;
pub use single_action_schedule::{ActionScheduleEntry, SingleActionSchedule};
pub use single_phase::SinglePhase;
pub use single_phase_export::SinglePhaseExport;
pub use single_phase_import::SinglePhaseImport;
pub use single_phase_mq::SinglePhaseMq;
pub use special_day_table::SpecialDayTable;
pub use status_diag::StatusDiag;
pub use supply_disabling::SupplyDisabling;
pub use tariff_plan::TariffPlan;
pub use tariff_schedule::TariffSchedule;
pub use tcp_udp_setup::TcpUdpSetup;
pub use total::Total;
pub use transport::Transport;
pub use ups::Ups;
pub use utility_sub_schedule::{SubScheduleEntry, UtilitySubSchedule};
pub use value_display::ValueDisplay;
pub use water_flow::WaterFlow;
pub use week_profile::WeekProfile;

/// COSEM Interface Class IDs (Blue Book Ed16 accurate)
pub mod class_id {
    pub const DATA: u16 = 1;
    pub const REGISTER: u16 = 3;
    pub const EXTENDED_REGISTER: u16 = 4;
    pub const DEMAND_REGISTER: u16 = 5;
    pub const REGISTER_ACTIVATION: u16 = 6;
    pub const PROFILE_GENERIC: u16 = 7;
    pub const CLOCK: u16 = 8;
    pub const SCRIPT_TABLE: u16 = 9;
    pub const SCHEDULE: u16 = 10;
    pub const SPECIAL_DAY_TABLE: u16 = 11;
    pub const ASSOCIATION_SN: u16 = 12;
    pub const ASSOCIATION_LN: u16 = 15;
    pub const SAP_ASSIGNMENT: u16 = 17;
    pub const IMAGE_TRANSFER: u16 = 18;
    pub const IEC_LOCAL_PORT_SETUP: u16 = 19;
    pub const ACTIVITY_CALENDAR: u16 = 20;
    pub const REGISTER_MONITOR: u16 = 21;
    pub const SINGLE_ACTION_SCHEDULE: u16 = 22;
    pub const IEC_HDLC_SETUP: u16 = 23;
    pub const MBUS_SLAVE_PORT_SETUP: u16 = 25;
    pub const UTILITY_TABLES: u16 = 26;
    pub const MODEM_CONFIGURATION: u16 = 27;
    pub const AUTO_CONNECT: u16 = 29;
    pub const COSEM_DATA_PROTECTION: u16 = 30;
    pub const PUSH_SETUP: u16 = 40;
    pub const TCP_UDP_SETUP: u16 = 41;
    pub const IPV4_SETUP: u16 = 42;
    pub const MAC_ADDRESS_SETUP: u16 = 43;
    pub const GPRS_MODEM_SETUP: u16 = 45;
    pub const SMTP_SETUP: u16 = 46;
    pub const IPV6_SETUP: u16 = 48;
    pub const COMPACT_DATA: u16 = 62;
    pub const STATUS_MAPPING: u16 = 63;
    pub const SECURITY_SETUP: u16 = 64;
    pub const DEMAND: u16 = 10;
    pub const BILLING: u16 = 17;
    pub const TOTAL: u16 = 20;
    pub const SINGLE_PHASE: u16 = 31;
    pub const MAXIMUM_DEMAND: u16 = 34;
    pub const PARAMETER_MONITOR: u16 = 65;
    pub const MEASUREMENT_DATA_MONITORING: u16 = 66;
    pub const SENSOR_MANAGER: u16 = 67;
    pub const ARBITRATOR: u16 = 68;
    pub const DISCONNECT_CONTROL: u16 = 70;
    pub const LIMITER: u16 = 71;
    pub const MBUS_CLIENT: u16 = 72;
    pub const APPLICATION_CONTEXT: u16 = 88;
    pub const NTP_SETUP: u16 = 100;
    pub const ACCOUNT: u16 = 111;
    pub const CREDIT: u16 = 112;
    pub const CHARGE: u16 = 113;
    // Custom utility class IDs (200+, not in Blue Book)
    pub const CUSTOM_QOS: u16 = 200;
    pub const CUSTOM_SENSOR: u16 = 201;
    pub const CUSTOM_SENSOR_SETUP: u16 = 202;
    pub const CUSTOM_ACTUATOR: u16 = 203;
    pub const CUSTOM_ACTUATOR_SETUP: u16 = 204;
    pub const CUSTOM_GAS_FLOW: u16 = 205;
    pub const CUSTOM_GAS_VALVE: u16 = 206;
    pub const CUSTOM_WATER_FLOW: u16 = 207;
    pub const CUSTOM_HEAT_COST_ALLOCATOR: u16 = 208;
    pub const CUSTOM_HAN_SETUP: u16 = 209;
    pub const CUSTOM_ACTIVE_POWER_IMPORT: u16 = 210;
    pub const CUSTOM_SINGLE_PHASE_IMPORT: u16 = 211;
    pub const CUSTOM_SINGLE_PHASE_EXPORT: u16 = 212;
    pub const CUSTOM_ROUTE_SETUP: u16 = 213;
    pub const CUSTOM_TRANSPORT: u16 = 214;
    pub const CUSTOM_MULTIPLIER: u16 = 215;
    pub const CUSTOM_MULTIPLIER_SETUP: u16 = 216;
    pub const CUSTOM_UTILITY_SUB_SCHEDULE: u16 = 217;
    pub const CUSTOM_CLUSTER: u16 = 218;
    pub const CUSTOM_ROUTE: u16 = 219;
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
