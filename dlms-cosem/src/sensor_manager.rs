//! IC67 Sensor Manager
//! Blue Book Ed16: class_id=67, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Sensor entry in the sensor list
#[derive(Debug, Clone)]
pub struct SensorEntry {
    pub sensor_reference: ObisCode,
    pub sensor_type: u8,
}

/// Sensor Manager - manages sensor devices
pub struct SensorManager {
    logical_name: ObisCode,
    sensor_list: Vec<SensorEntry>,
}

impl SensorManager {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            sensor_list: vec![],
        }
    }

    pub fn sensor_list(&self) -> &[SensorEntry] {
        &self.sensor_list
    }

    pub fn add_sensor(&mut self, entry: SensorEntry) {
        self.sensor_list.push(entry);
    }
    pub fn remove_sensor(&mut self, index: usize) -> Option<SensorEntry> {
        if index < self.sensor_list.len() {
            Some(self.sensor_list.remove(index))
        } else {
            None
        }
    }
}

impl CosemObject for SensorManager {
    fn class_id(&self) -> u16 {
        67
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        2
    }
    fn method_count(&self) -> u8 {
        3
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
                let list: Vec<DlmsData> = self
                    .sensor_list
                    .iter()
                    .map(|s| {
                        DlmsData::Structure(vec![
                            DlmsData::OctetString(s.sensor_reference.to_bytes().to_vec()),
                            DlmsData::Unsigned(s.sensor_type),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(list)))
            }
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
    fn test_sensor_manager_new() {
        let sm = SensorManager::new(ObisCode::CLOCK);
        assert_eq!(sm.class_id(), 67);
    }

    #[test]
    fn test_sensor_manager_add_remove() {
        let mut sm = SensorManager::new(ObisCode::CLOCK);
        sm.add_sensor(SensorEntry {
            sensor_reference: ObisCode::CLOCK,
            sensor_type: 1,
        });
        assert_eq!(sm.sensor_list().len(), 1);
        sm.remove_sensor(0);
        assert!(sm.sensor_list().is_empty());
    }

    #[test]
    fn test_sensor_manager_methods() {
        let mut sm = SensorManager::new(ObisCode::CLOCK);
        assert!(sm.execute_action(1, &[]).is_ok());
        assert!(sm.execute_action(4, &[]).is_err());
    }
}
