# dlms-cosem

COSEM Interface Classes for DLMS/COSEM.

## Overview

Implements 130+ COSEM interface class objects including:
- **IC001** Data, **IC003** Register, **IC004** ExtendedRegister, **IC005** DemandRegister
- **IC007** Profile Generic
- **IC008** Clock, **IC010** Demand, **IC012** Activity Calendar
- **IC017** Billing, **IC019** Calendar, **IC020** Total
- **IC031** Single Phase, **IC034** Maximum Demand
- **IC070** Disconnect Control, **IC072** Display
- Association (LN/SN), Script Table, Status Mapping, Comm Control
- And many more...

All classes implement the `CosemObject` trait from `dlms-core`.

## Usage

```rust
use dlms_cosem::{Clock, Register};
use dlms_core::{ObisCode, DlmsData, CosemObject};

let clock = Clock::new(ObisCode::CLOCK);
println!("Class ID: {}", clock.class_id()); // 8

let register = Register::new(
    ObisCode::ACTIVE_POWER_L1,
    DlmsData::DoubleLong(1234),
);
```

## Dependencies

- `dlms-core`
- `dlms-axdr`
