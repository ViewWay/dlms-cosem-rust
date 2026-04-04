//! IC067 Quality of Service

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// QoS Entry
#[derive(Debug, Clone)]
pub struct QosEntry {
    pub priority: u8,
    pub max_bitrate_up: u32,
    pub max_bitrate_down: u32,
}

/// Quality of Service - network QoS configuration
pub struct QualityOfService {
    logical_name: ObisCode,
    entries: Vec<QosEntry>,
}

impl QualityOfService {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            entries: vec![],
        }
    }

    pub fn entries(&self) -> &[QosEntry] {
        &self.entries
    }

    pub fn add_entry(&mut self, entry: QosEntry) {
        self.entries.push(entry);
    }
}

impl CosemObject for QualityOfService {
    fn class_id(&self) -> u16 {
        200
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        3
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
            2 => {
                let entries: Vec<DlmsData> = self
                    .entries
                    .iter()
                    .map(|e| {
                        DlmsData::Structure(vec![
                            DlmsData::Unsigned(e.priority),
                            DlmsData::DoubleLongUnsigned(e.max_bitrate_up),
                            DlmsData::DoubleLongUnsigned(e.max_bitrate_down),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(entries)))
            }
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
    fn test_qos_new() {
        let qos = QualityOfService::new(ObisCode::CLOCK);
        assert_eq!(qos.class_id(), 200);
    }

    #[test]
    fn test_qos_add_entry() {
        let mut qos = QualityOfService::new(ObisCode::CLOCK);
        qos.add_entry(QosEntry {
            priority: 1,
            max_bitrate_up: 1000,
            max_bitrate_down: 10000,
        });
        assert_eq!(qos.entries().len(), 1);
    }

    #[test]
    fn test_qos_attr1() {
        let qos = QualityOfService::new(ObisCode::CLOCK);
        let bytes = qos.attribute_to_bytes(1).unwrap();
        assert_eq!(bytes.len(), 8);
    }
}
