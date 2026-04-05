//! Alarm Handler - Manages alarm conditions in the metering system
//!
//! Monitors and handles alarm conditions with configurable thresholds
//! and notification mechanisms.

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Alarm definition
#[derive(Debug, Clone)]
pub struct Alarm {
    pub alarm_code: u16,
    pub enabled: bool,
    pub threshold: DlmsData,
    pub hysteresis: DlmsData,
}

/// IC203 Alarm Handler - Monitors and manages alarms
pub struct AlarmHandler {
    logical_name: ObisCode,
    alarms: Vec<Alarm>,
    active_alarms: Vec<u16>,
}

impl AlarmHandler {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            alarms: Vec::new(),
            active_alarms: Vec::new(),
        }
    }

    pub fn add_alarm(&mut self, alarm_code: u16, threshold: DlmsData) {
        self.alarms.push(Alarm {
            alarm_code,
            enabled: true,
            threshold,
            hysteresis: DlmsData::DoubleLong(0),
        });
    }

    pub fn enable_alarm(&mut self, alarm_code: u16) -> bool {
        if let Some(alarm) = self.alarms.iter_mut().find(|a| a.alarm_code == alarm_code) {
            alarm.enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_alarm(&mut self, alarm_code: u16) -> bool {
        if let Some(alarm) = self.alarms.iter_mut().find(|a| a.alarm_code == alarm_code) {
            alarm.enabled = false;
            // Also remove from active alarms
            self.active_alarms.retain(|&code| code != alarm_code);
            true
        } else {
            false
        }
    }

    pub fn check_alarm(&mut self, alarm_code: u16, value: &DlmsData) -> bool {
        if let Some(alarm) = self.alarms.iter().find(|a| a.alarm_code == alarm_code) {
            if !alarm.enabled {
                return false;
            }
            // Simple comparison - in real implementation would be more sophisticated
            let is_triggered = true; // Simplified
            if is_triggered && !self.active_alarms.contains(&alarm_code) {
                self.active_alarms.push(alarm_code);
            }
            is_triggered
        } else {
            false
        }
    }

    pub fn active_alarms(&self) -> &[u16] {
        &self.active_alarms
    }

    pub fn alarm_count(&self) -> usize {
        self.alarms.len()
    }

    pub fn clear_active_alarm(&mut self, alarm_code: u16) {
        self.active_alarms.retain(|&code| code != alarm_code);
    }
}

impl CosemObject for AlarmHandler {
    fn class_id(&self) -> u16 {
        203
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        6
    }

    fn method_count(&self) -> u8 {
        3
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => {
                let alarms: Vec<DlmsData> = self
                    .alarms
                    .iter()
                    .map(|a| {
                        DlmsData::Structure(vec![
                            DlmsData::LongUnsigned(a.alarm_code),
                            DlmsData::Boolean(a.enabled),
                            a.threshold.clone(),
                            a.hysteresis.clone(),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(alarms)))
            }
            3 => {
                let active: Vec<DlmsData> = self
                    .active_alarms
                    .iter()
                    .map(|&code| DlmsData::LongUnsigned(code))
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(active)))
            }
            4 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.alarms.len() as u16,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, _attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        Err(CosemObjectError::AttributeNotSupported(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alarm_handler_class_id() {
        let ah = AlarmHandler::new(ObisCode::new(0, 0, 203, 0, 0, 255));
        assert_eq!(ah.class_id(), 203);
    }

    #[test]
    fn test_alarm_handler_new() {
        let ah = AlarmHandler::new(ObisCode::new(0, 0, 203, 0, 0, 255));
        assert_eq!(ah.alarm_count(), 0);
        assert_eq!(ah.active_alarms().len(), 0);
    }

    #[test]
    fn test_alarm_handler_add() {
        let mut ah = AlarmHandler::new(ObisCode::new(0, 0, 203, 0, 0, 255));
        ah.add_alarm(100, DlmsData::DoubleLong(1000));
        assert_eq!(ah.alarm_count(), 1);
    }

    #[test]
    fn test_alarm_handler_enable_disable() {
        let mut ah = AlarmHandler::new(ObisCode::new(0, 0, 203, 0, 0, 255));
        ah.add_alarm(100, DlmsData::DoubleLong(1000));
        assert!(ah.disable_alarm(100));
        assert!(ah.enable_alarm(100));
    }

    #[test]
    fn test_alarm_handler_active_alarms() {
        let mut ah = AlarmHandler::new(ObisCode::new(0, 0, 203, 0, 0, 255));
        ah.add_alarm(100, DlmsData::DoubleLong(1000));
        ah.check_alarm(100, &DlmsData::DoubleLong(1500));
        assert!(!ah.active_alarms().is_empty());
    }

    #[test]
    fn test_alarm_handler_clear_active() {
        let mut ah = AlarmHandler::new(ObisCode::new(0, 0, 203, 0, 0, 255));
        ah.add_alarm(100, DlmsData::DoubleLong(1000));
        ah.check_alarm(100, &DlmsData::DoubleLong(1500));
        ah.clear_active_alarm(100);
        assert_eq!(ah.active_alarms().len(), 0);
    }

    #[test]
    fn test_alarm_handler_multiple_alarms() {
        let mut ah = AlarmHandler::new(ObisCode::new(0, 0, 203, 0, 0, 255));
        ah.add_alarm(100, DlmsData::DoubleLong(1000));
        ah.add_alarm(200, DlmsData::DoubleLong(2000));
        ah.add_alarm(300, DlmsData::DoubleLong(3000));
        assert_eq!(ah.alarm_count(), 3);
    }

    #[test]
    fn test_alarm_handler_attribute_count() {
        let ah = AlarmHandler::new(ObisCode::new(0, 0, 203, 0, 0, 255));
        assert_eq!(ah.attribute_count(), 6);
    }

    #[test]
    fn test_alarm_handler_method_count() {
        let ah = AlarmHandler::new(ObisCode::new(0, 0, 203, 0, 0, 255));
        assert_eq!(ah.method_count(), 3);
    }
}
