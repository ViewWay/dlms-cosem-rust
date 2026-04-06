//! IC65 Parameter Monitor
//! Blue Book Ed16: class_id=65, version=1

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Parameter Monitor - monitors parameters with capture capability
pub struct ParameterMonitor {
    logical_name: ObisCode,
    captured_value: Option<DlmsData>,
    captured_time: Option<DlmsData>,
    status: u8,
}

impl ParameterMonitor {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            captured_value: None,
            captured_time: None,
            status: 0,
        }
    }

    pub fn captured_value(&self) -> Option<&DlmsData> {
        self.captured_value.as_ref()
    }
    pub fn status(&self) -> u8 {
        self.status
    }
    pub fn set_status(&mut self, status: u8) {
        self.status = status;
    }

    pub fn capture(&mut self, value: DlmsData, time: DlmsData) {
        self.captured_value = Some(value);
        self.captured_time = Some(time);
    }
}

impl CosemObject for ParameterMonitor {
    fn class_id(&self) -> u16 {
        65
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        5
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
            2 => self.captured_value.as_ref().map(|v| dlms_axdr::encode(v)),
            3 => self.captured_time.as_ref().map(|t| dlms_axdr::encode(t)),
            4 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.status))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                self.captured_value =
                    Some(dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?);
                Ok(())
            }
            4 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = d.as_u8() {
                    self.status = v;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }

    fn execute_action(&mut self, method_id: u8, _data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        if method_id == 1 {
            self.captured_value = None;
            self.captured_time = None;
            Ok(vec![])
        } else {
            Err(CosemObjectError::MethodNotSupported(method_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_monitor_new() {
        let pm = ParameterMonitor::new(ObisCode::CLOCK);
        assert_eq!(pm.class_id(), 65);
    }

    #[test]
    fn test_parameter_monitor_capture() {
        let mut pm = ParameterMonitor::new(ObisCode::CLOCK);
        pm.capture(DlmsData::DoubleLong(42), DlmsData::OctetString(vec![0; 12]));
        assert!(pm.captured_value().is_some());
    }

    #[test]
    fn test_parameter_monitor_reset() {
        let mut pm = ParameterMonitor::new(ObisCode::CLOCK);
        pm.capture(DlmsData::DoubleLong(42), DlmsData::OctetString(vec![0; 12]));
        pm.execute_action(1, &[]).unwrap();
        assert!(pm.captured_value().is_none());
    }
}
