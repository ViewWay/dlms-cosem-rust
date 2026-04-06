//! IC040 Push Setup
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: push_object_list (array of structures)
//! 3: service (long-unsigned)
//! 4: destination (octet-string)
//! 5: communication_window (structure: start_time, stop_time)
//! 6: randomisation_start_interval (long-unsigned)
//! 7: number_of_retries (unsigned)
//! 8: repetition_delay (long-unsigned)

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct PushObject {
    pub class_id: u16,
    pub logical_name: ObisCode,
    pub attribute: u8,
}

/// Communication window start/stop times (seconds from midnight)
#[derive(Debug, Clone)]
pub struct CommunicationWindow {
    pub start_time: u32,
    pub stop_time: u32,
}

/// Send method enum (UDP, TCP, CoAP)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SendMethod {
    Udp = 0,
    Tcp = 1,
    Coap = 2,
}

/// Result of a push attempt
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PushResult {
    Success,
    Failed(String),
    OutsideWindow,
    MaxRetriesExceeded,
}

pub struct PushSetup {
    logical_name: ObisCode,
    objects: Vec<PushObject>,
    service: u16,
    destination: Vec<u8>,
    retries: u8,
    recurrence_delay: u32,
    communication_window: Option<CommunicationWindow>,
    randomisation_interval: u32,
    send_method: SendMethod,
}

impl PushSetup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            objects: Vec::new(),
            service: 0,
            destination: Vec::new(),
            retries: 3,
            recurrence_delay: 60,
            communication_window: None,
            randomisation_interval: 0,
            send_method: SendMethod::Udp,
        }
    }

    pub fn add_object(&mut self, class_id: u16, ln: ObisCode, attr: u8) {
        self.objects.push(PushObject {
            class_id,
            logical_name: ln,
            attribute: attr,
        });
    }

    pub fn object_count(&self) -> usize {
        self.objects.len()
    }
    pub fn service(&self) -> u16 {
        self.service
    }
    pub fn set_service(&mut self, s: u16) {
        self.service = s;
    }

    pub fn set_communication_window(&mut self, start: u32, stop: u32) {
        self.communication_window = Some(CommunicationWindow {
            start_time: start,
            stop_time: stop,
        });
    }

    pub fn communication_window(&self) -> Option<&CommunicationWindow> {
        self.communication_window.as_ref()
    }

    pub fn retries(&self) -> u8 {
        self.retries
    }

    pub fn recurrence_delay(&self) -> u32 {
        self.recurrence_delay
    }

    /// Check if current seconds-from-midnight falls within the communication window.
    /// Returns true if no window is configured (always allowed).
    pub fn is_within_window(&self, seconds_since_midnight: u32) -> bool {
        match &self.communication_window {
            None => true,
            Some(w) if w.start_time <= w.stop_time => {
                seconds_since_midnight >= w.start_time && seconds_since_midnight <= w.stop_time
            }
            Some(w) => {
                // Wraps around midnight
                seconds_since_midnight >= w.start_time || seconds_since_midnight <= w.stop_time
            }
        }
    }

    /// Whether another retry attempt should be made.
    pub fn should_retry(&self, attempt: u8) -> bool {
        attempt < self.retries
    }

    /// Get the delay before the next retry.
    pub fn get_retry_delay(&self, attempt: u8) -> u32 {
        if attempt == 0 {
            self.randomisation_interval
        } else {
            self.recurrence_delay
        }
    }

    /// Serialize push_object_list and prepare the push payload.
    /// Returns the encoded bytes for attribute 2 (push_object_list).
    pub fn serialize_push_list(&self) -> Option<Vec<u8>> {
        self.attribute_to_bytes(2)
    }

    /// Trigger a push operation. Checks window, serializes data.
    /// In production this would send over TCP/UDP/CoAP; here we return the result.
    pub fn execute_push(&self, seconds_since_midnight: u32) -> PushResult {
        if !self.is_within_window(seconds_since_midnight) {
            return PushResult::OutsideWindow;
        }
        if self.objects.is_empty() {
            return PushResult::Failed("empty push object list".into());
        }
        // Serialize payload
        match self.serialize_push_list() {
            Some(payload) => {
                // In a real implementation, payload would be sent via
                // TCP/UDP/CoAP to self.destination using self.send_method.
                let _ = payload;
                PushResult::Success
            }
            None => PushResult::Failed("serialization failed".into()),
        }
    }

    /// Simulate a push with retries, returning final result.
    pub fn execute_push_with_retries(
        &self,
        seconds_since_midnight: u32,
        mut send_fn: impl FnMut(&[u8]) -> bool,
    ) -> PushResult {
        if !self.is_within_window(seconds_since_midnight) {
            return PushResult::OutsideWindow;
        }
        if self.objects.is_empty() {
            return PushResult::Failed("empty push object list".into());
        }

        let payload = match self.serialize_push_list() {
            Some(p) => p,
            None => return PushResult::Failed("serialization failed".into()),
        };

        for attempt in 0..=self.retries {
            if send_fn(&payload) {
                return PushResult::Success;
            }
            if !self.should_retry(attempt + 1) {
                break;
            }
            // In production: sleep(self.get_retry_delay(attempt))
            let _ = self.get_retry_delay(attempt);
        }
        PushResult::MaxRetriesExceeded
    }
}

impl CosemObject for PushSetup {
    fn class_id(&self) -> u16 {
        40
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        8
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
            2 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.objects
                    .iter()
                    .map(|o| {
                        DlmsData::Structure(vec![
                            DlmsData::LongUnsigned(o.class_id),
                            DlmsData::OctetString(o.logical_name.to_bytes().to_vec()),
                            DlmsData::Integer(o.attribute as i8),
                            DlmsData::Unsigned(0),
                        ])
                    })
                    .collect(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(self.service))),
            4 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.destination.clone(),
            ))),
            5 => {
                if let Some(w) = &self.communication_window {
                    Some(dlms_axdr::encode(&DlmsData::Structure(vec![
                        DlmsData::DoubleLongUnsigned(w.start_time),
                        DlmsData::DoubleLongUnsigned(w.stop_time),
                    ])))
                } else {
                    Some(dlms_axdr::encode(&DlmsData::Structure(vec![
                        DlmsData::DoubleLongUnsigned(0),
                        DlmsData::DoubleLongUnsigned(86_400),
                    ])))
                }
            }
            6 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.randomisation_interval as u16,
            ))),
            7 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.retries))),
            8 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.recurrence_delay as u16,
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

    fn default_ps() -> PushSetup {
        PushSetup::new(ObisCode::new(0, 0, 15, 0, 0, 255))
    }

    #[test]
    fn test_push_setup_class_id() {
        let ps = default_ps();
        assert_eq!(ps.class_id(), 40);
    }

    #[test]
    fn test_push_setup_add_object() {
        let mut ps = default_ps();
        ps.add_object(8, ObisCode::CLOCK, 2);
        assert_eq!(ps.object_count(), 1);
    }

    #[test]
    fn test_push_setup_attribute_count() {
        let ps = default_ps();
        assert_eq!(ps.attribute_count(), 8);
    }

    #[test]
    fn test_push_setup_method_count() {
        let ps = default_ps();
        assert_eq!(ps.method_count(), 0);
    }

    #[test]
    fn test_push_setup_attr1() {
        let ps = default_ps();
        let bytes = ps.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_push_setup_attr2_objects() {
        let mut ps = default_ps();
        ps.add_object(8, ObisCode::CLOCK, 2);
        ps.add_object(3, ObisCode::ACTIVE_POWER_L1, 2);
        let bytes = ps.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_push_setup_service() {
        let mut ps = default_ps();
        assert_eq!(ps.service(), 0);
        ps.set_service(1234);
        assert_eq!(ps.service(), 1234);
    }

    #[test]
    fn test_push_setup_multiple_objects() {
        let mut ps = default_ps();
        ps.add_object(8, ObisCode::CLOCK, 2);
        ps.add_object(1, ObisCode::new(1, 0, 0, 9, 0, 255), 2);
        ps.add_object(7, ObisCode::new(1, 0, 99, 1, 0, 255), 2);
        assert_eq!(ps.object_count(), 3);
    }

    // --- New push logic tests ---

    #[test]
    fn test_is_within_window_no_window() {
        let ps = default_ps();
        assert!(ps.is_within_window(0));
        assert!(ps.is_within_window(86_399));
    }

    #[test]
    fn test_is_within_window_normal() {
        let mut ps = default_ps();
        ps.set_communication_window(3600, 7200); // 1:00-2:00 AM
        assert!(!ps.is_within_window(0));
        assert!(ps.is_within_window(3600));
        assert!(ps.is_within_window(5400));
        assert!(!ps.is_within_window(7201));
    }

    #[test]
    fn test_is_within_window_overnight() {
        let mut ps = default_ps();
        ps.set_communication_window(82800, 3600); // 23:00 - 01:00
        assert!(!ps.is_within_window(36000));
        assert!(ps.is_within_window(84000));
        assert!(ps.is_within_window(0));
        assert!(ps.is_within_window(1800));
        assert!(!ps.is_within_window(7200));
    }

    #[test]
    fn test_should_retry() {
        let ps = default_ps(); // retries = 3
        assert!(ps.should_retry(0));
        assert!(ps.should_retry(1));
        assert!(ps.should_retry(2));
        assert!(!ps.should_retry(3));
    }

    #[test]
    fn test_get_retry_delay_first_attempt() {
        let mut ps = default_ps();
        ps.set_communication_window(0, 0); // just set something
        assert_eq!(ps.get_retry_delay(0), 0); // randomisation_interval = 0
    }

    #[test]
    fn test_execute_push_success() {
        let mut ps = default_ps();
        ps.add_object(8, ObisCode::CLOCK, 2);
        assert_eq!(ps.execute_push(0), PushResult::Success);
    }

    #[test]
    fn test_execute_push_outside_window() {
        let mut ps = default_ps();
        ps.add_object(8, ObisCode::CLOCK, 2);
        ps.set_communication_window(3600, 7200);
        assert_eq!(ps.execute_push(0), PushResult::OutsideWindow);
    }

    #[test]
    fn test_execute_push_empty_list() {
        let ps = default_ps();
        assert_eq!(ps.execute_push(0), PushResult::Failed("empty push object list".into()));
    }

    #[test]
    fn test_execute_push_with_retries_success_on_second() {
        let mut ps = default_ps();
        ps.add_object(8, ObisCode::CLOCK, 2);
        let mut call_count = 0u8;
        let result = ps.execute_push_with_retries(0, |_| {
            call_count += 1;
            call_count == 2
        });
        assert_eq!(result, PushResult::Success);
        assert_eq!(call_count, 2);
    }

    #[test]
    fn test_execute_push_with_retries_all_fail() {
        let mut ps = default_ps();
        ps.add_object(8, ObisCode::CLOCK, 2);
        let result = ps.execute_push_with_retries(0, |_| false);
        assert_eq!(result, PushResult::MaxRetriesExceeded);
    }

    #[test]
    fn test_serialize_push_list() {
        let mut ps = default_ps();
        ps.add_object(8, ObisCode::CLOCK, 2);
        let payload = ps.serialize_push_list().unwrap();
        assert!(!payload.is_empty());
    }
}
