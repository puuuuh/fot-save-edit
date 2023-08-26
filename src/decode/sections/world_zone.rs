use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;
use crate::decode::sections::sdg::SSG;
use crate::decode::sections::ssg::SDG;
use crate::decode::stream::Stream;
use crate::{assert_section, dbg_str, decode, skip};
use std::io::{ErrorKind, Read};

const HEADER: &str = "<world_zone>\0";

#[derive(Debug)]
pub struct WorldZone {
    pub data: Vec<u8>,
}

impl WorldZone {
    pub fn read(data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        dbg!(data.read_cstr()?);
        Ok(WorldZone {
            data: data.read_slice(data.len() - data.pos())?.to_vec(),
        })
    }
}
