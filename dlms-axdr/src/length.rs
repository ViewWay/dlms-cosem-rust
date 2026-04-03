//! AXDR data length encoding/decoding

/// Encode a length value in AXDR format
pub fn encode_length(len: usize) -> Vec<u8> {
    if len < 0x80 {
        vec![len as u8]
    } else if len < 0x100 {
        vec![0x81, len as u8]
    } else if len < 0x10000 {
        vec![0x82, (len >> 8) as u8, len as u8]
    } else if len < 0x1000000 {
        vec![0x83, (len >> 16) as u8, (len >> 8) as u8, len as u8]
    } else {
        vec![0x84, (len >> 24) as u8, (len >> 16) as u8, (len >> 8) as u8, len as u8]
    }
}

/// Decode a length value from AXDR format. Returns (length, bytes_consumed).
pub fn decode_length(data: &[u8], _offset: &mut usize) -> Result<(usize, usize), super::AxdrError> {
    if data.is_empty() {
        return Err(super::AxdrError::InsufficientData);
    }
    let first = data[0] as usize;
    if first < 0x80 {
        Ok((first, 1))
    } else {
        let num_bytes = first & 0x7F;
        if num_bytes == 0 || data.len() < 1 + num_bytes {
            return Err(super::AxdrError::InvalidLength);
        }
        let mut len = 0usize;
        for i in 0..num_bytes {
            len = (len << 8) | data[1 + i] as usize;
        }
        Ok((len, 1 + num_bytes))
    }
}
