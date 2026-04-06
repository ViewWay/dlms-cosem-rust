//! IC010 Script Table
//!
//! Attributes:
//! 1: logical_name (octet-string)
//! 2: scripts (array of script definitions)
//!
//! Methods:
//! 1: execute
//! 2: execute_with_reply

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

#[derive(Debug, Clone)]
pub struct Script {
    pub script_id: u8,
    pub script_selector: u8,
    pub file_id: u16,
}

pub struct ScriptTable {
    logical_name: ObisCode,
    scripts: Vec<Script>,
}

impl ScriptTable {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            scripts: Vec::new(),
        }
    }

    pub fn scripts(&self) -> &[Script] {
        &self.scripts
    }

    pub fn add_script(&mut self, script: Script) {
        self.scripts.push(script);
    }

    pub fn remove_script(&mut self, script_id: u8) -> Option<Script> {
        if let Some(pos) = self.scripts.iter().position(|s| s.script_id == script_id) {
            Some(self.scripts.remove(pos))
        } else {
            None
        }
    }
}

impl CosemObject for ScriptTable {
    fn class_id(&self) -> u16 {
        9
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        2
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
            2 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.scripts
                    .iter()
                    .map(|s| {
                        DlmsData::Structure(vec![
                            DlmsData::Unsigned(s.script_id),
                            DlmsData::Unsigned(s.script_selector),
                            DlmsData::Long(s.file_id as i16),
                        ])
                    })
                    .collect(),
            ))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, _attr: u8, _data: &[u8]) -> Result<(), CosemObjectError> {
        Err(CosemObjectError::AttributeNotSupported(_attr))
    }

    fn execute_action(&mut self, method_id: u8, _data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        match method_id {
            1 => {
                // execute script
                Ok(vec![0x00, 0x00]) // success
            }
            2 => {
                // execute script with reply
                Ok(vec![0x00, 0x00]) // success
            }
            _ => Err(CosemObjectError::MethodNotSupported(method_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_table_class_id() {
        let st = ScriptTable::new(ObisCode::new(0, 0, 10, 0, 0, 255));
        assert_eq!(st.class_id(), 9);
    }

    #[test]
    fn test_script_table_attribute_count() {
        let st = ScriptTable::new(ObisCode::new(0, 0, 10, 0, 0, 255));
        assert_eq!(st.attribute_count(), 2);
    }

    #[test]
    fn test_script_table_method_count() {
        let st = ScriptTable::new(ObisCode::new(0, 0, 10, 0, 0, 255));
        assert_eq!(st.method_count(), 2);
    }

    #[test]
    fn test_script_table_add_script() {
        let mut st = ScriptTable::new(ObisCode::new(0, 0, 10, 0, 0, 255));
        st.add_script(Script {
            script_id: 1,
            script_selector: 0,
            file_id: 100,
        });
        assert_eq!(st.scripts().len(), 1);
    }

    #[test]
    fn test_script_table_remove_script() {
        let mut st = ScriptTable::new(ObisCode::new(0, 0, 10, 0, 0, 255));
        st.add_script(Script {
            script_id: 1,
            script_selector: 0,
            file_id: 100,
        });
        let removed = st.remove_script(1);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().script_id, 1);
        assert_eq!(st.scripts().len(), 0);
    }

    #[test]
    fn test_script_table_attr1() {
        let st = ScriptTable::new(ObisCode::new(0, 0, 10, 0, 0, 255));
        let bytes = st.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn test_script_table_attr2() {
        let mut st = ScriptTable::new(ObisCode::new(0, 0, 10, 0, 0, 255));
        st.add_script(Script {
            script_id: 1,
            script_selector: 0,
            file_id: 100,
        });
        let bytes = st.attribute_to_bytes(2).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_script_table_execute_method() {
        let mut st = ScriptTable::new(ObisCode::new(0, 0, 10, 0, 0, 255));
        let result = st.execute_action(1, &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_script_table_execute_with_reply_method() {
        let mut st = ScriptTable::new(ObisCode::new(0, 0, 10, 0, 0, 255));
        let result = st.execute_action(2, &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_script_table_unsupported_method() {
        let mut st = ScriptTable::new(ObisCode::new(0, 0, 10, 0, 0, 255));
        let result = st.execute_action(99, &[]);
        assert!(result.is_err());
    }
}
