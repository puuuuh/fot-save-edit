use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;
use crate::decode::stream::Stream;
use crate::{assert_section, skip};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;

const ESH_HEADER: &str = "<esh>";

/*
value_type
2 = i32,
3 = i32,
4 = string?
12 = bool
 */

#[derive(Debug)]
pub struct ESH {
    pub values: Vec<ESHEntry>,
}

#[derive(Debug)]
pub struct ESHEntry {
    pub name: FOTString,
    pub value: ESHValue,
}

#[derive(Debug)]
pub enum ESHValue {
    Bool(bool),
    Float(f32),
    I32(i32),
    String(FOTString),
    Color([u32; 3]),
    Team(FOTString),

    Sprite(FOTString),
    Type(FOTString),

    Bin(Vec<u8>),
    Link { flags: u16, entity: u16 },
    Frame(Vec<u8>),
    Rect([u32; 4]),

    ZoneName(FOTString),

    Unknown(u32, Vec<u8>),
}

impl ESH {
    pub fn read(mut data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, ESH_HEADER);
        skip!(data, 0x03);

        let values = (0..data.read_u32()?)
            .map(|_| -> Result<_, ParseError> {
                let name = FOTString::read(&mut data)?;
                let t = data.read_u32()?;

                let data_len = data.read_u32()? as usize;
                let value = match t {
                    1 => ESHValue::Bool(data.read_i8()? != 0),
                    2 => ESHValue::Float(f32::from_bits(data.read_u32()?)),
                    3 => ESHValue::I32(data.read_i32()?),
                    4 => ESHValue::String(data.read_string()?),
                    5 => ESHValue::Color([data.read_u32()?, data.read_u32()?, data.read_u32()?]),

                    8 => ESHValue::Sprite(data.read_string()?),
                    9 => ESHValue::Type(data.read_string()?),

                    11 => {
                        let len = data.read_u32()?;
                        ESHValue::Bin(data.read_slice(len as _)?.to_vec())
                    }

                    12 => {
                        let entity = data.read_u16::<LittleEndian>()?;
                        let flags = data.read_u16::<LittleEndian>()?;
                        ESHValue::Link { flags, entity }
                    }

                    13 => ESHValue::Frame(data.read_slice(data_len)?.to_vec()),

                    14 => ESHValue::Rect([
                        data.read_u32()?,
                        data.read_u32()?,
                        data.read_u32()?,
                        data.read_u32()?,
                    ]),

                    21 => ESHValue::ZoneName(data.read_string()?),

                    t => ESHValue::Unknown(t, data.read_slice(data_len)?.to_vec()),
                };
                Ok(ESHEntry { name, value })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { values })
    }
}
