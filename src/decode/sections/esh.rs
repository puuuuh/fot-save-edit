use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{assert_section, skip};
use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;

const ESH_HEADER: &str = "<esh>";

#[derive(Debug)]
pub struct ESH {
    pub values: Vec<ESHEntry>,
}

#[derive(Debug)]
pub struct ESHEntry {
    pub name: FOTString,
    pub t: u32,
    pub data: Vec<u8>
}

impl ESH {
    pub fn read(mut data: impl Read) -> Result<Self, ParseError> {
        assert_section!(data, ESH_HEADER);
        skip!(data, 0x03);

        let values = (0..data.read_u32::<LittleEndian>()?).map(|_| -> Result<_, ParseError> {
            let name = FOTString::read(&mut data)?;
            let t = data.read_u32::<LittleEndian>()?;

            let data_len = data.read_u32::<LittleEndian>()? as usize;
            let mut data_buf = vec![0; data_len];
            data.read_exact(&mut data_buf)?;
            Ok(ESHEntry {
                name,
                t,
                data: data_buf
            })
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            values
        })
    }
}
