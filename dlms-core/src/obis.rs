//! OBIS (Object Identification System) code - 6 byte identification code per IEC 62056-61

use core::fmt;

/// OBIS code: 6 bytes (A, B, C, D, E, F) per IEC 62056-61
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ObisCode {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
}

impl ObisCode {
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self { a, b, c, d, e, f }
    }

    pub fn from_bytes(bytes: [u8; 6]) -> Self {
        Self {
            a: bytes[0],
            b: bytes[1],
            c: bytes[2],
            d: bytes[3],
            e: bytes[4],
            f: bytes[5],
        }
    }

    pub fn to_bytes(&self) -> [u8; 6] {
        [self.a, self.b, self.c, self.d, self.e, self.f]
    }

    /// Common well-known OBIS codes
    pub const CLOCK: Self = Self::new(0, 0, 1, 0, 0, 255);
    pub const ACTIVE_POWER_L1: Self = Self::new(1, 0, 1, 7, 0, 255);
    pub const ACTIVE_POWER_L2: Self = Self::new(1, 0, 2, 7, 0, 255);
    pub const ACTIVE_POWER_L3: Self = Self::new(1, 0, 3, 7, 0, 255);
    pub const ACTIVE_ENERGY_IMPORT: Self = Self::new(1, 0, 1, 8, 0, 255);
    pub const VOLTAGE_L1: Self = Self::new(1, 0, 32, 7, 0, 255);
    pub const CURRENT_L1: Self = Self::new(1, 0, 31, 7, 0, 255);
    pub const DATA: Self = Self::new(1, 0, 0, 9, 0, 255);
}

impl fmt::Display for ObisCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}.{}.{}",
            self.a, self.b, self.c, self.d, self.e, self.f
        )
    }
}

impl fmt::Debug for ObisCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ObisCode({}.{}.{}.{}.{}.{})",
            self.a, self.b, self.c, self.d, self.e, self.f
        )
    }
}

impl core::str::FromStr for ObisCode {
    type Err = ObisParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 6 {
            return Err(ObisParseError::InvalidFormat);
        }
        let a = parts[0]
            .parse::<u8>()
            .map_err(|_| ObisParseError::InvalidValue)?;
        let b = parts[1]
            .parse::<u8>()
            .map_err(|_| ObisParseError::InvalidValue)?;
        let c = parts[2]
            .parse::<u8>()
            .map_err(|_| ObisParseError::InvalidValue)?;
        let d = parts[3]
            .parse::<u8>()
            .map_err(|_| ObisParseError::InvalidValue)?;
        let e = parts[4]
            .parse::<u8>()
            .map_err(|_| ObisParseError::InvalidValue)?;
        let f = parts[5]
            .parse::<u8>()
            .map_err(|_| ObisParseError::InvalidValue)?;
        Ok(Self::new(a, b, c, d, e, f))
    }
}

impl PartialOrd for ObisCode {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ObisCode {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.to_bytes().cmp(&other.to_bytes())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObisParseError {
    InvalidFormat,
    InvalidValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obis_display() {
        let obis = ObisCode::CLOCK;
        assert_eq!(format!("{}", obis), "0.0.1.0.0.255");
    }

    #[test]
    fn test_obis_from_str() {
        let obis: ObisCode = "1.0.1.8.0.255".parse().unwrap();
        assert_eq!(obis, ObisCode::ACTIVE_ENERGY_IMPORT);
    }

    #[test]
    fn test_obis_from_str_invalid() {
        let result: Result<ObisCode, _> = "1.0.1".parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_obis_to_bytes() {
        let obis = ObisCode::new(1, 0, 1, 8, 0, 255);
        assert_eq!(obis.to_bytes(), [1, 0, 1, 8, 0, 255]);
    }

    #[test]
    fn test_obis_from_bytes() {
        let obis = ObisCode::from_bytes([1, 0, 1, 8, 0, 255]);
        assert_eq!(obis, ObisCode::ACTIVE_ENERGY_IMPORT);
    }

    #[test]
    fn test_obis_ord() {
        let a = ObisCode::new(0, 0, 1, 0, 0, 255);
        let b = ObisCode::new(1, 0, 1, 8, 0, 255);
        assert!(a < b);
    }

    #[test]
    fn test_obis_equality() {
        let a = ObisCode::new(1, 0, 1, 8, 0, 255);
        let b = ObisCode::ACTIVE_ENERGY_IMPORT;
        assert_eq!(a, b);
    }

    #[test]
    fn test_obis_copy() {
        let a = ObisCode::CLOCK;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_obis_debug() {
        let obis = ObisCode::CLOCK;
        assert_eq!(format!("{:?}", obis), "ObisCode(0.0.1.0.0.255)");
    }

    #[test]
    fn test_obis_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(ObisCode::CLOCK);
        set.insert(ObisCode::CLOCK);
        assert_eq!(set.len(), 1);
    }
}
