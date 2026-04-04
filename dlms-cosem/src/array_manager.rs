//! IC123 Array Manager
//! Blue Book Ed16: class_id=123, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Array Manager - manages dynamic arrays of objects
pub struct ArrayManager {
    logical_name: ObisCode,
    managed_object_list: Vec<ObisCode>,
    allocation_size: u32,
}

impl ArrayManager {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            managed_object_list: vec![],
            allocation_size: 100,
        }
    }
    pub fn managed_objects(&self) -> &[ObisCode] {
        &self.managed_object_list
    }
    pub fn add_object(&mut self, obis: ObisCode) {
        self.managed_object_list.push(obis);
    }
    pub fn remove_object(&mut self, index: usize) -> Option<ObisCode> {
        if index < self.managed_object_list.len() {
            Some(self.managed_object_list.remove(index))
        } else {
            None
        }
    }
    pub fn clear(&mut self) {
        self.managed_object_list.clear();
    }
}

impl CosemObject for ArrayManager {
    fn class_id(&self) -> u16 {
        123
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        4
    }
    fn method_count(&self) -> u8 {
        3
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let n = self.logical_name.to_bytes();
                Some(vec![0x09, 0x06, n[0], n[1], n[2], n[3], n[4], n[5]])
            }
            2 => {
                let list: Vec<DlmsData> = self
                    .managed_object_list
                    .iter()
                    .map(|o| DlmsData::OctetString(o.to_bytes().to_vec()))
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(list)))
            }
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.allocation_size,
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, _attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        Err(CosemObjectError::AttributeNotSupported(_attr))
    }

    fn execute_action(&mut self, method_id: u8, _data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        match method_id {
            1 | 2 | 3 => Ok(vec![]),
            _ => Err(CosemObjectError::MethodNotSupported(method_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_am_new() {
        let a = ArrayManager::new(ObisCode::CLOCK);
        assert_eq!(a.class_id(), 123);
    }
    #[test]
    fn test_am_add_remove() {
        let mut a = ArrayManager::new(ObisCode::CLOCK);
        a.add_object(ObisCode::CLOCK);
        a.remove_object(0);
        assert!(a.managed_objects().is_empty());
    }
    #[test]
    fn test_am_clear() {
        let mut a = ArrayManager::new(ObisCode::CLOCK);
        a.add_object(ObisCode::CLOCK);
        a.clear();
        assert!(a.managed_objects().is_empty());
    }
    #[test]
    fn test_am_methods() {
        let mut a = ArrayManager::new(ObisCode::CLOCK);
        assert!(a.execute_action(1, &[]).is_ok());
        assert!(a.execute_action(4, &[]).is_err());
    }
}
