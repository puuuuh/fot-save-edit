use std::ffi::CString;
use std::io::{Error, Read, Write};
use crate::{assert_section};
use crate::codec::Encodable;
use crate::codec::error::ParseError;
use crate::codec::primitive::FOTString;
use crate::codec::stream::Stream;

const HEADER: &str = "<entity_file>\0";

#[derive(Debug)]
pub struct EntityFile {
    pub magic: CString,
    pub data: Vec<FOTString>,
}

impl<'a> Encodable<'a> for EntityFile {
    fn parse(data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        let magic = data.read_cstr()?.to_owned();

        Ok(Self {
            magic,
            data: <_>::parse(data)?,
        })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_all(HEADER.as_bytes())?;
        stream.write_all(self.magic.to_bytes_with_nul())?;
        self.data.write(stream)?;
        Ok(())
    }
}
