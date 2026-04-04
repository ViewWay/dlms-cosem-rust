//! IC011 Register Monitor

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Thresholds for register monitoring
#[derive(Debug, Clone)]
pub struct Thresholds {
    pub thresholds: Vec<DlmsData>,
}

/// Register Monitor - monitors register values against thresholds
pub struct RegisterMonitor {
    logical_name: ObisCode,
    thresholds: Thresholds,
    actions: Vec<DlmsData>,
}

impl RegisterMonitor {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            thresholds: Thresholds { thresholds: vec![] },
            actions: vec![],
        }
    }

    pub fn with_thresholds(logical_name: ObisCode, thresholds: Vec<DlmsData>) -> Self {
        Self {
            logical_name,
            thresholds: Thresholds { thresholds },
            actions: vec![],
        }
    }

    pub fn thresholds(&self) -> &[DlmsData] {
        &self.thresholds.thresholds
    }

    pub fn set_thresholds(&mut self, thresholds: Vec<DlmsData>) {
        self.thresholds.thresholds = thresholds;
    }

    pub fn actions(&self) -> &[DlmsData] {
        &self.actions
    }

    pub fn add_action(&mut self, action: DlmsData) {
        self.actions.push(action);
    }
}

impl CosemObject for RegisterMonitor {
    fn class_id(&self) -> u16 {
        11
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        4
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
            2 => Some(dlms_axdr::encode(&DlmsData::Array(
                self.thresholds.thresholds.clone(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::Array(self.actions.clone()))),
            4 => Some(dlms_axdr::encode(&DlmsData::Structure(vec![
                DlmsData::Unsigned(self.thresholds.thresholds.len() as u8),
            ]))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(arr) = decoded.as_array() {
                    self.thresholds.thresholds = arr.to_vec();
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(arr) = decoded.as_array() {
                    self.actions = arr.to_vec();
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_monitor_new() {
        let rm = RegisterMonitor::new(ObisCode::CLOCK);
        assert_eq!(rm.class_id(), 11);
    }

    #[test]
    fn test_register_monitor_thresholds() {
        let rm = RegisterMonitor::with_thresholds(
            ObisCode::CLOCK,
            vec![DlmsData::DoubleLong(100), DlmsData::DoubleLong(200)],
        );
        assert_eq!(rm.thresholds().len(), 2);
    }

    #[test]
    fn test_register_monitor_actions() {
        let mut rm = RegisterMonitor::new(ObisCode::CLOCK);
        rm.add_action(DlmsData::Unsigned(1));
        assert_eq!(rm.actions().len(), 1);
    }

    #[test]
    fn test_register_monitor_roundtrip() {
        let mut rm = RegisterMonitor::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Array(vec![DlmsData::DoubleLong(100)]));
        rm.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(rm.thresholds().len(), 1);
    }

    #[test]
    fn test_register_monitor_attr1() {
        let rm = RegisterMonitor::new(ObisCode::CLOCK);
        let bytes = rm.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }
}
