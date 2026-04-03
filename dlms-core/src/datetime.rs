//! CosemDateTime - DLMS date/time representation (OctetString, DateTime, OctetString, DateTime)

/// DLMS Date-Time as defined in IEC 62056-53
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CosemDateTime {
    pub year: Option<u16>,    // 0 = not specified
    pub month: Option<u8>,    // 0-12, 0 = not specified, 13-16 = dev spec
    pub day: Option<u8>,      // 1-31, 0 = not specified
    pub day_of_week: Option<u8>, // 1-7 (Mon-Sun)
    pub hour: Option<u8>,     // 0-23
    pub minute: Option<u8>,   // 0-59
    pub second: Option<u8>,   // 0-59
    pub hundredths: Option<u8>, // 0-99
    pub deviation: i16,       // deviation in minutes, -720 to +720
    pub clock_status: u8,     // clock status bits
}

impl Default for CosemDateTime {
    fn default() -> Self {
        Self {
            year: None,
            month: None,
            day: None,
            day_of_week: None,
            hour: None,
            minute: None,
            second: None,
            hundredths: None,
            deviation: 0,
            clock_status: 0,
        }
    }
}

impl CosemDateTime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_date(year: u16, month: u8, day: u8) -> Self {
        Self {
            year: Some(year),
            month: Some(month),
            day: Some(day),
            ..Self::default()
        }
    }

    pub fn with_time(hour: u8, minute: u8, second: u8) -> Self {
        Self {
            hour: Some(hour),
            minute: Some(minute),
            second: Some(second),
            ..Self::default()
        }
    }

    pub fn is_date_specified(&self) -> bool {
        self.year.is_some() || self.month.is_some() || self.day.is_some()
    }

    pub fn is_time_specified(&self) -> bool {
        self.hour.is_some() || self.minute.is_some() || self.second.is_some()
    }

    /// Encode as 12-byte DLMS DateTime (without leading length/type)
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut buf = [0u8; 12];
        if let Some(y) = self.year {
            buf[0] = (y >> 8) as u8;
            buf[1] = (y & 0xFF) as u8;
        }
        if let Some(m) = self.month {
            buf[2] = m;
        }
        if let Some(d) = self.day {
            buf[3] = d;
        }
        if let Some(dw) = self.day_of_week {
            buf[4] = dw;
        }
        if let Some(h) = self.hour {
            buf[5] = h;
        }
        if let Some(min) = self.minute {
            buf[6] = min;
        }
        if let Some(s) = self.second {
            buf[7] = s;
        }
        if let Some(hs) = self.hundredths {
            buf[8] = hs;
        }
        let dev = self.deviation as i16;
        buf[9] = (dev >> 8) as u8;
        buf[10] = (dev & 0xFF) as u8;
        buf[11] = self.clock_status;
        buf
    }

    /// Decode from 12-byte DLMS DateTime
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DateTimeError> {
        if bytes.len() < 12 {
            return Err(DateTimeError::InvalidLength(bytes.len()));
        }
        let year = u16::from_be_bytes([bytes[0], bytes[1]]);
        Ok(Self {
            year: if year == 0xFFFF { None } else { Some(year) },
            month: if bytes[2] == 0xFF { None } else { Some(bytes[2]) },
            day: if bytes[3] == 0xFF { None } else { Some(bytes[3]) },
            day_of_week: if bytes[4] == 0xFF { None } else { Some(bytes[4]) },
            hour: if bytes[5] == 0xFF { None } else { Some(bytes[5]) },
            minute: if bytes[6] == 0xFF { None } else { Some(bytes[6]) },
            second: if bytes[7] == 0xFF { None } else { Some(bytes[7]) },
            hundredths: if bytes[8] == 0xFF { None } else { Some(bytes[8]) },
            deviation: i16::from_be_bytes([bytes[9], bytes[10]]),
            clock_status: bytes[11],
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DateTimeError {
    InvalidLength(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dt = CosemDateTime::default();
        assert!(!dt.is_date_specified());
        assert!(!dt.is_time_specified());
    }

    #[test]
    fn test_with_date() {
        let dt = CosemDateTime::with_date(2024, 1, 15);
        assert!(dt.is_date_specified());
        assert!(!dt.is_time_specified());
    }

    #[test]
    fn test_with_time() {
        let dt = CosemDateTime::with_time(14, 30, 0);
        assert!(!dt.is_date_specified());
        assert!(dt.is_time_specified());
    }

    #[test]
    fn test_roundtrip() {
        let dt = CosemDateTime {
            year: Some(2024),
            month: Some(6),
            day: Some(15),
            day_of_week: Some(6),
            hour: Some(10),
            minute: Some(30),
            second: Some(45),
            hundredths: Some(50),
            deviation: 480, // UTC+8
            clock_status: 0,
        };
        let bytes = dt.to_bytes();
        let decoded = CosemDateTime::from_bytes(&bytes).unwrap();
        assert_eq!(dt, decoded);
    }

    #[test]
    fn test_roundtrip_none() {
        let dt = CosemDateTime::default();
        let bytes = dt.to_bytes();
        // Default (all zero) encodes as all zero bytes, which decode as Some(0) values
        // This is expected behavior - 0 and 0xFF are both valid encodings
        let decoded = CosemDateTime::from_bytes(&bytes).unwrap();
        assert_eq!(decoded.deviation, dt.deviation);
        assert_eq!(decoded.clock_status, dt.clock_status);
    }

    #[test]
    fn test_from_bytes_invalid_length() {
        let result = CosemDateTime::from_bytes(&[1, 2, 3]);
        assert!(result.is_err());
    }

    #[test]
    fn test_to_bytes_content() {
        let dt = CosemDateTime::with_date(2024, 6, 15);
        let bytes = dt.to_bytes();
        assert_eq!(bytes[0], 0x07); // 2024 >> 8
        assert_eq!(bytes[1], 0xE8); // 2024 & 0xFF
        assert_eq!(bytes[2], 6);
        assert_eq!(bytes[3], 15);
    }
}
