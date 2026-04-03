//! CosemAttribute and CosemMethod descriptors

/// COSEM attribute descriptor: (class_id, logical_name, attribute_id)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CosemAttribute {
    pub class_id: u16,
    pub logical_name: crate::ObisCode,
    pub attribute_id: u8,
}

impl CosemAttribute {
    pub fn new(class_id: u16, logical_name: crate::ObisCode, attribute_id: u8) -> Self {
        Self { class_id, logical_name, attribute_id }
    }
}

/// COSEM method descriptor: (class_id, logical_name, method_id)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CosemMethod {
    pub class_id: u16,
    pub logical_name: crate::ObisCode,
    pub method_id: u8,
}

impl CosemMethod {
    pub fn new(class_id: u16, logical_name: crate::ObisCode, method_id: u8) -> Self {
        Self { class_id, logical_name, method_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ObisCode;

    #[test]
    fn test_attribute_new() {
        let attr = CosemAttribute::new(1, ObisCode::CLOCK, 2);
        assert_eq!(attr.class_id, 1);
        assert_eq!(attr.attribute_id, 2);
    }

    #[test]
    fn test_method_new() {
        let method = CosemMethod::new(1, ObisCode::CLOCK, 1);
        assert_eq!(method.method_id, 1);
    }

    #[test]
    fn test_attribute_equality() {
        let a = CosemAttribute::new(1, ObisCode::CLOCK, 2);
        let b = CosemAttribute::new(1, ObisCode::CLOCK, 2);
        assert_eq!(a, b);
    }

    #[test]
    fn test_method_clone() {
        let m = CosemMethod::new(8, ObisCode::new(0, 0, 1, 0, 0, 255), 3);
        let m2 = m.clone();
        assert_eq!(m, m2);
    }
}
