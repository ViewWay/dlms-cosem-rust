//! CosemObject trait - Base trait for all COSEM interface classes

use crate::ObisCode;

/// COSEM Interface Object trait
/// All COSEM interface classes (IC) implement this trait.
pub trait CosemObject {
    /// COSEM interface class ID (e.g., 1 for Clock, 3 for Register)
    fn class_id(&self) -> u16;

    /// Logical name (OBIS code) of this instance
    fn logical_name(&self) -> ObisCode;

    /// Number of attributes supported
    fn attribute_count(&self) -> u8;

    /// Number of methods supported
    fn method_count(&self) -> u8;

    /// Serialize this object's data to bytes (for a given attribute)
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>>;

    /// Deserialize data for a given attribute
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CosemObjectError {
    AttributeNotSupported(u8),
    MethodNotSupported(u8),
    InvalidData,
    AccessDenied,
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestObject {
        name: ObisCode,
        value: u32,
    }

    impl CosemObject for TestObject {
        fn class_id(&self) -> u16 {
            1
        }
        fn logical_name(&self) -> ObisCode {
            self.name
        }
        fn attribute_count(&self) -> u8 {
            3
        }
        fn method_count(&self) -> u8 {
            1
        }
        fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
            match attr {
                1 => Some(self.value.to_be_bytes().to_vec()),
                _ => None,
            }
        }
        fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
            if attr == 1 && data.len() == 4 {
                self.value = u32::from_be_bytes(data.try_into().unwrap());
                Ok(())
            } else {
                Err(CosemObjectError::AttributeNotSupported(attr))
            }
        }
    }

    #[test]
    fn test_cosem_object_trait() {
        let obj = TestObject {
            name: ObisCode::CLOCK,
            value: 42,
        };
        assert_eq!(obj.class_id(), 1);
        assert_eq!(obj.logical_name(), ObisCode::CLOCK);
    }

    #[test]
    fn test_attribute_roundtrip() {
        let mut obj = TestObject {
            name: ObisCode::CLOCK,
            value: 42,
        };
        let bytes = obj.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes, vec![0, 0, 0, 42]);
        obj.attribute_from_bytes(1, &bytes).unwrap();
        assert_eq!(obj.value, 42);
    }
}
