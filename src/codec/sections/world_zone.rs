use crate::codec::error::ParseError;
use crate::codec::stream::Stream;
use crate::{assert_section};
use std::io::{Error, Read, Write};
use crate::codec::Encodable;

const HEADER: &str = "<world_zone>\0";

#[derive(Debug)]
pub struct WorldZone {
    pub data: Vec<u8>,
}

impl<'a> Encodable<'a> for WorldZone {
    fn parse(data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        dbg!(data.read_cstr()?);

        Ok(WorldZone {
            data: data.read_slice(data.len() - data.pos())?.to_vec(),
        })
    }

    fn write<T: Write>(&self, _stream: T) -> Result<(), Error> {
        todo!()
    }
}
