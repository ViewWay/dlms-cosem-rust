//! Example: Using COSEM Interface Classes
//!
//! Demonstrates creating and working with various DLMS/COSEM IC objects.

use dlms_core::{CosemObject, ObisCode};
use dlms_cosem::{Clock, Data, Register, ProfileGeneric, DisconnectControl, SupplyDisabling, LocalDisplay};
use dlms_core::DlmsData;

fn main() {
    // Clock (IC008)
    let clock = Clock::new(ObisCode::CLOCK);
    println!("Clock class_id: {}", clock.class_id());
    println!("Clock timezone: {} min", clock.timezone());

    // Register (IC003)
    let register = Register::new(ObisCode::ACTIVE_POWER_L1, DlmsData::DoubleLong(12345));
    println!("Register class_id: {}", register.class_id());

    // Data (IC001)
    let data = Data::new(ObisCode::new(1, 0, 0, 9, 0, 255), DlmsData::Unsigned(42));
    println!("Data class_id: {}", data.class_id());

    // Profile Generic (IC007)
    let profile = ProfileGeneric::new(ObisCode::new(1, 0, 99, 1, 0, 255));
    println!("Profile Generic class_id: {}", profile.class_id());

    // Disconnect Control (IC070)
    let dc = DisconnectControl::new(ObisCode::new(0, 0, 96, 1, 0, 255));
    println!("Disconnect Control class_id: {}", dc.class_id());

    // Supply Disabling (IC006)
    let sd = SupplyDisabling::new(ObisCode::new(0, 0, 96, 5, 0, 255));
    println!("Supply Disabling class_id: {}", sd.class_id());

    // Local Display (IC009)
    let ld = LocalDisplay::new(ObisCode::new(0, 0, 96, 9, 0, 255));
    println!("Local Display class_id: {}", ld.class_id());

    println!("\nAll IC classes working correctly!");
}
