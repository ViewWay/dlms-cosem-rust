//! dlms-cosem: COSEM Interface Classes for DLMS/COSEM
//!
//! Implements core IC (Interface Class) objects:
//! - IC001 Data, IC003 Register, IC004 ExtendedRegister, IC005 DemandRegister
//! - IC007 Profile Generic
//! - IC008 Clock, IC010 ScriptTable, IC011 SpecialDayTable
//! - IC012 Schedule, IC022 Module, IC029 ValueTable
//! - IC017 Billing, IC020 Total, IC031 Single Phase, IC034 Maximum Demand
//! - IC018 TariffPlan, IC021 WeekProfile, IC022 DayProfile
//! - IC070 DisconnectControl, SecuritySetup
//! - IC100 LP Setup, IC101 RS485 Setup, IC102 Infrared Setup, IC106 NB-IoT Setup, IC107 LoRaWAN Setup

#![cfg_attr(not(feature = "std"), no_std)]

mod C111_Account;
mod account_bluebook;
mod active_power_import;
mod C20_ActivityCalendar;
mod actuator;
mod actuator_setup;
mod application_context;
mod C68_Arbitrator;
mod alarm_handler;
mod C123_ArrayManager;
mod C15_AssociationLN;
mod C12_AssociationSN;
mod C29_AutoConnect;
mod billing;
mod calendar;
mod C113_Charge;
mod C8_Clock;
mod clock_control;
mod cluster;
mod comm_control;
mod C124_CommPortProtection;
mod C62_CompactData;
mod C30_COSEMDataProtection;
mod C112_Credit;
mod data;
mod data_logger;
mod data_storage;
mod day_profile;
mod demand;
mod C5_DemandRegister;
mod direct_disconnect;
mod C70_DisconnectControl;
mod event_log;
mod event_logger;
mod ethernet_setup;
mod extended_profile;
mod C4_ExtendedRegister;
mod C122_FunctionControl;
mod firmware_management;
mod gas_flow;
mod gas_valve;
mod C47_GSMDiagnostic;
mod C50_SFSKPhyMACSetup;
mod C51_SFSKActiveInitiator;
mod C52_SFSKMACSyncTimeouts;
mod C53_SFSKMACCounters;
mod C55_IEC61334LLCSetup;
mod C56_SFSKReportingSystemList;
mod C45_GPRSModemSetup;
mod harmonic_monitor;
mod han_setup;
mod heat_cost_allocator;
mod iec62055_attributes;
mod C57_LLCType1Setup;
mod C58_LLCType2Setup;
mod C59_LLCType3Setup;
mod C61_RegisterTable;
mod C23_IECHDLCSetup;
mod iec_public_key;
mod C18_ImageTransfer;
mod infrared_setup;
mod C42_IPv4Setup;
mod ipv4_tcp_setup;
mod ipv4_udp_setup;
mod C48_IPv6Setup;
mod ipv6_tcp_setup;
mod lift_management;
mod C26_UtilityTables;
mod C71_Limiter;
mod load_profile;
mod load_profile_switch;
mod local_display;
mod C90_G3MACCounters;
mod C91_G3MACSetup;
mod C92_G36LoWPANSetup;
mod C101_ZigbeeSASStartup;
mod C102_ZigbeeSASJoin;
mod C103_ZigbeeSASAPSFragmentation;
mod C104_ZigbeeNetworkControl;
mod C105_ZigbeeTunnelSetup;
mod C126_SCHCLPWANSetup;
mod C127_SCHCLPWANDiagnostic;
mod C129_LoRaWANDiagnostic;
mod C128_LoRaWANSetup;
mod lp_setup;
mod C130_IEC14908Identification;
mod C131_IEC14908ProtocolSetup;
mod C132_IEC14908ProtocolStatus;
mod C133_IEC14908Diagnostic;
mod C140_HSPLCMACSetup;
mod C141_HSPLCCPASSetup;
mod C142_HSPLCIPSSASSetup;
mod C143_HSPLCHDLCSSASSetup;
mod C152_CoAPSetup;
mod C153_CoAPDiagnostic;
mod C160_G3HybridRFCounters;
mod C161_G3HybridRFSetup;
mod C162_G3Hybrid6LoWPANSetup;
mod C151_LTEMonitoring;
mod module;
mod C43_MACAddressSetup;
mod maximum_demand;
mod C73_WirelessModeQChannel;
mod C76_DLMSMBusPortSetup;
mod C72_MBusClient;
mod C80_PRIMELLCSSCSSetup;
mod C81_PRIMEPhysicalCounters;
mod C82_PRIMEMACSetup;
mod C83_PRIMEMACFuncParams;
mod C84_PRIMEMACCounters;
mod C85_PRIMEMACNetworkAdmin;
mod C86_PRIMEAppIdentification;
mod C77_MBusDiagnostic;
mod mbus_master;
mod C74_MBusMasterPortSetup;
mod mbus_slave_setup;
mod C66_MeasurementDataMonitoring;
mod C28_AutoAnswer;
mod C27_ModemConfiguration;
mod mpl_diagnostic;
mod multiplier;
mod multiplier_setup;
mod nbiot_setup;
mod C100_NTPSetup;
mod C65_ParameterMonitor;
mod power_quality_monitor;
mod C44_PPPSetup;
mod C7_ProfileGeneric;
mod C40_PushSetup;
mod quality_of_service;
mod C3_Register;
mod C6_RegisterActivation;
mod C19_IECLocalPortSetup;
mod C24_IECTwistedPairSetup;
mod C25_MBusSlavePortSetup;
mod C21_RegisterMonitor;
mod route;
mod route_setup;
mod rpl_diagnostic;
mod rs485_setup;
mod sag_swell_monitor;
mod C17_SAPAssignment;
mod scheduled_activity;
mod C10_Schedule;
mod C64_SecuritySetup;
mod security_setup_bluebook;
mod sensor;
mod C67_SensorManager;
mod sensor_setup;
mod serial_port;
mod C22_SingleActionSchedule;
mod C9_ScriptTable;
mod C46_SMTPSetup;
mod standard_readout;
mod single_phase;
mod single_phase_export;
mod single_phase_import;
mod single_phase_mq;
mod C11_SpecialDaysTable;
mod status_diag;
mod C63_StatusMapping;
mod supply_disabling;
mod tariff_plan;
mod tariff_schedule;
mod C41_TCPUDPSetup;
mod tls_setup;
mod C115_TokenGateway;
mod total;
mod transport;
mod ups;
mod utility_sub_schedule;
mod value_display;
mod value_table;
mod water_flow;
mod week_profile;
mod wisun_diagnostic;
mod wisun_setup;
mod zigbee_setup;
mod identity;

pub use C111_Account::Account;
pub use account_bluebook::AccountBluebook;
pub use active_power_import::ActivePowerImport;
pub use C19_IECLocalPortSetup::IecLocalPortSetup;
pub use C24_IECTwistedPairSetup::IecTwistedPairSetup;
pub use C25_MBusSlavePortSetup::MBusSlavePortSetup;
pub use C20_ActivityCalendar::ActivityCalendar;
pub use actuator::{Actuator, ActuatorState};
pub use actuator_setup::ActuatorSetup;
pub use application_context::ApplicationContext;
pub use C68_Arbitrator::Arbitrator;
pub use alarm_handler::{AlarmHandler, Alarm};
pub use C123_ArrayManager::ArrayManager;
pub use C15_AssociationLN::AssociationLN;
pub use C12_AssociationSN::AssociationSN;
pub use C29_AutoConnect::AutoConnect;
pub use C28_AutoAnswer::AutoAnswer;
pub use billing::Billing;
pub use calendar::{Calendar, SeasonEntry, WeekDayProfile};
pub use C113_Charge::Charge;
pub use C8_Clock::Clock;
pub use clock_control::ClockControl;
pub use comm_control::{CommControl, CommChannel};
pub use cluster::Cluster;
pub use C124_CommPortProtection::CommPortProtection;
pub use C62_CompactData::CompactData;
pub use C30_COSEMDataProtection::CosemDataProtection;
pub use C112_Credit::Credit;
pub use data::Data;
pub use data_logger::{DataLogger, DataLogEntry};
pub use data_storage::{DataStorage, DataStorageEntry};
pub use day_profile::DayProfile;
pub use demand::Demand;
pub use C5_DemandRegister::DemandRegister;
pub use direct_disconnect::DirectDisconnect;
pub use C70_DisconnectControl::{DisconnectControl, DisconnectState};
pub use event_log::EventLog;
pub use event_logger::{EventLogger, EventLogEntry};
pub use ethernet_setup::{EthernetSetup, DuplexMode};
pub use extended_profile::{ExtendedProfile, ExtendedProfileEntry};
pub use C4_ExtendedRegister::ExtendedRegister;
pub use C122_FunctionControl::FunctionControl;
pub use firmware_management::FirmwareManagement;
pub use gas_flow::GasFlow;
pub use gas_valve::{GasValve, GasValveState};
pub use C47_GSMDiagnostic::GsmDiagnostic;
pub use C45_GPRSModemSetup::GprsModemSetup;
pub use C50_SFSKPhyMACSetup::SfskPhyMacSetup;
pub use C51_SFSKActiveInitiator::SfskActiveInitiator;
pub use C52_SFSKMACSyncTimeouts::SfskMacSyncTimeouts;
pub use C53_SFSKMACCounters::SfskMacCounters;
pub use C55_IEC61334LLCSetup::Iec61334LlcSetup;
pub use C56_SFSKReportingSystemList::SfskReportingSystemList;
pub use harmonic_monitor::{HarmonicMonitor, MonitoringMode};
pub use han_setup::HanSetup;
pub use heat_cost_allocator::HeatCostAllocator;
pub use iec62055_attributes::Iec62055Attributes;
pub use C57_LLCType1Setup::LlcType1Setup;
pub use C58_LLCType2Setup::LlcType2Setup;
pub use C59_LLCType3Setup::LlcType3Setup;
pub use C61_RegisterTable::RegisterTable;
pub use C23_IECHDLCSetup::IecHdlcSetup;
pub use iec_public_key::{IecPublicKey, KeyAlgorithm, KeyUsage};
pub use C18_ImageTransfer::{ImageTransfer, ImageTransferStatus};
pub use infrared_setup::InfraredSetup;
pub use C42_IPv4Setup::Ipv4Setup;
pub use ipv4_tcp_setup::Ipv4TcpSetup;
pub use ipv4_udp_setup::Ipv4UdpSetup;
pub use C48_IPv6Setup::Ipv6Setup;
pub use ipv6_tcp_setup::Ipv6TcpSetup;
pub use lift_management::LiftManagement;
pub use C26_UtilityTables::UtilityTables26;
pub use C71_Limiter::Limiter;
pub use load_profile::LoadProfile;
pub use load_profile_switch::UtilityTables;
pub use local_display::LocalDisplay;
pub use C90_G3MACCounters::G3MacCounters;
pub use C91_G3MACSetup::G3MacSetup;
pub use C92_G36LoWPANSetup::G36LoWPANSetup;
pub use C101_ZigbeeSASStartup::ZigbeeSasStartup;
pub use C102_ZigbeeSASJoin::ZigbeeSasJoin;
pub use C103_ZigbeeSASAPSFragmentation::ZigbeeSasApsFragmentation;
pub use C104_ZigbeeNetworkControl::ZigbeeNetworkControl;
pub use C105_ZigbeeTunnelSetup::ZigbeeTunnelSetup;
pub use C126_SCHCLPWANSetup::SchcLpwanSetup;
pub use C127_SCHCLPWANDiagnostic::SchcLpwanDiagnostic;
pub use C129_LoRaWANDiagnostic::LoRaWanDiagnostic;
pub use C128_LoRaWANSetup::LorawanSetup;
pub use lp_setup::LpSetup;
pub use C130_IEC14908Identification::Iec14908Identification;
pub use C131_IEC14908ProtocolSetup::Iec14908ProtocolSetup;
pub use C132_IEC14908ProtocolStatus::Iec14908ProtocolStatus;
pub use C133_IEC14908Diagnostic::Iec14908Diagnostic;
pub use C140_HSPLCMACSetup::HsplcMacSetup;
pub use C141_HSPLCCPASSetup::HsplcCpasSetup;
pub use C142_HSPLCIPSSASSetup::HsplcIpSsasSetup;
pub use C143_HSPLCHDLCSSASSetup::HsplcHdlcSsasSetup;
pub use C152_CoAPSetup::CoapSetup;
pub use C153_CoAPDiagnostic::CoapDiagnostic;
pub use C160_G3HybridRFCounters::G3HybridRfCounters;
pub use C161_G3HybridRFSetup::G3HybridRfSetup;
pub use C162_G3Hybrid6LoWPANSetup::G3Hybrid6LoWPANSetup;
pub use C151_LTEMonitoring::{LteSetup, PdpType, NetworkMode, RegistrationStatus};
pub use module::{Module, ModuleStatus};
pub use C43_MACAddressSetup::MacAddressSetup;
pub use maximum_demand::MaximumDemand;
pub use C73_WirelessModeQChannel::WirelessModeQChannel;
pub use C76_DLMSMBusPortSetup::DlmsMBusPortSetup;
pub use C72_MBusClient::MbusClient;
pub use C80_PRIMELLCSSCSSetup::PrimeLlcSscsSetup;
pub use C81_PRIMEPhysicalCounters::PrimePhysicalCounters;
pub use C82_PRIMEMACSetup::PrimeMacSetup;
pub use C83_PRIMEMACFuncParams::PrimeMacFuncParams;
pub use C84_PRIMEMACCounters::PrimeMacCounters;
pub use C85_PRIMEMACNetworkAdmin::PrimeMacNetworkAdmin;
pub use C86_PRIMEAppIdentification::PrimeAppIdentification;
pub use C77_MBusDiagnostic::MbusDiagnostic;
pub use mbus_master::MbusMaster;
pub use C74_MBusMasterPortSetup::MbusMasterPortSetup;
pub use mbus_slave_setup::MbusSlaveSetup;
pub use C66_MeasurementDataMonitoring::MeasurementDataMonitoring;
pub use C27_ModemConfiguration::ModemConfiguration;
pub use mpl_diagnostic::MplDiagnostic;
pub use multiplier::Multiplier;
pub use multiplier_setup::MultiplierSetup;
pub use nbiot_setup::NbiotSetup;
pub use C100_NTPSetup::NtpSetup;
pub use C65_ParameterMonitor::ParameterMonitor;
pub use power_quality_monitor::PowerQualityMonitor;
pub use C44_PPPSetup::{PppAuthProtocol, PppSetup};
pub use C7_ProfileGeneric::ProfileGeneric;
pub use standard_readout::StandardReadout;
pub use C40_PushSetup::PushSetup;
pub use quality_of_service::{QosEntry, QualityOfService};
pub use C3_Register::Register;
pub use C6_RegisterActivation::{RegisterActivation, RegisterAssignment};
pub use C21_RegisterMonitor::{RegisterMonitor, Thresholds};
pub use route::Route;
pub use route_setup::RouteSetup;
pub use rpl_diagnostic::RplDiagnostic;
pub use rs485_setup::Rs485Setup;
pub use sag_swell_monitor::{SagSwellMonitor, SagSwellEvent};
pub use C17_SAPAssignment::SapAssignment;
pub use scheduled_activity::ScheduledActivity;
pub use C10_Schedule::{Schedule, ScheduleEntry};
pub use C64_SecuritySetup::SecuritySetup;
pub use security_setup_bluebook::SecuritySetupBluebook;
pub use sensor::Sensor;
pub use C67_SensorManager::{SensorEntry, SensorManager};
pub use sensor_setup::SensorSetup;
pub use serial_port::SerialPort;
pub use C22_SingleActionSchedule::{ActionScheduleEntry, SingleActionSchedule};
pub use C9_ScriptTable::{Script, ScriptTable};
pub use C46_SMTPSetup::SmtpSetup;
pub use single_phase::SinglePhase;
pub use single_phase_export::SinglePhaseExport;
pub use single_phase_import::SinglePhaseImport;
pub use single_phase_mq::SinglePhaseMq;
pub use C11_SpecialDaysTable::SpecialDayTable;
pub use status_diag::StatusDiag;
pub use C63_StatusMapping::StatusMapping;
pub use supply_disabling::SupplyDisabling;
pub use tariff_plan::TariffPlan;
pub use tariff_schedule::TariffSchedule;
pub use C41_TCPUDPSetup::TcpUdpSetup;
pub use tls_setup::{TlsSetup, TlsVersion};
pub use C115_TokenGateway::TokenGateway;
pub use total::Total;
pub use transport::Transport;
pub use ups::Ups;
pub use utility_sub_schedule::{SubScheduleEntry, UtilitySubSchedule};
pub use value_display::ValueDisplay;
pub use value_table::{ValueDescriptor, ValueEntry, ValueTable};
pub use water_flow::WaterFlow;
pub use week_profile::WeekProfile;
pub use wisun_diagnostic::WiSunDiagnostic;
pub use wisun_setup::WiSunSetup;
pub use zigbee_setup::{ZigBeeDeviceType, ZigBeeSetup};
pub use identity::Identity;

/// COSEM Interface Class IDs (Blue Book Ed16 accurate)
pub mod class_id {
    pub const DATA: u16 = 1;
    pub const ASSOCIATION_SN: u16 = 2;
    pub const REGISTER: u16 = 3;
    pub const EXTENDED_REGISTER: u16 = 4;
    pub const DEMAND_REGISTER: u16 = 5;
    pub const REGISTER_ACTIVATION: u16 = 6;
    pub const PROFILE_GENERIC: u16 = 7;
    pub const CLOCK: u16 = 8;
    pub const ASSOCIATION_LN: u16 = 9;
    pub const DEMAND: u16 = 10;
    pub const SPECIAL_DAY_TABLE: u16 = 11;
    pub const SCHEDULE: u16 = 12;
    pub const ACTIVITY_CALENDAR: u16 = 20;
    pub const REGISTER_MONITOR: u16 = 21;
    pub const SINGLE_ACTION_SCHEDULE: u16 = 22;
    pub const MODULE: u16 = 22;
    pub const IEC_HDLC_SETUP: u16 = 23;
    pub const MBUS_SLAVE_PORT_SETUP: u16 = 25;
    pub const UTILITY_TABLES: u16 = 26;
    pub const MODEM_CONFIGURATION: u16 = 27;
    pub const ZIGBEE_SETUP: u16 = 27; // Same as MODEM_CONFIGURATION
    pub const AUTO_CONNECT: u16 = 29;
    pub const VALUE_TABLE: u16 = 29;
    pub const COSEM_DATA_PROTECTION: u16 = 30;
    pub const IDENTITY: u16 = 40;
    pub const PUSH_SETUP: u16 = 40; // Same as IDENTITY
    pub const PPP_SETUP: u16 = 44;
    pub const TCP_UDP_SETUP: u16 = 41;
    pub const IPV4_SETUP: u16 = 42;
    pub const MAC_ADDRESS_SETUP: u16 = 43;
    pub const GPRS_MODEM_SETUP: u16 = 45;
    pub const SMTP_SETUP: u16 = 46;
    pub const IPV6_SETUP: u16 = 48;
    pub const COMPACT_DATA: u16 = 62;
    pub const STATUS_MAPPING: u16 = 63;
    pub const SECURITY_SETUP: u16 = 64;
    pub const PARAMETER_MONITOR: u16 = 65;
    pub const MEASUREMENT_DATA_MONITORING: u16 = 66;
    pub const SENSOR_MANAGER: u16 = 67;
    pub const ARBITRATOR: u16 = 68;
    pub const DISCONNECT_CONTROL: u16 = 70;
    pub const LIMITER: u16 = 71;
    pub const MBUS_CLIENT: u16 = 72;
    pub const STANDARD_READOUT: u16 = 23;
    pub const APPLICATION_CONTEXT: u16 = 88;
    pub const IEC_PUBLIC_KEY: u16 = 90;
    pub const NTP_SETUP: u16 = 100;
    pub const WISUN_SETUP: u16 = 95;
    pub const WISUN_DIAGNOSTIC: u16 = 96;
    pub const RPL_DIAGNOSTIC: u16 = 97;
    pub const MPL_DIAGNOSTIC: u16 = 98;
    pub const ETHERNET_SETUP: u16 = 103;
    pub const LTE_SETUP: u16 = 104;
    pub const TLS_SETUP: u16 = 105;
    pub const MBUS_DIAGNOSTIC: u16 = 110;
    pub const ACCOUNT: u16 = 111;
    pub const POWER_QUALITY_MONITOR: u16 = 111;
    pub const CREDIT: u16 = 112;
    pub const HARMONIC_MONITOR: u16 = 112;
    pub const CHARGE: u16 = 113;
    pub const SAG_SWELL_MONITOR: u16 = 113;
    pub const TOKEN_GATEWAY: u16 = 115;
    pub const IEC_62055_ATTRIBUTES: u16 = 116;
    pub const FUNCTION_CONTROL: u16 = 122;
    pub const ARRAY_MANAGER: u16 = 123;
    pub const COMM_PORT_PROTECTION: u16 = 124;
    // Legacy/project-specific IDs
    pub const BILLING: u16 = 17;
    pub const TOTAL: u16 = 20;
    pub const SINGLE_PHASE: u16 = 31;
    pub const MAXIMUM_DEMAND: u16 = 34;
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
