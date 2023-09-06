use crate::codec::error::ParseError;
use crate::codec::primitive::FOTString;
use crate::codec::stream::Stream;
use crate::assert_section;
use byteorder::{LittleEndian, ReadBytesExt};
use derive_debug::Dbg;
use std::io::{Error, Read, Write};
use crate::codec::Encodable;

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

    Sprite(FOTString),
    Type(FOTString),

    Bin(#[dbg(formatter = "crate::codec::format::fmt_blob")] Vec<u8>),
    Link {
        flags: u16,
        entity: u16,
    },
    Frame([f32; 12]),
    Rect([u32; 4]),

    ZoneName(FOTString),

    Unknown(
        u32,
        #[dbg(formatter = "crate::codec::format::fmt_blob")] Vec<u8>,
    ),
}

impl<'a> Encodable<'a> for Esh {
    fn parse(mut data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, ESH_HEADER);
        data.read_cstr()?;

        let values = (0..data.read_u32()?)
            .map(|_| -> Result<_, ParseError> {
                let name = FOTString::parse(&mut data)?;
                let t = data.read_u32()?;

                let data_len = data.read_u32()? as usize;
                let value = match t {
                    1 => EshValue::Bool(data.read_i8()? != 0),
                    2 => EshValue::Float(<_>::parse(data)?),
                    3 => EshValue::I32(<_>::parse(data)?),
                    4 => EshValue::String(<_>::parse(data)?),
                    5 => EshValue::Color(<_>::parse(data)?),

                    8 => EshValue::Sprite(<_>::parse(data)?),
                    9 => EshValue::Type(<_>::parse(data)?),

                    11 => {
                        EshValue::Bin(<_>::parse(data)?)
                    }

                    12 => {
                        let entity = data.read_u16::<LittleEndian>()?;
                        let flags = data.read_u16::<LittleEndian>()?;
                        EshValue::Link { flags, entity }
                    }

                    13 => EshValue::Frame(<_>::parse(data)?),

                    14 => EshValue::Rect(<_>::parse(data)?),

                    21 => EshValue::ZoneName(<_>::parse(data)?),

                    t => EshValue::Unknown(t, data.read_slice(data_len)?.to_vec()),
                };
                Ok(EshEntry { name, value })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { values })
    }

    fn write<T: Write>(&self, _stream: T) -> Result<(), Error> {
        todo!()
    }
}
