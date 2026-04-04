//! IC086 Route

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// Route - network routing entry
pub struct Route {
    logical_name: ObisCode,
    destination: [u8; 6],
    next_hop: [u8; 6],
    metric: u8,
}

impl Route {
    pub fn new(logical_name: ObisCode) -> Self {
        Self {
            logical_name,
            destination: [0x00; 6],
            next_hop: [0x00; 6],
            metric: 0,
        }
    }

    pub fn destination(&self) -> &[u8; 6] {
        &self.destination
    }

    pub fn set_destination(&mut self, dest: [u8; 6]) {
        self.destination = dest;
    }

    pub fn next_hop(&self) -> &[u8; 6] {
        &self.next_hop
    }

    pub fn set_next_hop(&mut self, hop: [u8; 6]) {
        self.next_hop = hop;
    }

    pub fn metric(&self) -> u8 {
        self.metric
    }

    pub fn set_metric(&mut self, metric: u8) {
        self.metric = metric;
    }
}

impl CosemObject for Route {
    fn class_id(&self) -> u16 {
        219
    }
    fn logical_name(&self) -> ObisCode {
        self.logical_name
    }
    fn attribute_count(&self) -> u8 {
        5
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
            2 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.destination.to_vec(),
            ))),
            3 => Some(dlms_axdr::encode(&DlmsData::OctetString(
                self.next_hop.to_vec(),
            ))),
            4 => Some(dlms_axdr::encode(&DlmsData::Unsigned(self.metric))),
            _ => None,
        }
    }

    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 | 3 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(bytes) = decoded.as_octet_string() {
                    if bytes.len() == 6 {
                        let arr: [u8; 6] = bytes.try_into().unwrap();
                        if attr == 2 {
                            self.destination = arr;
                        } else {
                            self.next_hop = arr;
                        }
                        Ok(())
                    } else {
                        Err(CosemObjectError::InvalidData)
                    }
                } else {
                    Err(CosemObjectError::InvalidData)
                }
            }
            4 => {
                let decoded = dlms_axdr::decode(data).map_err(|_| CosemObjectError::InvalidData)?;
                if let Some(v) = decoded.as_u8() {
                    self.metric = v;
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
    fn test_route_new() {
        let r = Route::new(ObisCode::CLOCK);
        assert_eq!(r.class_id(), 219);
    }

    #[test]
    fn test_route_destination() {
        let mut r = Route::new(ObisCode::CLOCK);
        r.set_destination([1, 2, 3, 4, 5, 6]);
        assert_eq!(r.destination(), &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_route_roundtrip() {
        let mut r = Route::new(ObisCode::CLOCK);
        let bytes = dlms_axdr::encode(&DlmsData::Unsigned(10));
        r.attribute_from_bytes(4, &bytes).unwrap();
        assert_eq!(r.metric(), 10);
    }
}
