use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{assert_section, skip};
use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;
use crate::decode::stream::Stream;

const HEADER: &str = "<sgd>\0";

#[derive(Debug)]
pub struct SDG {
    pub names: Vec<FOTString>,
    pub replicas: Vec<Vec<FOTString>>,
}

impl SDG {
    pub fn read(mut data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        data.skip(0x4A)?;

        let cnt = data.read_u32()?;
        let names = (0..cnt)
            .map(|_| FOTString::read(&mut data))
            .collect::<Result<Vec<_>, ParseError>>()?;

        let cnt = data.read_u32()?;
        let replicas = (0..cnt)
            .map(|_| -> Result<Vec<_>, ParseError> {
                let cnt = data.read_u32()?;
                (0..cnt).map(|_| FOTString::read(&mut data)).collect()
            })
            .collect::<Result<Vec<_>, ParseError>>()?;

        Ok(Self { names, replicas })
    }
}