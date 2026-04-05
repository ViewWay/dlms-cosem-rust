//! IC63 Status Mapping - Status Bit Mapping

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IC63 Status Mapping - Status Bit Mapping Object
/// 
/// This class maps status bits to meaningful descriptions.
/// Used in monitoring and diagnostic systems.
#[derive(Debug, Clone)]
pub struct StatusMapping {
    logical_name: ObisCode,
    status_word: u32,
    mapping_table: Vec<(u8, String)>, // (bit_position, description)
}

impl StatusMapping {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            status_word: 0,
            mapping_table: Vec::new(),
        }
    }

    pub fn status_word(&self) -> u32 {
        self.status_word
    }

    pub fn set_status_word(&mut self, word: u32) {
        self.status_word = word;
    }

    pub fn mapping_table(&self) -> &[(u8, String)] {
        &self.mapping_table
    }

    pub fn add_mapping(&mut self, bit_position: u8, description: String) {
        self.mapping_table.push((bit_position, description));
    }

    pub fn clear_mappings(&mut self) {
        self.mapping_table.clear();
    }

    pub fn get_bit_status(&self, bit_position: u8) -> bool {
        if bit_position < 32 {
            (self.status_word & (1 << bit_position)) != 0
        } else {
            false
        }
    }

    pub fn set_bit(&mut self, bit_position: u8, value: bool) {
        if bit_position < 32 {
            if value {
                self.status_word |= 1 << bit_position;
            } else {
                self.status_word &= !(1 << bit_position);
            }
        }
    }

    pub fn get_active_descriptions(&self) -> Vec<&str> {
        self.mapping_table
            .iter()
            .filter(|(bit, _)| self.get_bit_status(*bit))
            .map(|(_, desc)| desc.as_str())
            .collect()
    }
}

impl CosemObject for StatusMapping {
    fn class_id(&self) -> u16 {
        63
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        2
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
                // Status word
                let mut bytes = vec![0x06]; // Unsigned32
                bytes.extend_from_slice(&self.status_word.to_be_bytes());
                Some(bytes)
            }
            3 => {
                // Mapping table - array of structures
                let mut bytes = vec![0x01]; // Array
                bytes.push(self.mapping_table.len() as u8);
                for (bit_pos, description) in &self.mapping_table {
                    bytes.push(0x02); // Structure
                    bytes.push(0x02); // 2 elements
                    bytes.push(0x0F);
                    bytes.push(*bit_pos);
                    bytes.push(0x09);
                    bytes.push(description.len() as u8);
                    bytes.extend_from_slice(description.as_bytes());
                }
                Some(bytes)
            }
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                if data.len() >= 5 {
                    self.status_word = u32::from_be_bytes([data[1], data[2], data[3], data[4]]);
                }
                Ok(())
            }
            3 => {
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
    fn test_status_mapping_new() {
        let mapping = StatusMapping::new(ObisCode::new(0, 0, 63, 0, 0, 255));
        assert_eq!(mapping.class_id(), 63);
    }

    #[test]
    fn test_status_mapping_status_word() {
        let mut mapping = StatusMapping::new(ObisCode::new(0, 0, 63, 0, 0, 255));
        mapping.set_status_word(0xDEADBEEF);
        assert_eq!(mapping.status_word(), 0xDEADBEEF);
    }

    #[test]
    fn test_status_mapping_bit_operations() {
        let mut mapping = StatusMapping::new(ObisCode::new(0, 0, 63, 0, 0, 255));
        mapping.set_bit(0, true);
        assert!(mapping.get_bit_status(0));
        mapping.set_bit(0, false);
        assert!(!mapping.get_bit_status(0));
    }

    #[test]
    fn test_status_mapping_multiple_bits() {
        let mut mapping = StatusMapping::new(ObisCode::new(0, 0, 63, 0, 0, 255));
        mapping.set_bit(0, true);
        mapping.set_bit(3, true);
        mapping.set_bit(7, true);
        assert!(mapping.get_bit_status(0));
        assert!(mapping.get_bit_status(3));
        assert!(mapping.get_bit_status(7));
        assert!(!mapping.get_bit_status(1));
    }

    #[test]
    fn test_status_mapping_mapping_table() {
        let mut mapping = StatusMapping::new(ObisCode::new(0, 0, 63, 0, 0, 255));
        mapping.add_mapping(0, "Power OK".to_string());
        mapping.add_mapping(1, "Communication Error".to_string());
        assert_eq!(mapping.mapping_table().len(), 2);
    }

    #[test]
    fn test_status_mapping_active_descriptions() {
        let mut mapping = StatusMapping::new(ObisCode::new(0, 0, 63, 0, 0, 255));
        mapping.add_mapping(0, "Power OK".to_string());
        mapping.add_mapping(1, "Error".to_string());
        mapping.set_bit(0, true);
        let active = mapping.get_active_descriptions();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0], "Power OK");
    }

    #[test]
    fn test_status_mapping_attribute_count() {
        let mapping = StatusMapping::new(ObisCode::new(0, 0, 63, 0, 0, 255));
        assert_eq!(mapping.attribute_count(), 2);
    }

    #[test]
    fn test_status_mapping_method_count() {
        let mapping = StatusMapping::new(ObisCode::new(0, 0, 63, 0, 0, 255));
        assert_eq!(mapping.method_count(), 0);
    }
}
