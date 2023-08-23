use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{ErrorKind, Read};
use std::ops::Shr;
use std::slice;

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
            let mut buf = vec![0u16; len as usize];
            let b =
                unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, buf.len() * 2) };
            data.read_exact(b)?;

            FOTString::Utf16(
                widestring::Utf16String::from_vec(buf)
                    .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?,
            )
        } else {
            let mut buf = vec![0; len as usize];
            data.read_exact(&mut buf)?;
            FOTString::Ascii(
                String::from_utf8(buf)
                    .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?,
            )
        })
    }
}
