//! IC081 Gas Valve

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Gas Valve state
#[derive(Debug, Clone, PartialEq)]
pub enum GasValveState {
    Closed,
    Open,
    Fault,
}

/// Gas Valve - gas valve control
pub struct GasValve {
    logical_name: ObisCode,
    state: GasValveState,
}

impl GasValve {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            state: GasValveState::Closed,
        }
    }

    pub fn state(&self) -> &GasValveState {
        &self.state
    }

    pub fn set_state(&mut self, state: GasValveState) {
        self.state = state;
    }

    pub fn is_open(&self) -> bool {
        self.state == GasValveState::Open
    }
}

impl CosemObject for GasValve {
    fn class_id(&self) -> u16 {
        81
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        2
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
                GasValveState::Closed => 0,
                GasValveState::Open => 1,
                GasValveState::Fault => 2,
            }))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_u8() {
                    self.state = match v {
                        0 => GasValveState::Closed,
                        1 => GasValveState::Open,
                        _ => GasValveState::Fault,
                    };
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }

    fn execute_action(&mut self, method_id: u8, data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        if method_id == 1 {
            let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
            if let Some(v) = decoded.as_u8() {
                self.state = match v {
                    0 => GasValveState::Closed,
                    1 => GasValveState::Open,
                    _ => GasValveState::Fault,
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
    fn test_gas_valve_new() {
        let gv = GasValve::new(ObisCode::CLOCK);
        assert_eq!(gv.class_id(), 81);
        assert!(!gv.is_open());
    }

    #[test]
    fn test_gas_valve_open() {
        let mut gv = GasValve::new(ObisCode::CLOCK);
        gv.set_state(GasValveState::Open);
        assert!(gv.is_open());
    }

    #[test]
    fn test_gas_valve_execute() {
        let mut gv = GasValve::new(ObisCode::CLOCK);
        let data = dlms_axdr::encode(&DlmsData::Unsigned(1));
        gv.execute_action(1, &data).unwrap();
        assert!(gv.is_open());
    }
}
