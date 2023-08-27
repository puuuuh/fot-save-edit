use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{assert_section, skip};
use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;
use crate::decode::stream::Stream;

const HEADER: &str = "<entity_file>\0";

#[derive(Debug)]
pub struct EntityFile {
    pub data: Vec<FOTString>,
}

impl EntityFile {
    pub fn read(mut data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        data.read_cstr()?;

        Ok(Self {
            data: (0..data.read_u32()?)
                .map(|_| FOTString::read(&mut data))
                .collect::<Result<_, _>>()?,
        })
    }
}
