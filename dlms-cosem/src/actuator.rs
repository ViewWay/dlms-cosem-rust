//! IC075 Actuator

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Actuator state
#[derive(Debug, Clone, PartialEq)]
pub enum ActuatorState {
    Off,
    On,
    Fault,
}

/// Actuator - generic actuator control
pub struct Actuator {
    logical_name: ObisCode,
    state: ActuatorState,
    value: DlmsData,
}

impl Actuator {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            state: ActuatorState::Off,
            value: DlmsData::DoubleLong(0),
        }
    }

    pub fn state(&self) -> &ActuatorState {
        &self.state
    }

    pub fn set_state(&mut self, state: ActuatorState) {
        self.state = state;
    }

    pub fn value(&self) -> &DlmsData {
        &self.value
    }

    pub fn set_value(&mut self, value: DlmsData) {
        self.value = value;
    }
}

impl CosemObject for Actuator {
    fn class_id(&self) -> u16 {
        75
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        3
    }
    fn method_count(&self) -> u8 {
        1
    }

    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => {
                let name = self.logical_name.to_bytes();
                Some(vec![
                    0x09, 0x06, name[0], name[1], name[2], name[3], name[4], name[5],
                ])
            }
            2 => Some(dlms_axdr::encode(&DlmsData::Enum(match self.state {
                ActuatorState::Off => 0,
                ActuatorState::On => 1,
                ActuatorState::Fault => 2,
            }))),
            3 => Some(dlms_axdr::encode(&self.value)),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_u8() {
                    self.state = match v {
                        0 => ActuatorState::Off,
                        1 => ActuatorState::On,
                        _ => ActuatorState::Fault,
                    };
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            3 => {
                self.value = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                Ok(())
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }

    fn execute_action(&mut self, method_id: u8, data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        if method_id == 1 {
            let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
            if let Some(v) = decoded.as_u8() {
                self.state = match v {
                    0 => ActuatorState::Off,
                    1 => ActuatorState::On,
                    _ => ActuatorState::Fault,
                };
                Ok(vec![])
            } else {
                Err(CosemObjectError::InvalidData)
            }
        } else {
            Err(CosemObjectError::MethodNotSupported(method_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actuator_new() {
        let a = Actuator::new(ObisCode::CLOCK);
        assert_eq!(a.class_id(), 75);
        assert_eq!(*a.state(), ActuatorState::Off);
    }

    #[test]
    fn test_actuator_set_state() {
        let mut a = Actuator::new(ObisCode::CLOCK);
        a.set_state(ActuatorState::On);
        assert_eq!(*a.state(), ActuatorState::On);
    }

    #[test]
    fn test_actuator_roundtrip() {
        let mut a = Actuator::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Unsigned(1));
        a.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(*a.state(), ActuatorState::On);
    }

    #[test]
    fn test_actuator_execute_action() {
        let mut a = Actuator::new(ObisCode::CLOCK);
        let data = dlms_axdr::encode(&DlmsData::Unsigned(1));
        a.execute_action(1, &data).unwrap();
        assert_eq!(*a.state(), ActuatorState::On);
    }

    #[test]
    fn test_actuator_invalid_method() {
        let mut a = Actuator::new(ObisCode::CLOCK);
        assert!(a.execute_action(99, &[]).is_err());
    }
}
