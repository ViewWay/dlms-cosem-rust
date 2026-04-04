//! IC122 Function Control
//! Blue Book Ed16: class_id=122, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Function Control - manages enabled functions
pub struct FunctionControl {
    logical_name: ObisCode,
    function_list: Vec<DlmsData>,
}

impl FunctionControl {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            function_list: vec![],
        }
    }
    pub fn function_list(&self) -> &[DlmsData] {
        &self.function_list
    }
    pub fn add_function(&mut self, f: DlmsData) {
        self.function_list.push(f);
    }
}

impl CosemObject for FunctionControl {
    fn class_id(&self) -> u16 {
        122
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
                let n = self.logical_name.to_bytes();
                Some(vec![0x09, 0x06, n[0], n[1], n[2], n[3], n[4], n[5]])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.function_list.clone(),
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
    fn test_fc_new() {
        let f = FunctionControl::new(ObisCode::CLOCK);
        assert_eq!(f.class_id(), 122);
    }
    #[test]
    fn test_fc_add() {
        let mut f = FunctionControl::new(ObisCode::CLOCK);
        f.add_function(DlmsData::Unsigned(1));
        assert_eq!(f.function_list().len(), 1);
    }
}
