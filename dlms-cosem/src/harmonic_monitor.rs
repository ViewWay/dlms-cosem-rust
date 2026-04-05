//! IC112 Harmonic Monitor - Harmonic Distortion Monitoring

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Harmonic Monitor - Harmonic Distortion Monitoring Object
/// 
/// This class monitors harmonic distortion in voltage and current waveforms.
/// Used in power quality analysis and compliance monitoring.
#[derive(Debug, Clone)]
pub struct HarmonicMonitor {
    logical_name: ObisCode,
    voltage_harmonics: Vec<u8>, // Up to 63 harmonics
    current_harmonics: Vec<u8>, // Up to 63 harmonics
    thd_voltage: u8,
    thd_current: u8,
    monitoring_mode: u8,
    sample_rate: u16,
}

/// Harmonic monitoring modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MonitoringMode {
    /// Voltage harmonics only
    VoltageOnly = 0,
    /// Current harmonics only
    CurrentOnly = 1,
    /// Both voltage and current
    Both = 2,
}

impl MonitoringMode {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => MonitoringMode::VoltageOnly,
            1 => MonitoringMode::CurrentOnly,
            _ => MonitoringMode::Both,
        }
    }
}

impl HarmonicMonitor {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            voltage_harmonics: vec![0; 63],
            current_harmonics: vec![0; 63],
            thd_voltage: 0,
            thd_current: 0,
            monitoring_mode: MonitoringMode::Both as u8,
            sample_rate: 256,
        }
    }

    pub fn voltage_harmonic(&self, order: usize) -> Option<u8> {
        if order >= 1 && order <= 63 {
            self.voltage_harmonics.get(order - 1).copied()
        } else {
            None
        }
    }

    pub fn set_voltage_harmonic(&mut self, order: usize, value: u8) -> bool {
        if order >= 1 && order <= 63 {
            self.voltage_harmonics[order - 1] = value;
            true
        } else {
            false
        }
    }

    pub fn current_harmonic(&self, order: usize) -> Option<u8> {
        if order >= 1 && order <= 63 {
            self.current_harmonics.get(order - 1).copied()
        } else {
            None
        }
    }

    pub fn set_current_harmonic(&mut self, order: usize, value: u8) -> bool {
        if order >= 1 && order <= 63 {
            self.current_harmonics[order - 1] = value;
            true
        } else {
            false
        }
    }

    pub fn thd_voltage(&self) -> u8 {
        self.thd_voltage
    }

    pub fn set_thd_voltage(&mut self, thd: u8) {
        self.thd_voltage = thd;
    }

    pub fn thd_current(&self) -> u8 {
        self.thd_current
    }

    pub fn set_thd_current(&mut self, thd: u8) {
        self.thd_current = thd;
    }

    pub fn monitoring_mode(&self) -> MonitoringMode {
        MonitoringMode::from_u8(self.monitoring_mode)
    }

    pub fn set_monitoring_mode(&mut self, mode: MonitoringMode) {
        self.monitoring_mode = mode as u8;
    }

    pub fn sample_rate(&self) -> u16 {
        self.sample_rate
    }

    pub fn set_sample_rate(&mut self, rate: u16) {
        self.sample_rate = rate;
    }

    pub fn calculate_thd_voltage(&mut self) -> u8 {
        if self.voltage_harmonics.is_empty() {
            return 0;
        }
        let sum_squares: u32 = self.voltage_harmonics.iter()
            .skip(1) // Skip fundamental
            .map(|&h| (h as u32).pow(2))
            .sum();
        let fundamental = self.voltage_harmonics.first().copied().unwrap_or(1) as f32;
        if fundamental > 0.0 {
            let thd = (sum_squares as f32).sqrt() / fundamental * 100.0;
            self.thd_voltage = thd.min(255.0) as u8;
        }
        self.thd_voltage
    }

    pub fn calculate_thd_current(&mut self) -> u8 {
        if self.current_harmonics.is_empty() {
            return 0;
        }
        let sum_squares: u32 = self.current_harmonics.iter()
            .skip(1)
            .map(|&h| (h as u32).pow(2))
            .sum();
        let fundamental = self.current_harmonics.first().copied().unwrap_or(1) as f32;
        if fundamental > 0.0 {
            let thd = (sum_squares as f32).sqrt() / fundamental * 100.0;
            self.thd_current = thd.min(255.0) as u8;
        }
        self.thd_current
    }
}

impl CosemObject for HarmonicMonitor {
    fn class_id(&self) -> u16 {
        112
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        7
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
            2 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.voltage_harmonics.iter()
                    .map(|&h| DlmsData::Unsigned(h))
                    .collect(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.current_harmonics.iter()
                    .map(|&h| DlmsData::Unsigned(h))
                    .collect(),
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.thd_voltage))),
            5 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.thd_current))),
            6 => Some(dlms_axdr::encode(&DlmsData::Enum(self.monitoring_mode))),
            7 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.sample_rate))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(thd) = decoded {
                    self.thd_voltage = thd;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            5 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(thd) = decoded {
                    self.thd_current = thd;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            6 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Enum(mode) = decoded {
                    self.monitoring_mode = mode;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            7 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::LongUnsigned(rate) = decoded {
                    self.sample_rate = rate;
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
                // Reset harmonics data
                self.voltage_harmonics.fill(0);
                self.current_harmonics.fill(0);
                self.thd_voltage = 0;
                self.thd_current = 0;
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
    fn test_harmonic_monitor_new() {
        let hm = HarmonicMonitor::new(ObisCode::new(0, 0, 112, 0, 0, 255));
        assert_eq!(hm.class_id(), 112);
    }

    #[test]
    fn test_harmonic_monitor_voltage_harmonic() {
        let mut hm = HarmonicMonitor::new(ObisCode::new(0, 0, 112, 0, 0, 255));
        assert!(hm.set_voltage_harmonic(1, 100));
        assert!(hm.set_voltage_harmonic(3, 5));
        assert_eq!(hm.voltage_harmonic(1), Some(100));
        assert_eq!(hm.voltage_harmonic(3), Some(5));
        assert_eq!(hm.voltage_harmonic(64), None);
    }

    #[test]
    fn test_harmonic_monitor_current_harmonic() {
        let mut hm = HarmonicMonitor::new(ObisCode::new(0, 0, 112, 0, 0, 255));
        assert!(hm.set_current_harmonic(1, 80));
        assert!(hm.set_current_harmonic(5, 10));
        assert_eq!(hm.current_harmonic(1), Some(80));
        assert_eq!(hm.current_harmonic(5), Some(10));
    }

    #[test]
    fn test_harmonic_monitor_thd() {
        let mut hm = HarmonicMonitor::new(ObisCode::new(0, 0, 112, 0, 0, 255));
        hm.set_thd_voltage(8);
        hm.set_thd_current(12);
        assert_eq!(hm.thd_voltage(), 8);
        assert_eq!(hm.thd_current(), 12);
    }

    #[test]
    fn test_harmonic_monitor_monitoring_mode() {
        let mut hm = HarmonicMonitor::new(ObisCode::new(0, 0, 112, 0, 0, 255));
        assert_eq!(hm.monitoring_mode(), MonitoringMode::Both);
        hm.set_monitoring_mode(MonitoringMode::VoltageOnly);
        assert_eq!(hm.monitoring_mode(), MonitoringMode::VoltageOnly);
    }

    #[test]
    fn test_harmonic_monitor_sample_rate() {
        let mut hm = HarmonicMonitor::new(ObisCode::new(0, 0, 112, 0, 0, 255));
        hm.set_sample_rate(512);
        assert_eq!(hm.sample_rate(), 512);
    }

    #[test]
    fn test_harmonic_monitor_calculate_thd() {
        let mut hm = HarmonicMonitor::new(ObisCode::new(0, 0, 112, 0, 0, 255));
        hm.set_voltage_harmonic(1, 100);
        hm.set_voltage_harmonic(3, 5);
        hm.set_voltage_harmonic(5, 3);
        let thd = hm.calculate_thd_voltage();
        assert!(thd > 0);
    }

    #[test]
    fn test_harmonic_monitor_reset() {
        let mut hm = HarmonicMonitor::new(ObisCode::new(0, 0, 112, 0, 0, 255));
        hm.set_voltage_harmonic(1, 100);
        hm.set_thd_voltage(10);
        hm.execute_action(1, &[]).unwrap();
        assert_eq!(hm.voltage_harmonic(1), Some(0));
        assert_eq!(hm.thd_voltage(), 0);
    }

    #[test]
    fn test_harmonic_monitor_attribute_count() {
        let hm = HarmonicMonitor::new(ObisCode::new(0, 0, 112, 0, 0, 255));
        assert_eq!(hm.attribute_count(), 7);
    }

    #[test]
    fn test_harmonic_monitor_method_count() {
        let hm = HarmonicMonitor::new(ObisCode::new(0, 0, 112, 0, 0, 255));
        assert_eq!(hm.method_count(), 1);
    }

    #[test]
    fn test_monitoring_mode_from_u8() {
        assert_eq!(MonitoringMode::from_u8(0), MonitoringMode::VoltageOnly);
        assert_eq!(MonitoringMode::from_u8(1), MonitoringMode::CurrentOnly);
        assert_eq!(MonitoringMode::from_u8(2), MonitoringMode::Both);
    }
}
