use std::io::{Error, Read, Write};
use crate::{assert_section};
use crate::codec::Encodable;
use crate::codec::error::ParseError;
use crate::codec::primitive::FOTString;
use crate::codec::stream::Stream;

const HEADER: &str = "<entity_file>\0";

#[derive(Debug)]
pub struct EntityFile {
    pub data: Vec<FOTString>,
}

impl<'a> Encodable<'a> for EntityFile {
    fn parse(mut data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        data.read_cstr()?;

        Ok(Self {
            data: <_>::parse(&mut data)?,
        })
    }

    fn write<T: Write>(&self, _stream: T) -> Result<(), Error> {
        todo!()
    }
}
