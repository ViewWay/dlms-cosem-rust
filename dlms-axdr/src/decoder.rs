//! AXDR decoder

use super::{decode_length, AxdrError};
use dlms_core::DlmsData;
use std::string::String;
use std::vec::Vec;

pub struct AxdrDecoder<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> AxdrDecoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn decode(&mut self) -> Result<DlmsData, AxdrError> {
        if self.pos >= self.data.len() {
            return Err(AxdrError::InsufficientData);
        }
        let tag = self.data[self.pos];
        self.pos += 1;
        self.decode_with_tag(tag)
    }

    fn decode_with_tag(&mut self, tag: u8) -> Result<DlmsData, AxdrError> {
        match tag {
            0x00 => Ok(DlmsData::None),
            0x01 => self.decode_array(),
            0x02 => self.decode_structure(),
            0x03 => self.decode_boolean(),
            0x04 => self.decode_bitstring(),
            0x05 => self.decode_double_long(),
            0x06 => self.decode_double_long_unsigned(),
            0x07 => self.decode_enum(),
            0x09 => self.decode_octet_string(),
            0x0A => self.decode_visible_string(),
            0x0C => self.decode_utf8_string(),
            0x0D => self.decode_bcd(),
            0x0F => self.decode_integer(),
            0x10 => self.decode_long(),
            0x11 => self.decode_unsigned(),
            0x12 => self.decode_long_unsigned(),
            0x16 => self.decode_long64(),
            0x17 => self.decode_long64_unsigned(),
            0x18 => self.decode_float(),
            0x19 => self.decode_double(),
            0x1A => self.decode_datetime(),
            0x1B => self.decode_date(),
            0x1C => self.decode_time(),
            0x1E => self.decode_compact_array_definition(),
            0x1F => self.decode_compact_array(),
            _ => Err(AxdrError::InvalidTag),
        }
    }

    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8], AxdrError> {
        if self.pos + len > self.data.len() {
            return Err(AxdrError::InsufficientData);
        }
        let slice = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Ok(slice)
    }

    fn read_u8(&mut self) -> Result<u8, AxdrError> {
        let bytes = self.read_bytes(1)?;
        Ok(bytes[0])
    }

    fn read_length(&mut self) -> Result<usize, AxdrError> {
        let (len, consumed) = decode_length(&self.data[self.pos..], &mut 0)?;
        self.pos += consumed;
        Ok(len)
    }

    fn decode_boolean(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 1 {
            return Err(AxdrError::InvalidLength);
        }
        let val = self.read_u8()?;
        Ok(DlmsData::Boolean(val != 0))
    }

    fn decode_bitstring(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_length()?;
        if len == 0 {
            return Err(AxdrError::InvalidLength);
        }
        let unused_bits = self.read_u8()?;
        let data = self.read_bytes(len - 1)?;
        Ok(DlmsData::BitString {
            unused_bits,
            data: data.to_vec(),
        })
    }

    fn decode_integer(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()? as usize;
        if len == 0 || len > 4 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(len)?;
        // DLMS integer is 1-byte signed
        let val = bytes[bytes.len() - 1] as i8;
        Ok(DlmsData::Integer(val))
    }

    fn decode_unsigned(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()? as usize;
        if len == 0 || len > 4 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(len)?;
        let mut arr = [0u8; 4];
        arr[4 - len..].copy_from_slice(bytes);
        Ok(DlmsData::Unsigned(u32::from_be_bytes(arr) as u8))
    }

    fn decode_long(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 2 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(2)?;
        Ok(DlmsData::Long(i16::from_be_bytes([bytes[0], bytes[1]])))
    }

    fn decode_long_unsigned(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 2 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(2)?;
        Ok(DlmsData::LongUnsigned(u16::from_be_bytes([
            bytes[0], bytes[1],
        ])))
    }

    fn decode_double_long(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 4 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(4)?;
        Ok(DlmsData::DoubleLong(i32::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
        ])))
    }

    fn decode_double_long_unsigned(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 4 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(4)?;
        Ok(DlmsData::DoubleLongUnsigned(u32::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
        ])))
    }

    fn decode_long64(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 8 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(8)?;
        let arr: [u8; 8] = bytes.try_into().map_err(|_| AxdrError::InvalidData)?;
        Ok(DlmsData::Long64(i64::from_be_bytes(arr)))
    }

    fn decode_long64_unsigned(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 8 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(8)?;
        let arr: [u8; 8] = bytes.try_into().map_err(|_| AxdrError::InvalidData)?;
        Ok(DlmsData::Long64Unsigned(u64::from_be_bytes(arr)))
    }

    fn decode_float(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 4 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(4)?;
        let arr: [u8; 4] = bytes.try_into().map_err(|_| AxdrError::InvalidData)?;
        Ok(DlmsData::Float(f32::from_be_bytes(arr)))
    }

    fn decode_double(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 8 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(8)?;
        let arr: [u8; 8] = bytes.try_into().map_err(|_| AxdrError::InvalidData)?;
        Ok(DlmsData::Double(f64::from_be_bytes(arr)))
    }

    fn decode_octet_string(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_length()?;
        let bytes = self.read_bytes(len)?;
        Ok(DlmsData::OctetString(bytes.to_vec()))
    }

    fn decode_visible_string(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_length()?;
        let bytes = self.read_bytes(len)?;
        Ok(DlmsData::VisibleString(
            String::from_utf8_lossy(bytes).into(),
        ))
    }

    fn decode_utf8_string(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_length()?;
        let bytes = self.read_bytes(len)?;
        Ok(DlmsData::Utf8String(String::from_utf8_lossy(bytes).into()))
    }

    fn decode_bcd(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_length()?;
        let bytes = self.read_bytes(len)?;
        Ok(DlmsData::Bcd(bytes.to_vec()))
    }

    fn decode_enum(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 1 {
            return Err(AxdrError::InvalidLength);
        }
        let val = self.read_u8()?;
        Ok(DlmsData::Enum(val))
    }

    fn decode_datetime(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 12 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(12)?;
        let arr: [u8; 12] = bytes.try_into().map_err(|_| AxdrError::InvalidData)?;
        Ok(DlmsData::DateTime(arr))
    }

    fn decode_date(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 5 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(5)?;
        let arr: [u8; 5] = bytes.try_into().map_err(|_| AxdrError::InvalidData)?;
        Ok(DlmsData::Date(arr))
    }

    fn decode_time(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_u8()?;
        if len != 4 {
            return Err(AxdrError::InvalidLength);
        }
        let bytes = self.read_bytes(4)?;
        let arr: [u8; 4] = bytes.try_into().map_err(|_| AxdrError::InvalidData)?;
        Ok(DlmsData::Time(arr))
    }

    fn decode_array(&mut self) -> Result<DlmsData, AxdrError> {
        let count = self.read_length()?;
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(self.decode()?);
        }
        Ok(DlmsData::Array(items))
    }

    fn decode_structure(&mut self) -> Result<DlmsData, AxdrError> {
        let count = self.read_length()?;
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(self.decode()?);
        }
        Ok(DlmsData::Structure(items))
    }

    fn decode_compact_array(&mut self) -> Result<DlmsData, AxdrError> {
        let len = self.read_length()?;
        let bytes = self.read_bytes(len)?;
        // Compact array: simplified decode
        Ok(DlmsData::CompactArray {
            header: bytes.to_vec(),
            data: vec![],
        })
    }

    fn decode_compact_array_definition(&mut self) -> Result<DlmsData, AxdrError> {
        let count = self.read_length()?;
        let mut items = Vec::with_capacity(count);
        for _ in 0..count {
            items.push(self.decode()?);
        }
        Ok(DlmsData::CompactArrayDefinition(items))
    }
}
