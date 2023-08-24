use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{assert_section, skip};
use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;

const SDG_HEADER: &str = "<sgd>";

#[derive(Debug)]
pub struct SDG {
    pub names: Vec<FOTString>,
    pub replicas: Vec<Vec<FOTString>>,
}

impl SDG {
    pub fn read(mut data: impl Read) -> Result<Self, ParseError> {
        assert_section!(data, SDG_HEADER);
        skip!(data, 0x4B);

        let cnt = data.read_u32::<LittleEndian>()?;
        let names = (0..cnt)
            .map(|_| FOTString::read(&mut data))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        let cnt = data.read_u32::<LittleEndian>()?;
        let replicas = (0..cnt)
            .map(|_| -> Result<Vec<_>, std::io::Error> {
                let cnt = data.read_u32::<LittleEndian>()?;
                (0..cnt).map(|_| FOTString::read(&mut data)).collect()
            })
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        Ok(Self { names, replicas })
    }
}