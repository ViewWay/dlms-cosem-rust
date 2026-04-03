//! dlms-cosem: COSEM Interface Classes for DLMS/COSEM
//!
//! Implements core IC (Interface Class) objects:
//! - IC001 Data, IC003 Register, IC004 ExtendedRegister, IC005 DemandRegister
//! - IC008 Clock, IC011 SpecialDayTable
//! - IC018 TariffPlan, IC019 TariffTable, IC020 SeasonProfile, IC021 WeekProfile, IC022 DayProfile
//! - IC070 SecuritySetup
//! - IC100 LP Setup, IC101 RS485 Setup, IC102 Infrared Setup, IC106 NB-IoT Setup, IC107 LoRaWAN Setup
//! - Profile Generic (load profile)

#![cfg_attr(not(feature = "std"), no_std)]

// no_std support: feature gate for no_std



mod data;
mod register;
mod extended_register;
mod demand_register;
mod clock;
mod profile_generic;
mod security_setup;
mod lp_setup;
mod rs485_setup;
mod infrared_setup;
mod nbiot_setup;
mod lorawan_setup;

pub use data::Data;
pub use register::Register;
pub use extended_register::ExtendedRegister;
pub use demand_register::DemandRegister;
pub use clock::Clock;
pub use profile_generic::ProfileGeneric;
pub use security_setup::SecuritySetup;
pub use lp_setup::LpSetup;
pub use rs485_setup::Rs485Setup;
pub use infrared_setup::InfraredSetup;
pub use nbiot_setup::NbiotSetup;
pub use lorawan_setup::LorawanSetup;

/// COSEM Interface Class IDs
pub mod class_id {
    pub const DATA: u16 = 1;
    pub const REGISTER: u16 = 3;
    pub const EXTENDED_REGISTER: u16 = 4;
    pub const DEMAND_REGISTER: u16 = 5;
    pub const CLOCK: u16 = 8;
    pub const SPECIAL_DAY_TABLE: u16 = 11;
    pub const TARIFF_PLAN: u16 = 18;
    pub const TARIFF_TABLE: u16 = 19;
    pub const SEASON_PROFILE: u16 = 20;
    pub const WEEK_PROFILE: u16 = 21;
    pub const DAY_PROFILE: u16 = 22;
    pub const SECURITY_SETUP: u16 = 70;
    pub const LP_SETUP: u16 = 100;
    pub const RS485_SETUP: u16 = 101;
    pub const INFRARED_SETUP: u16 = 102;
    pub const NBIOT_SETUP: u16 = 106;
    pub const LORAWAN_SETUP: u16 = 107;
    pub const PROFILE_GENERIC: u16 = 7;
}

#[cfg(test)]
mod tests {
    use super::*;
    use dlms_core::{ObisCode, CosemObject};

    #[test]
    fn test_data_class() {
        let obj = Data::new(ObisCode::new(1, 0, 0, 9, 0, 255), dlms_core::DlmsData::None);
        assert_eq!(obj.class_id(), 1);
        assert_eq!(obj.logical_name(), ObisCode::new(1, 0, 0, 9, 0, 255));
    }

    #[test]
    fn test_register_class() {
        let obj = Register::new(ObisCode::ACTIVE_POWER_L1, dlms_core::DlmsData::DoubleLong(1000));
        assert_eq!(obj.class_id(), 3);
    }

    #[test]
    fn test_clock_class() {
        let obj = Clock::new(ObisCode::CLOCK);
        assert_eq!(obj.class_id(), 8);
        assert_eq!(obj.attribute_count(), 10);
    }

    #[test]
    fn test_profile_generic_class() {
        let obj = ProfileGeneric::new(ObisCode::new(1, 0, 99, 1, 0, 255));
        assert_eq!(obj.class_id(), 7);
    }

    #[test]
    fn test_security_setup_class() {
        let obj = SecuritySetup::new(ObisCode::new(0, 0, 43, 0, 0, 255));
        assert_eq!(obj.class_id(), 70);
    }

    #[test]
    fn test_lp_setup_class() {
        let obj = LpSetup::new();
        assert_eq!(obj.class_id(), 100);
    }

    #[test]
    fn test_rs485_setup_class() {
        let obj = Rs485Setup::new();
        assert_eq!(obj.class_id(), 101);
    }

    #[test]
    fn test_infrared_setup_class() {
        let obj = InfraredSetup::new();
        assert_eq!(obj.class_id(), 102);
    }

    #[test]
    fn test_nbiot_setup_class() {
        let obj = NbiotSetup::new();
        assert_eq!(obj.class_id(), 106);
    }

    #[test]
    fn test_lorawan_setup_class() {
        let obj = LorawanSetup::new();
        assert_eq!(obj.class_id(), 107);
    }

    #[test]
    fn test_extended_register_class() {
        let obj = ExtendedRegister::new(ObisCode::ACTIVE_ENERGY_IMPORT, dlms_core::DlmsData::DoubleLong(0));
        assert_eq!(obj.class_id(), 4);
    }

    #[test]
    fn test_demand_register_class() {
        let obj = DemandRegister::new(ObisCode::ACTIVE_POWER_L1, dlms_core::DlmsData::DoubleLong(0));
        assert_eq!(obj.class_id(), 5);
    }

    #[test]
    fn test_class_id_constants() {
        assert_eq!(class_id::DATA, 1);
        assert_eq!(class_id::REGISTER, 3);
        assert_eq!(class_id::CLOCK, 8);
        assert_eq!(class_id::SECURITY_SETUP, 70);
        assert_eq!(class_id::PROFILE_GENERIC, 7);
    }
}
