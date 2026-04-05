//! IC23 Standard Readout - Standard Data Readout

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// IC23 Standard Readout - Standard Data Readout Object
/// 
/// This class provides a standard mechanism for reading multiple data values.
/// Used for bulk data retrieval and meter reading operations.
#[derive(Debug, Clone)]
pub struct StandardReadout {
    logical_name: ObisCode,
    capture_objects: Vec<(u16, ObisCode, u8)>, // (class_id, obis, attribute_index)
    capture_period: u32,
    last_capture_time: Option<u32>,
    buffer: Vec<Vec<u8>>,
}

impl StandardReadout {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            capture_objects: Vec::new(),
            capture_period: 0,
            last_capture_time: None,
            buffer: Vec::new(),
        }
    }

    pub fn capture_objects(&self) -> &[(u16, ObisCode, u8)] {
        &self.capture_objects
    }

    pub fn add_capture_object(&mut self, class_id: u16, obis: ObisCode, attr_index: u8) {
        self.capture_objects.push((class_id, obis, attr_index));
    }

    pub fn clear_capture_objects(&mut self) {
        self.capture_objects.clear();
    }

    pub fn capture_period(&self) -> u32 {
        self.capture_period
    }

    pub fn set_capture_period(&mut self, period: u32) {
        self.capture_period = period;
    }

    pub fn last_capture_time(&self) -> Option<u32> {
        self.last_capture_time
    }

    pub fn set_last_capture_time(&mut self, time: u32) {
        self.last_capture_time = Some(time);
    }

    pub fn buffer(&self) -> &[Vec<u8>] {
        &self.buffer
    }

    pub fn add_to_buffer(&mut self, data: Vec<u8>) {
        self.buffer.push(data);
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }
}

impl CosemObject for StandardReadout {
    fn class_id(&self) -> u16 {
        23
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        4
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
            2 => {
                // Capture objects - array of structures
                let mut bytes = vec![0x01]; // Array
                bytes.push(self.capture_objects.len() as u8);
                for (class_id, obis, attr_idx) in &self.capture_objects {
                    bytes.push(0x02); // Structure
                    bytes.push(0x03); // 3 elements
                    bytes.extend_from_slice(&(*class_id as u32).to_be_bytes()[2..]);
                    bytes.push(0x09);
                    bytes.push(0x06);
                    bytes.extend_from_slice(&obis.to_bytes());
                    bytes.push(0x0F);
                    bytes.push(*attr_idx);
                }
                Some(bytes)
            }
            3 => {
                // Capture period
                let mut bytes = vec![0x06]; // Unsigned32
                bytes.extend_from_slice(&self.capture_period.to_be_bytes());
                Some(bytes)
            }
            4 => {
                // Buffer - array of octet strings
                let mut bytes = vec![0x01]; // Array
                bytes.extend_from_slice(&(self.buffer.len() as u16).to_be_bytes());
                for data in &self.buffer {
                    bytes.push(0x09);
                    bytes.push(data.len() as u8);
                    bytes.extend_from_slice(data);
                }
                Some(bytes)
            }
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 | 3 | 4 => {
                let _ = data;
                Ok(())
            }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_readout_new() {
        let readout = StandardReadout::new(ObisCode::new(0, 0, 23, 0, 0, 255));
        assert_eq!(readout.class_id(), 23);
    }

    #[test]
    fn test_standard_readout_capture_objects() {
        let mut readout = StandardReadout::new(ObisCode::new(0, 0, 23, 0, 0, 255));
        readout.add_capture_object(1, ObisCode::DATA, 2);
        readout.add_capture_object(3, ObisCode::ACTIVE_POWER_L1, 2);
        assert_eq!(readout.capture_objects().len(), 2);
    }

    #[test]
    fn test_standard_readout_capture_period() {
        let mut readout = StandardReadout::new(ObisCode::new(0, 0, 23, 0, 0, 255));
        readout.set_capture_period(900);
        assert_eq!(readout.capture_period(), 900);
    }

    #[test]
    fn test_standard_readout_buffer() {
        let mut readout = StandardReadout::new(ObisCode::new(0, 0, 23, 0, 0, 255));
        readout.add_to_buffer(vec![0x01, 0x02, 0x03]);
        readout.add_to_buffer(vec![0x04, 0x05, 0x06]);
        assert_eq!(readout.buffer_size(), 2);
    }

    #[test]
    fn test_standard_readout_clear_buffer() {
        let mut readout = StandardReadout::new(ObisCode::new(0, 0, 23, 0, 0, 255));
        readout.add_to_buffer(vec![0x01, 0x02, 0x03]);
        readout.clear_buffer();
        assert_eq!(readout.buffer_size(), 0);
    }

    #[test]
    fn test_standard_readout_attribute_count() {
        let readout = StandardReadout::new(ObisCode::new(0, 0, 23, 0, 0, 255));
        assert_eq!(readout.attribute_count(), 4);
    }

    #[test]
    fn test_standard_readout_method_count() {
        let readout = StandardReadout::new(ObisCode::new(0, 0, 23, 0, 0, 255));
        assert_eq!(readout.method_count(), 2);
    }

    #[test]
    fn test_standard_readout_last_capture_time() {
        let mut readout = StandardReadout::new(ObisCode::new(0, 0, 23, 0, 0, 255));
        assert!(readout.last_capture_time().is_none());
        readout.set_last_capture_time(12345678);
        assert_eq!(readout.last_capture_time(), Some(12345678));
    }
}
