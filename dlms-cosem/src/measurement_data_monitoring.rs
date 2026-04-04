//! IC66 Measurement Data Monitoring
//! Blue Book Ed16: class_id=66, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Measurement Data Monitoring - trigger-based measurement capture
pub struct MeasurementDataMonitoring {
    logical_name: ObisCode,
    status: u8,
    trigger_source: u8,
    sampling_rate: u32,
    samples_before_trigger: u16,
    samples_after_trigger: u16,
    trigger_time: DlmsData,
}

impl MeasurementDataMonitoring {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            status: 0,
            trigger_source: 0,
            sampling_rate: 1000,
            samples_before_trigger: 10,
            samples_after_trigger: 10,
            trigger_time: DlmsData::OctetString(vec![0xFF; 12]),
        }
    }

    pub fn status(&self) -> u8 {
        self.status
    }
    pub fn set_status(&mut self, s: u8) {
        self.status = s;
    }
    pub fn sampling_rate(&self) -> u32 {
        self.sampling_rate
    }
    pub fn set_sampling_rate(&mut self, r: u32) {
        self.sampling_rate = r;
    }
}

impl CosemObject for MeasurementDataMonitoring {
    fn class_id(&self) -> u16 {
        66
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        9
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
            2 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.status))),
            3 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.trigger_source))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(
                self.sampling_rate,
            ))),
            5 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.samples_before_trigger,
            ))),
            6 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.samples_after_trigger,
            ))),
            7 => Some(dlms_axdr::encode(&self.trigger_time)),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = d.as_u8() {
                    self.status = v;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            4 => {
                let d = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = d.as_i32() {
                    self.sampling_rate = v as u32;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }

    fn execute_action(&mut self, method_id: u8, _data: &[u8]) -> Result<Vec<u8>, CosemObjectError> {
        match method_id {
            1 | 2 => Ok(vec![]),
            _ => Err(CosemObjectError::MethodNotSupported(method_id)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdm_new() {
        let mdm = MeasurementDataMonitoring::new(ObisCode::CLOCK);
        assert_eq!(mdm.class_id(), 66);
    }

    #[test]
    fn test_mdm_sampling_rate() {
        let mut mdm = MeasurementDataMonitoring::new(ObisCode::CLOCK);
        mdm.set_sampling_rate(500);
        assert_eq!(mdm.sampling_rate(), 500);
    }

    #[test]
    fn test_mdm_roundtrip() {
        let mut mdm = MeasurementDataMonitoring::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Unsigned(3));
        mdm.attribute_from_bytes(2, &bytes).unwrap();
        assert_eq!(mdm.status(), 3);
    }
}
