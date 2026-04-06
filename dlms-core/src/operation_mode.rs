//! Operation modes for DLMS/COSEM (Green Book 6.2-6.3)
//!
//! DLMS/COSEM defines two operation modes:
//! - Readout mode: Read-only access with HLS password authentication
//! - Programming mode: Read-write access with HLS-SLS signature authentication

use core::fmt;

/// Operation mode for DLMS/COSEM connections
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationMode {
    /// Readout mode: Read-only access with HLS password authentication
    /// In this mode, only read operations are allowed.
    Readout,

    /// Programming mode: Read-write access with HLS-SLS signature authentication
    /// In this mode, both read and write operations are allowed.
    Programming,
}

impl OperationMode {
    /// Check if write operations are allowed in this mode
    pub fn allows_write(&self) -> bool {
        matches!(self, OperationMode::Programming)
    }

    /// Check if read operations are allowed in this mode
    pub fn allows_read(&self) -> bool {
        true // Both modes allow read operations
    }

    /// Switch to the specified operation mode
    pub fn switch_to(&mut self, new_mode: OperationMode) {
        *self = new_mode;
    }
}

impl Default for OperationMode {
    fn default() -> Self {
        OperationMode::Readout
    }
}

impl fmt::Display for OperationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationMode::Readout => write!(f, "Readout"),
            OperationMode::Programming => write!(f, "Programming"),
        }
    }
}

/// Error type for operation mode violations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationModeError {
    pub message: String,
}

impl OperationModeError {
    pub fn new(msg: impl Into<String>) -> Self {
        OperationModeError {
            message: msg.into(),
        }
    }
}

impl fmt::Display for OperationModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operation mode error: {}", self.message)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for OperationModeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readout_mode_permissions() {
        let mode = OperationMode::Readout;
        assert!(mode.allows_read());
        assert!(!mode.allows_write());
    }

    #[test]
    fn test_programming_mode_permissions() {
        let mode = OperationMode::Programming;
        assert!(mode.allows_read());
        assert!(mode.allows_write());
    }

    #[test]
    fn test_switch_mode() {
        let mut mode = OperationMode::Readout;
        assert_eq!(mode, OperationMode::Readout);

        mode.switch_to(OperationMode::Programming);
        assert_eq!(mode, OperationMode::Programming);
    }

    #[test]
    fn test_default_mode() {
        let mode = OperationMode::default();
        assert_eq!(mode, OperationMode::Readout);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", OperationMode::Readout), "Readout");
        assert_eq!(format!("{}", OperationMode::Programming), "Programming");
    }
}
