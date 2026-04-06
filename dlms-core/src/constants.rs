//! DLMS/COSEM protocol constants
//!
//! This module defines well-known constants used in DLMS/COSEM protocol.

/// DLMS/COSEM UDP reserved port (Green Book 7.3.3.4)
///
/// Port 4059 is the IANA-assigned port for DLMS/COSEM over UDP.
/// This port should be used for DLMS/COSEM UDP communications.
pub const DLMS_UDP_PORT: u16 = 4059;

/// DLMS/COSEM TCP default port
///
/// While not officially reserved, port 4059 is commonly used for
/// DLMS/COSEM over TCP as well.
pub const DLMS_TCP_PORT: u16 = 4059;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dlms_udp_port() {
        assert_eq!(DLMS_UDP_PORT, 4059);
    }

    #[test]
    fn test_dlms_tcp_port() {
        assert_eq!(DLMS_TCP_PORT, 4059);
    }
}
