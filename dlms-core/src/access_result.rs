//! AccessResult enumeration per DLMS specification

/// Access result codes returned by the server
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AccessResult {
    Success = 0,
    HardwareFault = 1,
    TemporaryFailure = 2,
    ReadWriteDenied = 3,
    ObjectUndefined = 4,
    ObjectClassInconsistent = 5,
    ObjectUnavailable = 6,
    TypeUnmatched = 7,
    ScopeOfAccessViolated = 8,
    DataBlockUnavailable = 9,
    LongGetAborted = 10,
    NoLongGetInProgress = 11,
    LongSetAborted = 12,
    NoLongSetInProgress = 13,
    DataBlockUndefined = 14,
    OtherReason = 15,
    Unknown = 255,
}

impl AccessResult {
    pub fn from_code(code: u8) -> Self {
        match code {
            0 => AccessResult::Success,
            1 => AccessResult::HardwareFault,
            2 => AccessResult::TemporaryFailure,
            3 => AccessResult::ReadWriteDenied,
            4 => AccessResult::ObjectUndefined,
            5 => AccessResult::ObjectClassInconsistent,
            6 => AccessResult::ObjectUnavailable,
            7 => AccessResult::TypeUnmatched,
            8 => AccessResult::ScopeOfAccessViolated,
            9 => AccessResult::DataBlockUnavailable,
            10 => AccessResult::LongGetAborted,
            11 => AccessResult::NoLongGetInProgress,
            12 => AccessResult::LongSetAborted,
            13 => AccessResult::NoLongSetInProgress,
            14 => AccessResult::DataBlockUndefined,
            15 => AccessResult::OtherReason,
            _ => AccessResult::Unknown,
        }
    }

    pub fn to_code(&self) -> u8 {
        *self as u8
    }

    pub fn is_success(&self) -> bool {
        matches!(self, AccessResult::Success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let r = AccessResult::Success;
        assert!(r.is_success());
        assert_eq!(r.to_code(), 0);
    }

    #[test]
    fn test_from_code() {
        assert_eq!(AccessResult::from_code(4), AccessResult::ObjectUndefined);
    }

    #[test]
    fn test_unknown_code() {
        assert_eq!(AccessResult::from_code(200), AccessResult::Unknown);
    }

    #[test]
    fn test_roundtrip_all() {
        for code in 0..=16u8 {
            let result = AccessResult::from_code(code);
            assert_eq!(AccessResult::from_code(result.to_code()), result);
        }
    }

    #[test]
    fn test_copy() {
        let a = AccessResult::TemporaryFailure;
        let b = a;
        assert_eq!(a, b);
    }
}
