use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;
use crate::decode::stream::Stream;
use crate::{assert_section, skip};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::Read;
use derive_debug::Dbg;

const ESH_HEADER: &str = "<esh>\0";

/*
value_type
2 = i32,
3 = i32,
4 = string?
12 = bool
 */

#[derive(Debug)]
pub struct Esh {
    pub values: Vec<EshEntry>,
}

#[derive(Debug)]
pub struct EshEntry {
    pub name: FOTString,
    pub value: EshValue,
}

#[derive(Dbg)]
pub enum EshValue {
    Bool(bool),
    Float(f32),
    I32(i32),
    String(FOTString),
    Color(#[dbg(placeholder = "...")] [u32; 3]),
    Team(FOTString),

    Sprite(FOTString),
    Type(FOTString),

    Bin(#[dbg(formatter = "crate::decode::format::fmt_blob")] Vec<u8>),
    Link { flags: u16, entity: u16 },
    Frame(#[dbg(formatter = "crate::decode::format::fmt_blob")] Vec<u8>),
    Rect([u32; 4]),

    ZoneName(FOTString),

    Unknown(u32, #[dbg(formatter = "crate::decode::format::fmt_blob")] Vec<u8>),
}

impl Esh {
    pub fn read(mut data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, ESH_HEADER);
        data.read_cstr()?;

        let values = (0..data.read_u32()?)
            .map(|_| -> Result<_, ParseError> {
                let name = FOTString::read(&mut data)?;
                let t = data.read_u32()?;

                let data_len = data.read_u32()? as usize;
                let value = match t {
                    1 => EshValue::Bool(data.read_i8()? != 0),
                    2 => EshValue::Float(f32::from_bits(data.read_u32()?)),
                    3 => EshValue::I32(data.read_i32()?),
                    4 => EshValue::String(data.read_string()?),
                    5 => EshValue::Color([data.read_u32()?, data.read_u32()?, data.read_u32()?]),

                    8 => EshValue::Sprite(data.read_string()?),
                    9 => EshValue::Type(data.read_string()?),

                    11 => {
                        let len = data.read_u32()?;
                        EshValue::Bin(data.read_slice(len as _)?.to_vec())
                    }

                    12 => {
                        let entity = data.read_u16::<LittleEndian>()?;
                        let flags = data.read_u16::<BigEndian>()?;
                        EshValue::Link { flags, entity }
                    }

                    13 => EshValue::Frame(data.read_slice(data_len)?.to_vec()),

                    14 => EshValue::Rect([
                        data.read_u32()?,
                        data.read_u32()?,
                        data.read_u32()?,
                        data.read_u32()?,
                    ]),

                    21 => EshValue::ZoneName(data.read_string()?),

                    t => EshValue::Unknown(t, data.read_slice(data_len)?.to_vec()),
                };
                Ok(EshEntry { name, value })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { values })
    }
}
