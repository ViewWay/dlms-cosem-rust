//! dlms-core: Core types for DLMS/COSEM protocol
//!
//! Provides fundamental types used across all DLMS/COSEM crates:
//! - `ObisCode`: 6-byte OBIS identification code
//! - `CosemDateTime`: DLMS date/time representation
//! - `DlmsData`: All DLMS data type enumerations
//! - `CosemObject` trait: Base trait for all COSEM interface classes
//! - `CosemAttribute` / `CosemMethod`: Attribute and method descriptors
//! - `AccessResult`: Access result enumerations

#![cfg_attr(not(feature = "std"), no_std)]

// no_std support: feature gate for no_std

// Re-exports
mod access_result;
mod attribute;
mod cosem_object;
mod data;
mod datetime;
mod obis;

pub use access_result::AccessResult;
pub use attribute::{CosemAttribute, CosemMethod};
pub use cosem_object::{CosemObject, CosemObjectError};
pub use data::DlmsData;
pub use datetime::CosemDateTime;
pub use obis::ObisCode;
