//! IC111 Power Quality Monitor - Power Quality Monitoring

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Power Quality Monitor - Power Quality Monitoring Object
/// 
/// This class monitors power quality parameters including voltage,
/// current, and frequency deviations.
#[derive(Debug, Clone)]
pub struct PowerQualityMonitor {
    logical_name: ObisCode,
    voltage_sag_count: u32,
    voltage_swell_count: u32,
    interruption_count: u32,
    voltage_unbalance: u8,
    frequency_deviation: i16,
    thd_voltage: u8,
    thd_current: u8,
    power_factor_avg: u8,
}

impl PowerQualityMonitor {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            voltage_sag_count: 0,
            voltage_swell_count: 0,
            interruption_count: 0,
            voltage_unbalance: 0,
            frequency_deviation: 0,
            thd_voltage: 0,
            thd_current: 0,
            power_factor_avg: 100, // 1.00 as percentage
        }
    }

    pub fn voltage_sag_count(&self) -> u32 {
        self.voltage_sag_count
    }

    pub fn increment_sag(&mut self) {
        self.voltage_sag_count = self.voltage_sag_count.saturating_add(1);
    }

    pub fn voltage_swell_count(&self) -> u32 {
        self.voltage_swell_count
    }

    pub fn increment_swell(&mut self) {
        self.voltage_swell_count = self.voltage_swell_count.saturating_add(1);
    }

    pub fn interruption_count(&self) -> u32 {
        self.interruption_count
    }

    pub fn increment_interruption(&mut self) {
        self.interruption_count = self.interruption_count.saturating_add(1);
    }

    pub fn voltage_unbalance(&self) -> u8 {
        self.voltage_unbalance
    }

    pub fn set_voltage_unbalance(&mut self, unbalance: u8) {
        self.voltage_unbalance = unbalance;
    }

    pub fn frequency_deviation(&self) -> i16 {
        self.frequency_deviation
    }

    pub fn set_frequency_deviation(&mut self, deviation: i16) {
        self.frequency_deviation = deviation;
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

    pub fn power_factor_avg(&self) -> u8 {
        self.power_factor_avg
    }

    pub fn set_power_factor_avg(&mut self, pf: u8) {
        self.power_factor_avg = pf;
    }

    pub fn power_factor_decimal(&self) -> f32 {
        self.power_factor_avg as f32 / 100.0
    }

    pub fn reset_counters(&mut self) {
        self.voltage_sag_count = 0;
        self.voltage_swell_count = 0;
        self.interruption_count = 0;
    }

    pub fn total_events(&self) -> u32 {
        self.voltage_sag_count
            .saturating_add(self.voltage_swell_count)
            .saturating_add(self.interruption_count)
    }
}

impl CosemObject for PowerQualityMonitor {
    fn class_id(&self) -> u16 {
        111
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
                self.voltage_sag_count,
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.voltage_swell_count,
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.interruption_count,
            ))),
            5 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.voltage_unbalance))),
            6 => Some(dlms_axdr::encode(&DlmsData::Integer(self.frequency_deviation))),
            7 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.thd_voltage))),
            8 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.thd_current))),
            9 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.power_factor_avg))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            5 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(unbalance) = decoded {
                    self.voltage_unbalance = unbalance;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            6 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Integer(deviation) = decoded {
                    self.frequency_deviation = deviation;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            7 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(thd) = decoded {
                    self.thd_voltage = thd;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            8 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(thd) = decoded {
                    self.thd_current = thd;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            9 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Unsigned(pf) = decoded {
                    self.power_factor_avg = pf;
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
    fn test_power_quality_monitor_new() {
        let pqm = PowerQualityMonitor::new(ObisCode::new(0, 0, 111, 0, 0, 255));
        assert_eq!(pqm.class_id(), 111);
    }

    #[test]
    fn test_power_quality_monitor_sag_count() {
        let mut pqm = PowerQualityMonitor::new(ObisCode::new(0, 0, 111, 0, 0, 255));
        pqm.increment_sag();
        pqm.increment_sag();
        assert_eq!(pqm.voltage_sag_count(), 2);
    }

    #[test]
    fn test_power_quality_monitor_swell_count() {
        let mut pqm = PowerQualityMonitor::new(ObisCode::new(0, 0, 111, 0, 0, 255));
        pqm.increment_swell();
        assert_eq!(pqm.voltage_swell_count(), 1);
    }

    #[test]
    fn test_power_quality_monitor_interruption_count() {
        let mut pqm = PowerQualityMonitor::new(ObisCode::new(0, 0, 111, 0, 0, 255));
        pqm.increment_interruption();
        assert_eq!(pqm.interruption_count(), 1);
    }

    #[test]
    fn test_power_quality_monitor_thd() {
        let mut pqm = PowerQualityMonitor::new(ObisCode::new(0, 0, 111, 0, 0, 255));
        pqm.set_thd_voltage(5);
        pqm.set_thd_current(8);
        assert_eq!(pqm.thd_voltage(), 5);
        assert_eq!(pqm.thd_current(), 8);
    }

    #[test]
    fn test_power_quality_monitor_power_factor() {
        let mut pqm = PowerQualityMonitor::new(ObisCode::new(0, 0, 111, 0, 0, 255));
        pqm.set_power_factor_avg(95);
        assert_eq!(pqm.power_factor_avg(), 95);
        assert!((pqm.power_factor_decimal() - 0.95).abs() < 0.01);
    }

    #[test]
    fn test_power_quality_monitor_frequency_deviation() {
        let mut pqm = PowerQualityMonitor::new(ObisCode::new(0, 0, 111, 0, 0, 255));
        pqm.set_frequency_deviation(-50); // -0.5 Hz
        assert_eq!(pqm.frequency_deviation(), -50);
    }

    #[test]
    fn test_power_quality_monitor_total_events() {
        let mut pqm = PowerQualityMonitor::new(ObisCode::new(0, 0, 111, 0, 0, 255));
        pqm.increment_sag();
        pqm.increment_swell();
        pqm.increment_interruption();
        assert_eq!(pqm.total_events(), 3);
    }

    #[test]
    fn test_power_quality_monitor_reset() {
        let mut pqm = PowerQualityMonitor::new(ObisCode::new(0, 0, 111, 0, 0, 255));
        pqm.increment_sag();
        pqm.reset_counters();
        assert_eq!(pqm.voltage_sag_count(), 0);
    }

    #[test]
    fn test_power_quality_monitor_attribute_count() {
        let pqm = PowerQualityMonitor::new(ObisCode::new(0, 0, 111, 0, 0, 255));
        assert_eq!(pqm.attribute_count(), 9);
    }

    #[test]
    fn test_power_quality_monitor_method_count() {
        let pqm = PowerQualityMonitor::new(ObisCode::new(0, 0, 111, 0, 0, 255));
        assert_eq!(pqm.method_count(), 1);
    }
}
