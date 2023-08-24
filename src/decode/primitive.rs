use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{ErrorKind, Read};
use std::ops::Shr;
use std::slice;
use crate::read_primitive_vec;

// TODO: Win1251
#[derive(Debug)]
pub enum FOTString {
    Ascii(String),
    Utf16(widestring::Utf16String),
}

impl FOTString {
    pub fn read(mut data: impl Read) -> Result<FOTString, std::io::Error> {
        let header = data.read_u32::<LittleEndian>()?;
        let utf = header.shr(31) == 1u32;
        let len = header & 0x7FFFFFFF;

        Ok(if utf {
            let buf = read_primitive_vec!(data, u16, len);
            FOTString::Utf16(
                widestring::Utf16String::from_vec(buf)
                    .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?,
            )
        } else {
            let buf = read_primitive_vec!(data, u8, len);
            FOTString::Ascii(
                String::from_utf8(buf)
                    .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?,
            )
        })
    }
}
