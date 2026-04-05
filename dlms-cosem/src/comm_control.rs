//! Communication Control - Manages communication settings
//!
//! Controls communication parameters, modes, and connectivity
//! for metering devices.

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Communication channel configuration
#[derive(Debug, Clone)]
pub struct CommChannel {
    pub channel_id: u8,
    pub channel_type: u8,
    pub enabled: bool,
    pub priority: u8,
}

/// IC206 Communication Control - Manages communication
pub struct CommControl {
    logical_name: ObisCode,
    channels: Vec<CommChannel>,
    enabled: bool,
    auto_connect: bool,
}

impl CommControl {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            channels: Vec::new(),
            enabled: true,
            auto_connect: true,
        }
    }

    pub fn add_channel(&mut self, channel_id: u8, channel_type: u8) {
        self.channels.push(CommChannel {
            channel_id,
            channel_type,
            enabled: true,
            priority: 0,
        });
    }

    pub fn enable_channel(&mut self, channel_id: u8) -> bool {
        if let Some(ch) = self.channels.iter_mut().find(|c| c.channel_id == channel_id) {
            ch.enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_channel(&mut self, channel_id: u8) -> bool {
        if let Some(ch) = self.channels.iter_mut().find(|c| c.channel_id == channel_id) {
            ch.enabled = false;
            true
        } else {
            false
        }
    }

    pub fn get_channel(&self, channel_id: u8) -> Option<&CommChannel> {
        self.channels.iter().find(|c| c.channel_id == channel_id)
    }

    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn auto_connect(&self) -> bool {
        self.auto_connect
    }

    pub fn set_auto_connect(&mut self, auto: bool) {
        self.auto_connect = auto;
    }
}

impl CosemObject for CommControl {
    fn class_id(&self) -> u16 {
        206
    }

    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }

    fn attribute_count(&self) -> u8 {
        6
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
                let channels: Vec<DlmsData> = self
                    .channels
                    .iter()
                    .map(|c| {
                        DlmsData::Structure(vec![
                            DlmsData::Unsigned(c.channel_id),
                            DlmsData::Unsigned(c.channel_type),
                            DlmsData::Boolean(c.enabled),
                            DlmsData::Unsigned(c.priority),
                        ])
                    })
                    .collect();
                Some(dlms_axdr::encode(&DlmsData::Array(channels)))
            }
            3 => Some(dlms_axdr::encode(&DlmsData::LongUnsigned(
                self.channels.len() as u16,
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.enabled))),
            5 => Some(dlms_axdr::encode(&DlmsData::Boolean(self.auto_connect))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Boolean(v) = decoded {
                    self.enabled = v;
                    Ok(())
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            5 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let DlmsData::Boolean(v) = decoded {
                    self.auto_connect = v;
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
    fn test_comm_control_class_id() {
        let cc = CommControl::new(ObisCode::new(0, 0, 206, 0, 0, 255));
        assert_eq!(cc.class_id(), 206);
    }

    #[test]
    fn test_comm_control_new() {
        let cc = CommControl::new(ObisCode::new(0, 0, 206, 0, 0, 255));
        assert_eq!(cc.channel_count(), 0);
        assert!(cc.is_enabled());
        assert!(cc.auto_connect());
    }

    #[test]
    fn test_comm_control_add_channel() {
        let mut cc = CommControl::new(ObisCode::new(0, 0, 206, 0, 0, 255));
        cc.add_channel(1, 1);
        assert_eq!(cc.channel_count(), 1);
    }

    #[test]
    fn test_comm_control_enable_disable_channel() {
        let mut cc = CommControl::new(ObisCode::new(0, 0, 206, 0, 0, 255));
        cc.add_channel(1, 1);
        assert!(cc.disable_channel(1));
        assert!(!cc.get_channel(1).unwrap().enabled);
        assert!(cc.enable_channel(1));
        assert!(cc.get_channel(1).unwrap().enabled);
    }

    #[test]
    fn test_comm_control_enabled() {
        let mut cc = CommControl::new(ObisCode::new(0, 0, 206, 0, 0, 255));
        cc.set_enabled(false);
        assert!(!cc.is_enabled());
        cc.set_enabled(true);
        assert!(cc.is_enabled());
    }

    #[test]
    fn test_comm_control_auto_connect() {
        let mut cc = CommControl::new(ObisCode::new(0, 0, 206, 0, 0, 255));
        cc.set_auto_connect(false);
        assert!(!cc.auto_connect());
        cc.set_auto_connect(true);
        assert!(cc.auto_connect());
    }

    #[test]
    fn test_comm_control_multiple_channels() {
        let mut cc = CommControl::new(ObisCode::new(0, 0, 206, 0, 0, 255));
        cc.add_channel(1, 1);
        cc.add_channel(2, 2);
        cc.add_channel(3, 3);
        assert_eq!(cc.channel_count(), 3);
    }

    #[test]
    fn test_comm_control_attribute_count() {
        let cc = CommControl::new(ObisCode::new(0, 0, 206, 0, 0, 255));
        assert_eq!(cc.attribute_count(), 6);
    }

    #[test]
    fn test_comm_control_method_count() {
        let cc = CommControl::new(ObisCode::new(0, 0, 206, 0, 0, 255));
        assert_eq!(cc.method_count(), 2);
    }
}
