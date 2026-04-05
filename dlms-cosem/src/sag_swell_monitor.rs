//! IC113 Sag Swell Monitor - Voltage Sag and Swell Monitoring

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Sag Swell Monitor - Voltage Sag and Swell Monitoring Object
/// 
/// This class monitors voltage sags (dips) and swells (surges).
/// Used in power quality compliance monitoring (EN 50160, IEEE 1159).
#[derive(Debug, Clone)]
pub struct SagSwellMonitor {
    logical_name: ObisCode,
    sag_threshold_percent: u8,
    swell_threshold_percent: u8,
    sag_count: u32,
    swell_count: u32,
    sag_duration_ms: u32,
    swell_duration_ms: u32,
    last_sag_depth_percent: u8,
    last_swell_magnitude_percent: u8,
    monitoring_enabled: bool,
}

impl SagSwellMonitor {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            sag_threshold_percent: 90,    // 90% of nominal
            swell_threshold_percent: 110, // 110% of nominal
            sag_count: 0,
            swell_count: 0,
            sag_duration_ms: 0,
            swell_duration_ms: 0,
            last_sag_depth_percent: 100,
            last_swell_magnitude_percent: 100,
            monitoring_enabled: true,
        }
    }

    pub fn sag_threshold_percent(&self) -> u8 {
        self.sag_threshold_percent
    }

    pub fn set_sag_threshold_percent(&mut self, threshold: u8) {
        self.sag_threshold_percent = threshold;
    }

    pub fn swell_threshold_percent(&self) -> u8 {
        self.swell_threshold_percent
    }

    pub fn set_swell_threshold_percent(&mut self, threshold: u8) {
        self.swell_threshold_percent = threshold;
    }

    pub fn sag_count(&self) -> u32 {
        self.sag_count
    }

    pub fn record_sag(&mut self, depth_percent: u8, duration_ms: u32) {
        self.sag_count = self.sag_count.saturating_add(1);
        self.last_sag_depth_percent = depth_percent;
        self.sag_duration_ms = duration_ms;
    }

    pub fn swell_count(&self) -> u32 {
        self.swell_count
    }

    pub fn record_swell(&mut self, magnitude_percent: u8, duration_ms: u32) {
        self.swell_count = self.swell_count.saturating_add(1);
        self.last_swell_magnitude_percent = magnitude_percent;
        self.swell_duration_ms = duration_ms;
    }

    pub fn last_sag_depth_percent(&self) -> u8 {
        self.last_sag_depth_percent
    }

    pub fn last_swell_magnitude_percent(&self) -> u8 {
        self.last_swell_magnitude_percent
    }

    pub fn sag_duration_ms(&self) -> u32 {
        self.sag_duration_ms
    }

    pub fn swell_duration_ms(&self) -> u32 {
        self.swell_duration_ms
    }

    pub fn monitoring_enabled(&self) -> bool {
        self.monitoring_enabled
    }

    pub fn set_monitoring_enabled(&mut self, enabled: bool) {
        self.monitoring_enabled = enabled;
    }

    pub fn check_voltage(&mut self, voltage_percent: u8) -> Option<SagSwellEvent> {
        if !self.monitoring_enabled {
            return None;
        }

        if voltage_percent < self.sag_threshold_percent {
            Some(SagSwellEvent::Sag {
                depth: voltage_percent,
            })
        } else if voltage_percent > self.swell_threshold_percent {
            Some(SagSwellEvent::Swell {
                magnitude: voltage_percent,
            })
        } else {
            None
        }
    }

    pub fn reset_counters(&mut self) {
        self.sag_count = 0;
        self.swell_count = 0;
        self.sag_duration_ms = 0;
        self.swell_duration_ms = 0;
    }
}

/// Sag or swell event
#[derive(Debug, Clone)]
pub enum SagSwellEvent {
    /// Voltage sag event with depth as percentage of nominal
    Sag { depth: u8 },
    /// Voltage swell event with magnitude as percentage of nominal
    Swell { magnitude: u8 },
}

impl CosemObject for SagSwellMonitor {
    fn class_id(&self) -> u16 {
        113
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        10
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
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.sag_threshold_percent))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.swell_threshold_percent))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.sag_count))),
            5 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.swell_count))),
            6 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.sag_duration_ms))),
            7 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.swell_duration_ms))),
            8 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.last_sag_depth_percent))),
            9 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.last_swell_magnitude_percent))),
            10 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.monitoring_enabled))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(threshold) = decoded {
                    self.sag_threshold_percent = threshold;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(threshold) = decoded {
                    self.swell_threshold_percent = threshold;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            10 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Boolean(enabled) = decoded {
                    self.monitoring_enabled = enabled;
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
    fn test_sag_swell_monitor_new() {
        let ssm = SagSwellMonitor::new(ObisCode::new(0, 0, 113, 0, 0, 255));
        assert_eq!(ssm.class_id(), 113);
    }

    #[test]
    fn test_sag_swell_monitor_thresholds() {
        let mut ssm = SagSwellMonitor::new(ObisCode::new(0, 0, 113, 0, 0, 255));
        assert_eq!(ssm.sag_threshold_percent(), 90);
        assert_eq!(ssm.swell_threshold_percent(), 110);
        
        ssm.set_sag_threshold_percent(85);
        ssm.set_swell_threshold_percent(115);
        assert_eq!(ssm.sag_threshold_percent(), 85);
        assert_eq!(ssm.swell_threshold_percent(), 115);
    }

    #[test]
    fn test_sag_swell_monitor_record_sag() {
        let mut ssm = SagSwellMonitor::new(ObisCode::new(0, 0, 113, 0, 0, 255));
        ssm.record_sag(75, 150); // 75% depth, 150ms duration
        assert_eq!(ssm.sag_count(), 1);
        assert_eq!(ssm.last_sag_depth_percent(), 75);
        assert_eq!(ssm.sag_duration_ms(), 150);
    }

    #[test]
    fn test_sag_swell_monitor_record_swell() {
        let mut ssm = SagSwellMonitor::new(ObisCode::new(0, 0, 113, 0, 0, 255));
        ssm.record_swell(125, 80); // 125% magnitude, 80ms duration
        assert_eq!(ssm.swell_count(), 1);
        assert_eq!(ssm.last_swell_magnitude_percent(), 125);
        assert_eq!(ssm.swell_duration_ms(), 80);
    }

    #[test]
    fn test_sag_swell_monitor_check_voltage_sag() {
        let mut ssm = SagSwellMonitor::new(ObisCode::new(0, 0, 113, 0, 0, 255));
        let event = ssm.check_voltage(80);
        assert!(matches!(event, Some(SagSwellEvent::Sag { depth: 80 })));
    }

    #[test]
    fn test_sag_swell_monitor_check_voltage_swell() {
        let mut ssm = SagSwellMonitor::new(ObisCode::new(0, 0, 113, 0, 0, 255));
        let event = ssm.check_voltage(120);
        assert!(matches!(event, Some(SagSwellEvent::Swell { magnitude: 120 })));
    }

    #[test]
    fn test_sag_swell_monitor_check_voltage_normal() {
        let mut ssm = SagSwellMonitor::new(ObisCode::new(0, 0, 113, 0, 0, 255));
        let event = ssm.check_voltage(100);
        assert!(event.is_none());
    }

    #[test]
    fn test_sag_swell_monitor_monitoring_enabled() {
        let mut ssm = SagSwellMonitor::new(ObisCode::new(0, 0, 113, 0, 0, 255));
        assert!(ssm.monitoring_enabled());
        
        ssm.set_monitoring_enabled(false);
        assert!(!ssm.monitoring_enabled());
        
        // Should not detect events when disabled
        let event = ssm.check_voltage(80);
        assert!(event.is_none());
    }

    #[test]
    fn test_sag_swell_monitor_reset() {
        let mut ssm = SagSwellMonitor::new(ObisCode::new(0, 0, 113, 0, 0, 255));
        ssm.record_sag(75, 100);
        ssm.record_swell(120, 50);
        ssm.reset_counters();
        assert_eq!(ssm.sag_count(), 0);
        assert_eq!(ssm.swell_count(), 0);
    }

    #[test]
    fn test_sag_swell_monitor_attribute_count() {
        let ssm = SagSwellMonitor::new(ObisCode::new(0, 0, 113, 0, 0, 255));
        assert_eq!(ssm.attribute_count(), 10);
    }

    #[test]
    fn test_sag_swell_monitor_method_count() {
        let ssm = SagSwellMonitor::new(ObisCode::new(0, 0, 113, 0, 0, 255));
        assert_eq!(ssm.method_count(), 1);
    }
}
