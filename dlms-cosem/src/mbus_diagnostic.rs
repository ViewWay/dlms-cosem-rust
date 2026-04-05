//! IC110 M-Bus Diagnostic - M-Bus Communication Diagnostics

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// M-Bus Diagnostic - M-Bus Communication Diagnostics Object
/// 
/// This class provides diagnostic information for M-Bus communication.
/// Used in wired and wireless M-Bus metering systems.
#[derive(Debug, Clone)]
pub struct MbusDiagnostic {
    logical_name: ObisCode,
    total_messages_sent: u32,
    total_messages_received: u32,
    failed_messages: u32,
    crc_errors: u32,
    timeout_errors: u32,
    last_error_code: u8,
    signal_quality: u8,
    bus_voltage: f32,
}

impl MbusDiagnostic {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            total_messages_sent: 0,
            total_messages_received: 0,
            failed_messages: 0,
            crc_errors: 0,
            timeout_errors: 0,
            last_error_code: 0,
            signal_quality: 0,
            bus_voltage: 0.0,
        }
    }

    pub fn total_messages_sent(&self) -> u32 {
        self.total_messages_sent
    }

    pub fn increment_sent(&mut self) {
        self.total_messages_sent = self.total_messages_sent.saturating_add(1);
    }

    pub fn total_messages_received(&self) -> u32 {
        self.total_messages_received
    }

    pub fn increment_received(&mut self) {
        self.total_messages_received = self.total_messages_received.saturating_add(1);
    }

    pub fn failed_messages(&self) -> u32 {
        self.failed_messages
    }

    pub fn increment_failed(&mut self) {
        self.failed_messages = self.failed_messages.saturating_add(1);
    }

    pub fn crc_errors(&self) -> u32 {
        self.crc_errors
    }

    pub fn increment_crc_error(&mut self) {
        self.crc_errors = self.crc_errors.saturating_add(1);
    }

    pub fn timeout_errors(&self) -> u32 {
        self.timeout_errors
    }

    pub fn increment_timeout_error(&mut self) {
        self.timeout_errors = self.timeout_errors.saturating_add(1);
    }

    pub fn last_error_code(&self) -> u8 {
        self.last_error_code
    }

    pub fn set_last_error_code(&mut self, code: u8) {
        self.last_error_code = code;
    }

    pub fn signal_quality(&self) -> u8 {
        self.signal_quality
    }

    pub fn set_signal_quality(&mut self, quality: u8) {
        self.signal_quality = quality;
    }

    pub fn bus_voltage(&self) -> f32 {
        self.bus_voltage
    }

    pub fn set_bus_voltage(&mut self, voltage: f32) {
        self.bus_voltage = voltage;
    }

    pub fn success_rate(&self) -> f32 {
        let total = self.total_messages_sent + self.total_messages_received;
        if total == 0 {
            return 100.0;
        }
        let failed = self.failed_messages as f32;
        (1.0 - failed / total as f32) * 100.0
    }

    pub fn reset_counters(&mut self) {
        self.total_messages_sent = 0;
        self.total_messages_received = 0;
        self.failed_messages = 0;
        self.crc_errors = 0;
        self.timeout_errors = 0;
        self.last_error_code = 0;
    }
}

impl CosemObject for MbusDiagnostic {
    fn class_id(&self) -> u16 {
        110
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        9
    }

    fn method_count(&self) -> u8 {
        1
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.total_messages_sent,
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.total_messages_received,
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.failed_messages,
            ))),
            5 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.crc_errors,
            ))),
            6 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.timeout_errors,
            ))),
            7 => Some(dlms_axdr::encode(&DlmsData::Enum(self.last_error_code))),
            8 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.signal_quality))),
            9 => Some(dlms_axdr::encode(&DlmsData::Float32(self.bus_voltage))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            7 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(code) = decoded {
                    self.last_error_code = code;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            8 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(quality) = decoded {
                    self.signal_quality = quality;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            9 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Float32(voltage) = decoded {
                    self.bus_voltage = voltage;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }

    fn execute_action(&mut self, method_id: u8, _data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        match method_id {
            1 => {
                // Reset counters
                self.reset_counters();
                Ok(vec![0x00, 0x00])
            }
            _ => Err(CosemObjectError::MethodNotSupported(method_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mbus_diagnostic_new() {
        let diag = MbusDiagnostic::new(ObisCode::new(0, 0, 110, 0, 0, 255));
        assert_eq!(diag.class_id(), 110);
    }

    #[test]
    fn test_mbus_diagnostic_counters() {
        let mut diag = MbusDiagnostic::new(ObisCode::new(0, 0, 110, 0, 0, 255));
        diag.increment_sent();
        diag.increment_sent();
        diag.increment_received();
        diag.increment_failed();
        assert_eq!(diag.total_messages_sent(), 2);
        assert_eq!(diag.total_messages_received(), 1);
        assert_eq!(diag.failed_messages(), 1);
    }

    #[test]
    fn test_mbus_diagnostic_error_counters() {
        let mut diag = MbusDiagnostic::new(ObisCode::new(0, 0, 110, 0, 0, 255));
        diag.increment_crc_error();
        diag.increment_timeout_error();
        assert_eq!(diag.crc_errors(), 1);
        assert_eq!(diag.timeout_errors(), 1);
    }

    #[test]
    fn test_mbus_diagnostic_success_rate() {
        let mut diag = MbusDiagnostic::new(ObisCode::new(0, 0, 110, 0, 0, 255));
        assert_eq!(diag.success_rate(), 100.0);
        
        diag.increment_sent();
        diag.increment_sent();
        diag.increment_failed();
        assert!((diag.success_rate() - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_mbus_diagnostic_signal_quality() {
        let mut diag = MbusDiagnostic::new(ObisCode::new(0, 0, 110, 0, 0, 255));
        diag.set_signal_quality(85);
        assert_eq!(diag.signal_quality(), 85);
    }

    #[test]
    fn test_mbus_diagnostic_bus_voltage() {
        let mut diag = MbusDiagnostic::new(ObisCode::new(0, 0, 110, 0, 0, 255));
        diag.set_bus_voltage(24.5);
        assert!((diag.bus_voltage() - 24.5).abs() < 0.01);
    }

    #[test]
    fn test_mbus_diagnostic_reset() {
        let mut diag = MbusDiagnostic::new(ObisCode::new(0, 0, 110, 0, 0, 255));
        diag.increment_sent();
        diag.increment_failed();
        diag.reset_counters();
        assert_eq!(diag.total_messages_sent(), 0);
        assert_eq!(diag.failed_messages(), 0);
    }

    #[test]
    fn test_mbus_diagnostic_attribute_count() {
        let diag = MbusDiagnostic::new(ObisCode::new(0, 0, 110, 0, 0, 255));
        assert_eq!(diag.attribute_count(), 9);
    }

    #[test]
    fn test_mbus_diagnostic_method_count() {
        let diag = MbusDiagnostic::new(ObisCode::new(0, 0, 110, 0, 0, 255));
        assert_eq!(diag.method_count(), 1);
    }

    #[test]
    fn test_mbus_diagnostic_reset_method() {
        let mut diag = MbusDiagnostic::new(ObisCode::new(0, 0, 110, 0, 0, 255));
        diag.increment_sent();
        let result = diag.execute_action(1, &[]);
        assert!(result.is_ok());
        assert_eq!(diag.total_messages_sent(), 0);
    }
}
