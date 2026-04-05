//! IC09 Association LN (Logical Name) - DLMS Logical Name Association

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IC09 Association LN - Logical Name Association Object
/// 
/// This class is used to model an association using logical naming (LN).
/// It's fundamental for DLMS/COSEM client-server communication.
#[derive(Debug, Clone)]
pub struct AssociationLN {
    logical_name: ObisCode,
    object_list: Vec<(u16, ObisCode)>,
    associated_partners: (u16, u16), // (client_sap, server_sap)
    application_context_name: String,
    authentication_mechanism_name: String,
    lls_secret: Vec<u8>,
    hls_secret: Vec<u8>,
    authentication_status: bool,
}

impl AssociationLN {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            object_list: Vec::new(),
            associated_partners: (16, 1), // Default client SAP 16, server SAP 1
            application_context_name: String::new(),
            authentication_mechanism_name: String::new(),
            lls_secret: Vec::new(),
            hls_secret: Vec::new(),
            authentication_status: false,
        }
    }

    pub fn object_list(&self) -> &[(u16, ObisCode)] {
        &self.object_list
    }

    pub fn add_object(&mut self, class_id: u16, obis: ObisCode) {
        self.object_list.push((class_id, obis));
    }

    pub fn associated_partners(&self) -> (u16, u16) {
        self.associated_partners
    }

    pub fn set_associated_partners(&mut self, client_sap: u16, server_sap: u16) {
        self.associated_partners = (client_sap, server_sap);
    }

    pub fn application_context_name(&self) -> &str {
        &self.application_context_name
    }

    pub fn set_application_context_name(&mut self, name: String) {
        self.application_context_name = name;
    }

    pub fn is_authenticated(&self) -> bool {
        self.authentication_status
    }

    pub fn set_authenticated(&mut self, status: bool) {
        self.authentication_status = status;
    }
}

impl CosemObject for AssociationLN {
    fn class_id(&self) -> u16 {
        9
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        9
    }

    fn method_count(&self) -> u8 {
        5
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
                // Object list - simplified array encoding
                let mut bytes = vec![0x01]; // Array
                bytes.push(self.object_list.len() as u8);
                for (class_id, obis) in &self.object_list {
                    bytes.push(0x02); // Structure
                    bytes.push(0x02); // 2 elements
                    bytes.extend_from_slice(&(*class_id as u32).to_be_bytes()[2..]);
                    bytes.push(0x09);
                    bytes.push(0x06);
                    bytes.extend_from_slice(&obis.to_bytes());
                }
                Some(bytes)
            }
            3 => {
                // Associated partners
                let mut bytes = vec![0x02, 0x02]; // Structure with 2 elements
                bytes.extend_from_slice(&self.associated_partners.0.to_be_bytes()[2..]);
                bytes.extend_from_slice(&self.associated_partners.1.to_be_bytes()[2..]);
                Some(bytes)
            }
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => {
                // For now, just accept the data without parsing
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
    fn test_association_ln_new() {
        let assoc = AssociationLN::new(ObisCode::new(0, 0, 40, 0, 0, 255));
        assert_eq!(assoc.class_id(), 9);
    }

    #[test]
    fn test_association_ln_partners() {
        let mut assoc = AssociationLN::new(ObisCode::new(0, 0, 40, 0, 0, 255));
        assoc.set_associated_partners(32, 1);
        assert_eq!(assoc.associated_partners(), (32, 1));
    }

    #[test]
    fn test_association_ln_object_list() {
        let mut assoc = AssociationLN::new(ObisCode::new(0, 0, 40, 0, 0, 255));
        assoc.add_object(1, ObisCode::DATA);
        assoc.add_object(3, ObisCode::ACTIVE_POWER_L1);
        assert_eq!(assoc.object_list().len(), 2);
    }

    #[test]
    fn test_association_ln_authentication() {
        let mut assoc = AssociationLN::new(ObisCode::new(0, 0, 40, 0, 0, 255));
        assert!(!assoc.is_authenticated());
        assoc.set_authenticated(true);
        assert!(assoc.is_authenticated());
    }

    #[test]
    fn test_association_ln_attribute_to_bytes() {
        let assoc = AssociationLN::new(ObisCode::new(0, 0, 40, 0, 0, 255));
        let bytes = assoc.attribute_to_bytes(1).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_association_ln_attribute_count() {
        let assoc = AssociationLN::new(ObisCode::new(0, 0, 40, 0, 0, 255));
        assert_eq!(assoc.attribute_count(), 9);
    }

    #[test]
    fn test_association_ln_method_count() {
        let assoc = AssociationLN::new(ObisCode::new(0, 0, 40, 0, 0, 255));
        assert_eq!(assoc.method_count(), 5);
    }
}
