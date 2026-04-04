//! IC085 Cluster

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Cluster - grouping of COSEM objects
pub struct Cluster {
    logical_name: ObisCode,
    members: Vec<ObisCode>,
}

impl Cluster {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            members: vec![],
        }
    }

    pub fn members(&self) -> &[ObisCode] {
        &self.members
    }

    pub fn add_member(&mut self, member: ObisCode) {
        self.members.push(member);
    }

    pub fn clear_members(&mut self) {
        self.members.clear();
    }
}

impl CosemObject for Cluster {
    fn class_id(&self) -> u16 {
        85
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
                let members: Vec<DlmsData> = self
                    .members
                    .iter()
                    .map(|m| DlmsData::OctetString(m.to_bytes().to_vec()))
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(members)))
            }
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(
                self.members.len() as u8
            ))),
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
    fn test_cluster_new() {
        let c = Cluster::new(ObisCode::CLOCK);
        assert_eq!(c.class_id(), 85);
    }

    #[test]
    fn test_cluster_members() {
        let mut c = Cluster::new(ObisCode::CLOCK);
        c.add_member(ObisCode::CLOCK);
        c.add_member(ObisCode::CLOCK);
        assert_eq!(c.members().len(), 2);
    }

    #[test]
    fn test_cluster_clear() {
        let mut c = Cluster::new(ObisCode::CLOCK);
        c.add_member(ObisCode::CLOCK);
        c.clear_members();
        assert!(c.members().is_empty());
    }

    #[test]
    fn test_cluster_attr1() {
        let c = Cluster::new(ObisCode::CLOCK);
        let bytes = c.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }
}
