use crate::codec::error::ParseError;
use crate::codec::stream::Stream;
use crate::codec::Encodable;
use byteorder::{LittleEndian, WriteBytesExt};
use encoding_rs::{UTF_16LE, WINDOWS_1251};
use std::io::Write;
use std::ops::{Deref, Shr};

#[derive(Debug, PartialEq, Eq)]
pub enum FOTString {
    Ascii(String),
    Win1251(String),
    Utf16(String),
}

impl Deref for FOTString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            FOTString::Ascii(ref data) => data,
            FOTString::Win1251(ref data) => data,
            FOTString::Utf16(ref data) => data,
        }
    }
}

impl FOTString {
    pub fn serialized_length(&self) -> usize {
        (match self {
            FOTString::Ascii(data) => data.len(),
            FOTString::Utf16(data) | FOTString::Win1251(data) => data.len() * 2,
        }) + 4
    }
}

impl<'a> Encodable<'a> for FOTString {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        let header = data.read_u32()?;
        let utf = header.shr(31) == 1u32;
        let len = header & 0x7FFFFFFF;

        Ok(if utf {
            let buf = data
                .read_slice(len as usize * 2)?
                .iter()
                .enumerate()
                .filter_map(|(i, a)| (i % 2 == 0).then_some(*a))
                .collect::<Vec<_>>();
            let (str, _, err) = WINDOWS_1251.decode(&buf);
            if !err {
                FOTString::Win1251(str.into_owned())
            } else {
                let (str, _, _) = UTF_16LE.decode(&buf);
                FOTString::Utf16(str.into_owned())
            }
        } else {
            let buf = data.read_slice(len as usize)?;
            FOTString::Ascii(String::from_utf8_lossy(buf).into_owned())
        })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), std::io::Error> {
        match self {
            FOTString::Ascii(data) => {
                let data = data.as_bytes();
                stream.write_u32::<LittleEndian>(data.len() as u32)?;
                stream.write_all(data)?;
            }
            FOTString::Win1251(data) => {
                let (data, _, _err) = WINDOWS_1251.encode(data);
                stream.write_u32::<LittleEndian>(data.len() as u32 | 1u32 << 31)?;
                for v in &*data {
                    stream.write_all(&[*v, 0])?;
                }
            }
            FOTString::Utf16(data) => {
                let (data, _, _err) = UTF_16LE.encode(data);
                stream.write_u32::<LittleEndian>((data.len() / 2) as u32 | 1u32 << 31)?;
                stream.write_all(&data)?;
            }
        };

        Ok(())
    }
}
