use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{assert_section, skip};
use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;

const ENTITYFILE_HEADER: &str = "<entity_file>";

#[derive(Debug)]
pub struct EntityFile {
    pub data: Vec<FOTString>,
}

impl EntityFile {
    pub fn read(mut data: impl Read) -> Result<Self, ParseError> {
        assert_section!(data, ENTITYFILE_HEADER);
        skip!(data, 0x03);

        Ok(Self {
            data: (0..data.read_u32::<LittleEndian>()?)
                .map(|_| FOTString::read(&mut data))
                .collect::<Result<_, _>>()?,
        })
    }
}
