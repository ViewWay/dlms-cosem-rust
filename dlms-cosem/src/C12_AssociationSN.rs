//! IC02 Association SN (Short Name) - DLMS Short Name Association

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IC02 Association SN - Short Name Association Object
/// 
/// This class is used to model an association using short naming (SN).
/// It's used in legacy DLMS implementations and provides backward compatibility.
#[derive(Debug, Clone)]
pub struct AssociationSN {
    logical_name: ObisCode,
    object_list: Vec<(u16, u16)>, // (base_address, class_id)
    access_rights: Vec<(u16, u8)>, // (base_address, access_rights)
    associated_partners: (u16, u16), // (client_sap, server_sap)
    lls_secret: Vec<u8>,
}

impl AssociationSN {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            object_list: Vec::new(),
            access_rights: Vec::new(),
            associated_partners: (16, 1),
            lls_secret: Vec::new(),
        }
    }

    pub fn object_list(&self) -> &[(u16, u16)] {
        &self.object_list
    }

    pub fn add_object(&mut self, base_address: u16, class_id: u16) {
        self.object_list.push((base_address, class_id));
    }

    pub fn access_rights(&self) -> &[(u16, u8)] {
        &self.access_rights
    }

    pub fn set_access_rights(&mut self, base_address: u16, rights: u8) {
        self.access_rights.push((base_address, rights));
    }

    pub fn associated_partners(&self) -> (u16, u16) {
        self.associated_partners
    }

    pub fn set_associated_partners(&mut self, client_sap: u16, server_sap: u16) {
        self.associated_partners = (client_sap, server_sap);
    }

    pub fn lls_secret(&self) -> &[u8] {
        &self.lls_secret
    }

    pub fn set_lls_secret(&mut self, secret: Vec<u8>) {
        self.lls_secret = secret;
    }
}

impl CosemObject for AssociationSN {
    fn class_id(&self) -> u16 {
        12
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        5
    }

    fn method_count(&self) -> u8 {
        2
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
                // Object list - simplified encoding
                let mut bytes = vec![0x01]; // Array
                bytes.push(self.object_list.len() as u8);
                for (base_addr, class_id) in &self.object_list {
                    bytes.push(0x02); // Structure
                    bytes.push(0x02); // 2 elements
                    bytes.extend_from_slice(&base_addr.to_be_bytes()[2..]);
                    bytes.extend_from_slice(&(*class_id as u32).to_be_bytes()[2..]);
                }
                Some(bytes)
            }
            3 => {
                // Access rights
                let mut bytes = vec![0x01]; // Array
                bytes.push(self.access_rights.len() as u8);
                for (base_addr, rights) in &self.access_rights {
                    bytes.push(0x02); // Structure
                    bytes.push(0x02); // 2 elements
                    bytes.extend_from_slice(&base_addr.to_be_bytes()[2..]);
                    bytes.push(*rights);
                }
                Some(bytes)
            }
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 | 3 | 4 | 5 => {
                let _ = data;
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
    fn test_association_sn_new() {
        let assoc = AssociationSN::new(ObisCode::new(0, 0, 40, 0, 1, 255));
        assert_eq!(assoc.class_id(), 12);
    }

    #[test]
    fn test_association_sn_object_list() {
        let mut assoc = AssociationSN::new(ObisCode::new(0, 0, 40, 0, 1, 255));
        assoc.add_object(0x1000, 1);
        assoc.add_object(0x2000, 3);
        assert_eq!(assoc.object_list().len(), 2);
    }

    #[test]
    fn test_association_sn_access_rights() {
        let mut assoc = AssociationSN::new(ObisCode::new(0, 0, 40, 0, 1, 255));
        assoc.set_access_rights(0x1000, 0x3F);
        assert_eq!(assoc.access_rights().len(), 1);
    }

    #[test]
    fn test_association_sn_partners() {
        let mut assoc = AssociationSN::new(ObisCode::new(0, 0, 40, 0, 1, 255));
        assoc.set_associated_partners(64, 1);
        assert_eq!(assoc.associated_partners(), (64, 1));
    }

    #[test]
    fn test_association_sn_lls_secret() {
        let mut assoc = AssociationSN::new(ObisCode::new(0, 0, 40, 0, 1, 255));
        assoc.set_lls_secret(vec![0x01, 0x02, 0x03]);
        assert_eq!(assoc.lls_secret(), &[0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_association_sn_attribute_count() {
        let assoc = AssociationSN::new(ObisCode::new(0, 0, 40, 0, 1, 255));
        assert_eq!(assoc.attribute_count(), 5);
    }

    #[test]
    fn test_association_sn_method_count() {
        let assoc = AssociationSN::new(ObisCode::new(0, 0, 40, 0, 1, 255));
        assert_eq!(assoc.method_count(), 2);
    }
}
