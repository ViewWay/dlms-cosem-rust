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

mod activity_calendar;
mod billing;
mod clock;
mod data;
mod day_profile;
mod demand;
mod demand_register;
mod disconnect_control;
mod extended_register;
mod image_transfer;
mod infrared_setup;
mod lorawan_setup;
mod lp_setup;
mod maximum_demand;
mod nbiot_setup;
mod profile_generic;
mod push_setup;
mod register;
mod rs485_setup;
mod security_setup;
mod single_phase;
mod special_day_table;
mod tariff_plan;
mod total;
mod value_display;
mod week_profile;

pub use activity_calendar::ActivityCalendar;
pub use billing::Billing;
pub use clock::Clock;
pub use data::Data;
pub use day_profile::DayProfile;
pub use demand::Demand;
pub use demand_register::DemandRegister;
pub use disconnect_control::{DisconnectControl, DisconnectState};
pub use extended_register::ExtendedRegister;
pub use image_transfer::{ImageTransfer, ImageTransferStatus};
pub use infrared_setup::InfraredSetup;
pub use lorawan_setup::LorawanSetup;
pub use lp_setup::LpSetup;
pub use maximum_demand::MaximumDemand;
pub use nbiot_setup::NbiotSetup;
pub use profile_generic::ProfileGeneric;
pub use push_setup::PushSetup;
pub use register::Register;
pub use rs485_setup::Rs485Setup;
pub use security_setup::SecuritySetup;
pub use single_phase::SinglePhase;
pub use special_day_table::SpecialDayTable;
pub use tariff_plan::TariffPlan;
pub use total::Total;
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
