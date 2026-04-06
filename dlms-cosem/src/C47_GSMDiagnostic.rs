//! IC47 GsmDiagnostic
//! Blue Book Ed16: class_id=47, version=0

use dlms_core::{CosemObject, CosemObjectError, DlmsData, ObisCode};

/// GsmDiagnostic
pub struct GsmDiagnostic {
    logical_name: ObisCode,
    operator: String,
    status: u32,
    cell_id: u32,
    signal_strength: u32,
}

impl GsmDiagnostic {
    pub fn new(logical_name: ObisCode) -> Self {
        Self { logical_name,
            operator: String::new(),
            status: 0,
            cell_id: 0,
            signal_strength: 0,
        }
    }
    pub fn operator(&self) -> &str { &self.operator }
    pub fn set_operator(&mut self, v: impl Into<String>) { self.operator = v.into(); }
    pub fn status(&self) -> u32 { self.status }
    pub fn set_status(&mut self, v: u32) { self.status = v; }
    pub fn cell_id(&self) -> u32 { self.cell_id }
    pub fn set_cell_id(&mut self, v: u32) { self.cell_id = v; }
    pub fn signal_strength(&self) -> u32 { self.signal_strength }
    pub fn set_signal_strength(&mut self, v: u32) { self.signal_strength = v; }
}

impl CosemObject for GsmDiagnostic {
    fn class_id(&self) -> u16 { 47 }
    fn logical_name(&self) -> ObisCode { self.logical_name }
    fn attribute_count(&self) -> u8 { 6 }
    fn method_count(&self) -> u8 { 0 }
    fn attribute_to_bytes(&self, attr: u8) -> Option<Vec<u8>> {
        match attr {
            1 => { let n=self.logical_name.to_bytes(); Some(vec![0x09,0x06,n[0],n[1],n[2],n[3],n[4],n[5]]) }
            2 => Some(dlms_axdr::encode(&DlmsData::VisibleString(self.operator.clone()))),
            3 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.status))),
            4 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.cell_id))),
            5 => Some(dlms_axdr::encode(&DlmsData::DoubleLongUnsigned(self.signal_strength))),
            _ => None,
        }
    }
    fn attribute_from_bytes(&mut self, attr: u8, data: &[u8]) -> Result<(), CosemObjectError> {
        match attr {
            2 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::VisibleString(s)|DlmsData::Utf8String(s) => { self.operator=s.clone(); Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            3 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.status=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            4 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.cell_id=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            5 => { let d=dlms_axdr::decode(data).map_err(|_|CosemObjectError::InvalidData)?; match d { DlmsData::DoubleLongUnsigned(v) => { self.signal_strength=v; Ok(()) } _ => Err(CosemObjectError::InvalidData) } }
            _ => Err(CosemObjectError::AttributeNotSupported(attr)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let obj = GsmDiagnostic::new(ObisCode::new(0,0,0,0,0,0));
        assert_eq!(obj.class_id(), 47);
    }
    #[test]
    fn test_getter() {
        let obj = GsmDiagnostic::new(ObisCode::new(0,0,0,0,0,0));
        let _ = obj.signal_strength();
    }
}
