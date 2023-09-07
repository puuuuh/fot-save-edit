use crate::codec::error::ParseError;
use crate::codec::primitive::FOTString;
use crate::codec::Encodable;
use byteorder::{LittleEndian, ReadBytesExt};
use std::ffi::CStr;
use std::io::{ErrorKind, Read};

#[derive(Debug, Clone, Copy)]
pub struct Stream<'a> {
    buf: &'a [u8],
    cursor: &'a [u8],
}

impl<'a> Stream<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            buf: data,
            cursor: data,
        }
    }
    pub fn pos(&self) -> usize {
        unsafe { self.cursor.as_ptr().byte_offset_from(self.buf.as_ptr()) as _ }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn remain(&self) -> usize {
        self.cursor.len()
    }

    pub fn skip(&mut self, cnt: usize) -> Result<(), ParseError> {
        self.cursor = &self.cursor[cnt..];
        Ok(())
    }
    pub fn read_i32(&mut self) -> Result<i32, ParseError> {
        Ok(self.cursor.read_i32::<LittleEndian>()?)
    }

    pub fn read_u32(&mut self) -> Result<u32, ParseError> {
        Ok(self.cursor.read_u32::<LittleEndian>()?)
    }

    pub fn read_string(&mut self) -> Result<FOTString, ParseError> {
        FOTString::parse(self)
    }

    pub fn read_slice<'b>(&mut self, cnt: usize) -> Result<&'b [u8], ParseError>
    where
        'a: 'b,
    {
        let s = &self.cursor[..cnt];
        self.cursor = &self.cursor[cnt..];
        Ok(s)
    }

    pub fn read_cstr(&mut self) -> Result<&'a CStr, ParseError> {
        let s = CStr::from_bytes_until_nul(self.cursor)
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;

        self.cursor = &self.cursor[s.to_bytes_with_nul().len()..];
        Ok(s)
    }
}

impl Read for Stream<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.cursor.read(buf)
    }
}
