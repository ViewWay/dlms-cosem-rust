//! IC6 Register Activation
//! Blue Book Ed16: class_id=6, version=0
//! Manages register activation with mask lists

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Register Activation entry
#[derive(Debug, Clone)]
pub struct RegisterAssignment {
    pub register_reference: ObisCode,
    pub mask_list: Vec<DlmsData>,
}

/// Register Activation - controls which registers are active
pub struct RegisterActivation {
    logical_name: ObisCode,
    register_assignments: Vec<RegisterAssignment>,
}

impl RegisterActivation {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            register_assignments: vec![],
        }
    }

    pub fn register_assignments(&self) -> &[RegisterAssignment] {
        &self.register_assignments
    }

    pub fn add_register(&mut self, assignment: RegisterAssignment) {
        self.register_assignments.push(assignment);
    }

    pub fn remove_register(&mut self, index: usize) -> Option<RegisterAssignment> {
        if index < self.register_assignments.len() {
            Some(self.register_assignments.remove(index))
        } else {
            None
        }
    }
}

impl CosemObject for RegisterActivation {
    fn class_id(&self) -> u16 {
        6
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        3
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
                let list: Vec<DlmsData> = self
                    .register_assignments
                    .iter()
                    .map(|ra| {
                        DlmsData::Structure(vec![
                            DlmsData::OctetString(ra.register_reference.to_bytes().to_vec()),
                            DlmsData::Array(ra.mask_list.clone()),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(list)))
            }
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, _attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        Err(CosemObjectError::AttributeNotSupported(_attr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_activation_new() {
        let ra = RegisterActivation::new(ObisCode::CLOCK);
        assert_eq!(ra.class_id(), 6);
    }

    #[test]
    fn test_register_activation_add() {
        let mut ra = RegisterActivation::new(ObisCode::CLOCK);
        ra.add_register(RegisterAssignment {
            register_reference: ObisCode::CLOCK,
            mask_list: vec![],
        });
        assert_eq!(ra.register_assignments().len(), 1);
    }

    #[test]
    fn test_register_activation_remove() {
        let mut ra = RegisterActivation::new(ObisCode::CLOCK);
        ra.add_register(RegisterAssignment {
            register_reference: ObisCode::CLOCK,
            mask_list: vec![],
        });
        ra.remove_register(0);
        assert!(ra.register_assignments().is_empty());
    }

    #[test]
    fn test_register_activation_attr1() {
        let ra = RegisterActivation::new(ObisCode::CLOCK);
        let bytes = ra.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }
}
