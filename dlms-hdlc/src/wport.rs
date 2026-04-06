//! wPort constants for DLMS/COSEM (Green Book 7.3.3.4)
//!
//! Wrapper port numbers (wPort) are used for addressing DLMS/COSEM
//! Application Entities in UDP and TCP transport layers.

/// Reserved wrapper port numbers
pub const WPORT_DLMS_COSEM_UDP: u16 = 4059; // DLMS/COSEM UDP
pub const WPORT_DLMS_COSEM_TCP: u16 = 4059; // DLMS/COSEM TCP

/// Client side reserved addresses
pub const WPORT_NO_STATION: u16 = 0x0000; // No-station
pub const WPORT_CLIENT_MGMT_PROCESS: u16 = 0x0001; // Client Management Process
pub const WPORT_PUBLIC_CLIENT: u16 = 0x0010; // Public Client

/// Server side reserved addresses
pub const WPORT_MGMT_LOGICAL_DEVICE: u16 = 0x0001; // Management Logical Device
pub const WPORT_ALL_STATION: u16 = 0x007F; // All-station (Broadcast)

/// Check if a wPort number is reserved
pub fn is_reserved_wport(wport: u16) -> bool {
    const RESERVED_PORTS: &[u16] = &[
        WPORT_NO_STATION,
        WPORT_CLIENT_MGMT_PROCESS,
        WPORT_PUBLIC_CLIENT,
        WPORT_MGMT_LOGICAL_DEVICE,
        WPORT_ALL_STATION,
    ];

    RESERVED_PORTS.contains(&wport)
}

/// Get a description of a wPort number
pub fn get_wport_description(wport: u16) -> &'static str {
    match wport {
        WPORT_NO_STATION => "No-station",
        WPORT_CLIENT_MGMT_PROCESS => "Client Management Process",
        WPORT_PUBLIC_CLIENT => "Public Client",
        WPORT_MGMT_LOGICAL_DEVICE => "Management Logical Device",
        WPORT_ALL_STATION => "All-station (Broadcast)",
        WPORT_DLMS_COSEM_UDP => "DLMS/COSEM UDP",
        WPORT_DLMS_COSEM_TCP => "DLMS/COSEM TCP",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reserved_wport() {
        assert!(is_reserved_wport(WPORT_NO_STATION));
        assert!(is_reserved_wport(WPORT_CLIENT_MGMT_PROCESS));
        assert!(is_reserved_wport(WPORT_PUBLIC_CLIENT));
        assert!(is_reserved_wport(WPORT_MGMT_LOGICAL_DEVICE));
        assert!(is_reserved_wport(WPORT_ALL_STATION));
    }

    #[test]
    fn test_non_reserved_wport() {
        assert!(!is_reserved_wport(0x0011));
        assert!(!is_reserved_wport(0x0020));
    }

    #[test]
    fn test_wport_description() {
        // Test a few key descriptions
        assert_eq!(get_wport_description(WPORT_ALL_STATION), "All-station (Broadcast)");
        assert_eq!(get_wport_description(WPORT_PUBLIC_CLIENT), "Public Client");
        assert_eq!(get_wport_description(WPORT_NO_STATION), "No-station");
        // Unknown port returns Unknown
        assert_eq!(get_wport_description(0x1234), "Unknown");
    }

    #[test]
    fn test_wport_values() {
        assert_eq!(WPORT_DLMS_COSEM_UDP, 4059);
        assert_eq!(WPORT_DLMS_COSEM_TCP, 4059);
        assert_eq!(WPORT_NO_STATION, 0x0000);
        assert_eq!(WPORT_CLIENT_MGMT_PROCESS, 0x0001);
        assert_eq!(WPORT_PUBLIC_CLIENT, 0x0010);
        assert_eq!(WPORT_MGMT_LOGICAL_DEVICE, 0x0001);
        assert_eq!(WPORT_ALL_STATION, 0x007F);
    }
}
