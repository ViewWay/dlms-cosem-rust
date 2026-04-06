//! IC071 Limiter
//! Blue Book Ed16: class_id=71, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Threshold evaluation result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThresholdAction {
    Normal,
    OverThreshold,
    UnderThreshold,
}

/// Limiter - limits supply based on monitored values
pub struct Limiter {
    logical_name: ObisCode,
    monitored_value: DlmsData,
    threshold_normal: DlmsData,
    threshold_over: DlmsData,
    threshold_under: DlmsData,
    min_over_duration: u32,
    min_under_duration: u32,
    emergency_profile_active: bool,
    /// Timestamp (seconds) when over-threshold condition started
    over_threshold_start: Option<u32>,
    /// Timestamp (seconds) when under-threshold condition started
    under_threshold_start: Option<u32>,
}

impl Limiter {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            monitored_value: DlmsData::DoubleLong(0),
            threshold_normal: DlmsData::DoubleLong(100),
            threshold_over: DlmsData::DoubleLong(120),
            threshold_under: DlmsData::DoubleLong(0),
            min_over_duration: 60,
            min_under_duration: 60,
            emergency_profile_active: false,
            over_threshold_start: None,
            under_threshold_start: None,
        }
    }

    pub fn monitored_value(&self) -> &DlmsData {
        &self.monitored_value
    }
    pub fn set_monitored_value(&mut self, v: DlmsData) {
        self.monitored_value = v;
    }
    pub fn emergency_active(&self) -> bool {
        self.emergency_profile_active
    }
    pub fn set_emergency_active(&mut self, a: bool) {
        self.emergency_profile_active = a;
    }

    /// Check if monitored_value exceeds threshold_normal.
    /// Returns true if value > threshold_normal.
    pub fn check_threshold(&self) -> bool {
        let val = self.monitored_value.as_i32().unwrap_or(0);
        let normal = self.threshold_normal.as_i32().unwrap_or(0);
        val > normal
    }

    /// Activate emergency profile. Returns true if state changed.
    pub fn activate_emergency(&mut self) -> bool {
        if !self.emergency_profile_active {
            self.emergency_profile_active = true;
            true
        } else {
            false
        }
    }

    /// Deactivate emergency profile. Returns true if state changed.
    pub fn deactivate_emergency(&mut self) -> bool {
        if self.emergency_profile_active {
            self.emergency_profile_active = false;
            true
        } else {
            false
        }
    }

    /// Comprehensive threshold evaluation.
    /// Takes current timestamp and returns the action to take.
    /// Considers min_over/under_threshold_duration before triggering.
    pub fn evaluate_thresholds(&mut self, current_timestamp: u32) -> ThresholdAction {
        let val = self.monitored_value.as_i32().unwrap_or(0);
        let over = self.threshold_over.as_i32().unwrap_or(i32::MAX);
        let under = self.threshold_under.as_i32().unwrap_or(i32::MIN);

        if val > over {
            // Over-threshold condition
            match self.over_threshold_start {
                None => {
                    self.over_threshold_start = Some(current_timestamp);
                    self.under_threshold_start = None;
                    ThresholdAction::Normal
                }
                Some(start) if current_timestamp - start >= self.min_over_duration => {
                    self.activate_emergency();
                    ThresholdAction::OverThreshold
                }
                _ => ThresholdAction::Normal,
            }
        } else if val < under {
            // Under-threshold condition
            match self.under_threshold_start {
                None => {
                    self.under_threshold_start = Some(current_timestamp);
                    self.over_threshold_start = None;
                    ThresholdAction::Normal
                }
                Some(start) if current_timestamp - start >= self.min_under_duration => {
                    self.activate_emergency();
                    ThresholdAction::UnderThreshold
                }
                _ => ThresholdAction::Normal,
            }
        } else {
            // Within normal range — reset timers, deactivate emergency
            self.over_threshold_start = None;
            self.under_threshold_start = None;
            self.deactivate_emergency();
            ThresholdAction::Normal
        }
    }

    /// Get the disconnect control action based on current state.
    /// Returns true if disconnect should be triggered (emergency active).
    /// This is the interface for C70 DisconnectControl linkage.
    pub fn should_disconnect(&self) -> bool {
        self.emergency_profile_active
    }

    /// Get the reconnect condition: emergency not active and value within range.
    pub fn should_reconnect(&self) -> bool {
        !self.emergency_profile_active && !self.check_threshold()
    }

    /// Set threshold parameters
    pub fn set_thresholds(&mut self, normal: DlmsData, over: DlmsData, under: DlmsData) {
        self.threshold_normal = normal;
        self.threshold_over = over;
        self.threshold_under = under;
    }

    /// Set duration parameters (in seconds)
    pub fn set_durations(&mut self, min_over: u32, min_under: u32) {
        self.min_over_duration = min_over;
        self.min_under_duration = min_under;
    }
}

impl CosemObject for Limiter {
    fn class_id(&self) -> u16 {
        71
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        11
    }
    fn method_count(&self) -> u8 {
        0
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => Some(dlms_axdr::encode(&self.monitored_value)),
            3 => Some(dlms_axdr::encode(&self.threshold_normal)),
            4 => Some(dlms_axdr::encode(&self.threshold_over)),
            5 => Some(dlms_axdr::encode(&self.threshold_under)),
            6 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.min_over_duration,
            ))),
            7 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.min_under_duration,
            ))),
            11 => Some(dlms_axdr::encode(&DlmsData::Boolean(
                self.emergency_profile_active,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                self.monitored_value =
                    dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                Ok(())
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_limiter() -> Limiter {
        let mut l = Limiter::new(ObisCode::CLOCK);
        l.set_thresholds(
            DlmsData::DoubleLong(100),
            DlmsData::DoubleLong(120),
            DlmsData::DoubleLong(20),
        );
        l.set_durations(10, 10);
        l
    }

    #[test]
    fn test_limiter_new() {
        let l = Limiter::new(ObisCode::CLOCK);
        assert_eq!(l.class_id(), 71);
    }

    #[test]
    fn test_limiter_monitored_value() {
        let mut l = Limiter::new(ObisCode::CLOCK);
        l.set_monitored_value(DlmsData::DoubleLong(50));
        assert_eq!(l.monitored_value().as_i32(), Some(50));
    }

    #[test]
    fn test_limiter_roundtrip() {
        let mut l = Limiter::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::DoubleLong(999));
        l.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(l.monitored_value().as_i32(), Some(999));
    }

    #[test]
    fn test_limiter_emergency() {
        let mut l = Limiter::new(ObisCode::CLOCK);
        l.set_emergency_active(true);
        assert!(l.emergency_active());
    }

    // --- New limiter logic tests ---

    #[test]
    fn test_check_threshold_below() {
        let l = default_limiter();
        assert!(!l.check_threshold()); // value=0, normal=100
    }

    #[test]
    fn test_check_threshold_above() {
        let mut l = default_limiter();
        l.set_monitored_value(DlmsData::DoubleLong(150));
        assert!(l.check_threshold());
    }

    #[test]
    fn test_activate_emergency_changes_state() {
        let mut l = default_limiter();
        assert!(l.activate_emergency());
        assert!(!l.activate_emergency()); // already active
    }

    #[test]
    fn test_deactivate_emergency_changes_state() {
        let mut l = default_limiter();
        l.activate_emergency();
        assert!(l.deactivate_emergency());
        assert!(!l.deactivate_emergency()); // already inactive
    }

    #[test]
    fn test_evaluate_thresholds_over_duration_met() {
        let mut l = default_limiter();
        l.set_monitored_value(DlmsData::DoubleLong(150)); // over threshold_over=120
        // First call: start timer
        assert_eq!(l.evaluate_thresholds(0), ThresholdAction::Normal);
        // Second call after duration met
        assert_eq!(l.evaluate_thresholds(15), ThresholdAction::OverThreshold);
        assert!(l.emergency_active());
    }

    #[test]
    fn test_evaluate_thresholds_under_duration_met() {
        let mut l = default_limiter();
        l.set_monitored_value(DlmsData::DoubleLong(10)); // under threshold_under=20
        assert_eq!(l.evaluate_thresholds(0), ThresholdAction::Normal);
        assert_eq!(l.evaluate_thresholds(15), ThresholdAction::UnderThreshold);
        assert!(l.emergency_active());
    }

    #[test]
    fn test_evaluate_thresholds_back_to_normal() {
        let mut l = default_limiter();
        l.set_monitored_value(DlmsData::DoubleLong(150));
        l.evaluate_thresholds(0);
        l.evaluate_thresholds(15); // triggers emergency
        l.set_monitored_value(DlmsData::DoubleLong(80)); // back to normal
        assert_eq!(l.evaluate_thresholds(20), ThresholdAction::Normal);
        assert!(!l.emergency_active());
    }

    #[test]
    fn test_should_disconnect() {
        let mut l = default_limiter();
        assert!(!l.should_disconnect());
        l.activate_emergency();
        assert!(l.should_disconnect());
    }

    #[test]
    fn test_should_reconnect() {
        let mut l = default_limiter();
        assert!(l.should_reconnect()); // not emergency, value=0 < 100
        l.activate_emergency();
        assert!(!l.should_reconnect());
    }
}
