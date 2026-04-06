//! IC42 IPv4 Setup - IPv4 Network Configuration

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IC42 IPv4 Setup - IPv4 Network Configuration Object
/// 
/// This class manages IPv4 network configuration.
/// Used in networked metering and IoT systems.
#[derive(Debug, Clone)]
pub struct Ipv4Setup {
    logical_name: ObisCode,
    ip_address: [u8; 4],
    subnet_mask: [u8; 4],
    gateway: [u8; 4],
    primary_dns: [u8; 4],
    secondary_dns: [u8; 4],
    dhcp_enabled: bool,
}

impl Ipv4Setup {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            ip_address: [0; 4],
            subnet_mask: [255, 255, 255, 0],
            gateway: [0; 4],
            primary_dns: [0; 4],
            secondary_dns: [0; 4],
            dhcp_enabled: false,
        }
    }

    pub fn ip_address(&self) -> [u8; 4] {
        self.ip_address
    }

    pub fn set_ip_address(&mut self, ip: [u8; 4]) {
        self.ip_address = ip;
    }

    pub fn subnet_mask(&self) -> [u8; 4] {
        self.subnet_mask
    }

    pub fn set_subnet_mask(&mut self, mask: [u8; 4]) {
        self.subnet_mask = mask;
    }

    pub fn gateway(&self) -> [u8; 4] {
        self.gateway
    }

    pub fn set_gateway(&mut self, gateway: [u8; 4]) {
        self.gateway = gateway;
    }

    pub fn primary_dns(&self) -> [u8; 4] {
        self.primary_dns
    }

    pub fn set_primary_dns(&mut self, dns: [u8; 4]) {
        self.primary_dns = dns;
    }

    pub fn secondary_dns(&self) -> [u8; 4] {
        self.secondary_dns
    }

    pub fn set_secondary_dns(&mut self, dns: [u8; 4]) {
        self.secondary_dns = dns;
    }

    pub fn dhcp_enabled(&self) -> bool {
        self.dhcp_enabled
    }

    pub fn set_dhcp_enabled(&mut self, enabled: bool) {
        self.dhcp_enabled = enabled;
    }

    fn ip_to_string(ip: [u8; 4]) -> String {
        format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3])
    }
}

impl CosemObject for Ipv4Setup {
    fn class_id(&self) -> u16 {
        42
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        7
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
            2 => {
                // IP address - octet string
                let mut bytes = vec![0x09, 0x04];
                bytes.extend_from_slice(&self.ip_address);
                Some(bytes)
            }
            3 => {
                // Subnet mask
                let mut bytes = vec![0x09, 0x04];
                bytes.extend_from_slice(&self.subnet_mask);
                Some(bytes)
            }
            4 => {
                // Gateway
                let mut bytes = vec![0x09, 0x04];
                bytes.extend_from_slice(&self.gateway);
                Some(bytes)
            }
            5 => {
                // Primary DNS
                let mut bytes = vec![0x09, 0x04];
                bytes.extend_from_slice(&self.primary_dns);
                Some(bytes)
            }
            6 => {
                // Secondary DNS
                let mut bytes = vec![0x09, 0x04];
                bytes.extend_from_slice(&self.secondary_dns);
                Some(bytes)
            }
            7 => {
                // DHCP enabled
                Some(vec![0x0F, if self.dhcp_enabled { 1 } else { 0 }])
            }
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                if data.len() >= 6 {
                    self.ip_address.copy_from_slice(&data[2..6]);
                }
                Ok(())
            }
            3 => {
                if data.len() >= 6 {
                    self.subnet_mask.copy_from_slice(&data[2..6]);
                }
                Ok(())
            }
            4 => {
                if data.len() >= 6 {
                    self.gateway.copy_from_slice(&data[2..6]);
                }
                Ok(())
            }
            5 => {
                if data.len() >= 6 {
                    self.primary_dns.copy_from_slice(&data[2..6]);
                }
                Ok(())
            }
            6 => {
                if data.len() >= 6 {
                    self.secondary_dns.copy_from_slice(&data[2..6]);
                }
                Ok(())
            }
            7 => {
                if data.len() >= 2 {
                    self.dhcp_enabled = data[1] != 0;
                }
                Ok(())
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_setup_new() {
        let ipv4 = Ipv4Setup::new(ObisCode::new(0, 0, 42, 0, 0, 255));
        assert_eq!(ipv4.class_id(), 42);
    }

    #[test]
    fn test_ipv4_setup_ip_address() {
        let mut ipv4 = Ipv4Setup::new(ObisCode::new(0, 0, 42, 0, 0, 255));
        ipv4.set_ip_address([192, 168, 1, 100]);
        assert_eq!(ipv4.ip_address(), [192, 168, 1, 100]);
    }

    #[test]
    fn test_ipv4_setup_subnet_mask() {
        let mut ipv4 = Ipv4Setup::new(ObisCode::new(0, 0, 42, 0, 0, 255));
        ipv4.set_subnet_mask([255, 255, 255, 0]);
        assert_eq!(ipv4.subnet_mask(), [255, 255, 255, 0]);
    }

    #[test]
    fn test_ipv4_setup_gateway() {
        let mut ipv4 = Ipv4Setup::new(ObisCode::new(0, 0, 42, 0, 0, 255));
        ipv4.set_gateway([192, 168, 1, 1]);
        assert_eq!(ipv4.gateway(), [192, 168, 1, 1]);
    }

    #[test]
    fn test_ipv4_setup_dns() {
        let mut ipv4 = Ipv4Setup::new(ObisCode::new(0, 0, 42, 0, 0, 255));
        ipv4.set_primary_dns([8, 8, 8, 8]);
        ipv4.set_secondary_dns([8, 8, 4, 4]);
        assert_eq!(ipv4.primary_dns(), [8, 8, 8, 8]);
        assert_eq!(ipv4.secondary_dns(), [8, 8, 4, 4]);
    }

    #[test]
    fn test_ipv4_setup_dhcp() {
        let mut ipv4 = Ipv4Setup::new(ObisCode::new(0, 0, 42, 0, 0, 255));
        ipv4.set_dhcp_enabled(true);
        assert!(ipv4.dhcp_enabled());
    }

    #[test]
    fn test_ipv4_setup_attribute_count() {
        let ipv4 = Ipv4Setup::new(ObisCode::new(0, 0, 42, 0, 0, 255));
        assert_eq!(ipv4.attribute_count(), 7);
    }

    #[test]
    fn test_ipv4_setup_method_count() {
        let ipv4 = Ipv4Setup::new(ObisCode::new(0, 0, 42, 0, 0, 255));
        assert_eq!(ipv4.method_count(), 0);
    }

    #[test]
    fn test_ipv4_setup_ip_to_string() {
        let ip = [192, 168, 1, 100];
        let s = Ipv4Setup::ip_to_string(ip);
        assert_eq!(s, "192.168.1.100");
    }
}
