use crate::assert_section;
use crate::codec::error::ParseError;
use crate::codec::primitive::FOTString;
use crate::codec::stream::Stream;
use crate::codec::Encodable;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use derive_debug::Dbg;
use std::ffi::CString;
use std::io::{Error, Read, Write};

const HEADER: &str = "<esh>\0";

#[derive(Debug)]
pub struct Esh {
    pub magic: CString,
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

impl <'a> Encodable<'a> for EshValue {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        let t = data.read_u32()?;
        let data_len = data.read_u32()? as usize;
        Ok(match t {
            1 => EshValue::Bool(data.read_i8()? != 0),
            2 => EshValue::Float(<_>::parse(data)?),
            3 => EshValue::I32(<_>::parse(data)?),
            4 => EshValue::String(<_>::parse(data)?),
            5 => EshValue::Color(<_>::parse(data)?),

            8 => EshValue::Sprite(<_>::parse(data)?),
            9 => EshValue::Type(<_>::parse(data)?),

            11 => EshValue::Bin(data.read_slice(data_len)?.to_vec()),

            12 => {
                let entity = data.read_u16::<LittleEndian>()?;
                let flags = data.read_u16::<LittleEndian>()?;
                EshValue::Link { flags, entity }
            }

            13 => EshValue::Frame(<_>::parse(data)?),

            14 => EshValue::Rect(<_>::parse(data)?),

            21 => EshValue::ZoneName(<_>::parse(data)?),

            t => EshValue::Unknown(t, data.read_slice(data_len)?.to_vec()),
        })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        match self {
            EshValue::Bool(b) => {
                stream.write_u32::<LittleEndian>(1)?;
                stream.write_u32::<LittleEndian>(1)?;
                stream.write_u8(*b as _)?;
            }
            EshValue::Float(data) => {
                stream.write_u32::<LittleEndian>(2)?;
                stream.write_u32::<LittleEndian>(4)?;
                stream.write_f32::<LittleEndian>(*data)?;
            }
            EshValue::I32(data) => {
                stream.write_u32::<LittleEndian>(3)?;
                stream.write_u32::<LittleEndian>(4)?;
                stream.write_i32::<LittleEndian>(*data)?;
            }
            EshValue::String(data) => {
                stream.write_u32::<LittleEndian>(4)?;
                stream.write_u32::<LittleEndian>(data.serialized_length() as _)?;
                data.write(stream)?;
            }
            EshValue::Color(data) => {
                stream.write_u32::<LittleEndian>(5)?;
                stream.write_u32::<LittleEndian>(12)?;
                data.write(stream)?;
            }

            EshValue::Sprite(data) => {
                stream.write_u32::<LittleEndian>(8)?;
                stream.write_u32::<LittleEndian>(data.serialized_length() as _)?;
                data.write(stream)?;
            }
            EshValue::Type(data) => {
                stream.write_u32::<LittleEndian>(9)?;
                stream.write_u32::<LittleEndian>(data.serialized_length() as _)?;
                data.write(stream)?;
            }
            EshValue::Bin(data) => {
                stream.write_u32::<LittleEndian>(11)?;
                stream.write_u32::<LittleEndian>(data.len() as _)?;
                stream.write_all(data)?;
            }
            EshValue::Link { flags, entity } => {
                stream.write_u32::<LittleEndian>(12)?;
                stream.write_u32::<LittleEndian>(4)?;
                stream.write_u16::<LittleEndian>(*entity)?;
                stream.write_u16::<LittleEndian>(*flags)?;
            }
            EshValue::Frame(data) => {
                stream.write_u32::<LittleEndian>(13)?;
                stream.write_u32::<LittleEndian>(48)?;
                data.write(stream)?;
            }
            EshValue::Rect(data) => {
                stream.write_u32::<LittleEndian>(14)?;
                stream.write_u32::<LittleEndian>(16)?;
                data.write(stream)?;
            }
            EshValue::ZoneName(data) => {
                stream.write_u32::<LittleEndian>(21)?;
                stream.write_u32::<LittleEndian>(data.serialized_length() as _)?;
                data.write(stream)?;
            }
            EshValue::Unknown(t, data) => {
                stream.write_u32::<LittleEndian>(*t)?;
                stream.write_u32::<LittleEndian>(data.len() as _)?;
                stream.write_all(data)?;
            }
        }
        Ok(())
    }
}

impl<'a> Encodable<'a> for EshEntry {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        let name = FOTString::parse(data)?;
        let value = EshValue::parse(data)?;

        Ok(EshEntry { name, value })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        self.name.write(&mut stream)?;
        self.value.write(&mut stream)?;
        Ok(())
    }
}

impl<'a> Encodable<'a> for Esh {
    fn parse(mut data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        let magic = data.read_cstr()?.to_owned();

        let values = (0..data.read_u32()?)
            .map(|_| -> Result<_, ParseError> {
                EshEntry::parse(data)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { magic, values })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_all(HEADER.as_bytes())?;
        stream.write_all(self.magic.to_bytes_with_nul())?;
        self.values.write(stream)?;
        Ok(())

    }
}
