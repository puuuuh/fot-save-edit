use crate::decode::error::ParseError;
use crate::decode::stream::Stream;
use crate::{assert_section};
use std::io::{Read};

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
