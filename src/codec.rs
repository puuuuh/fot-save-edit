use crate::codec::error::ParseError;
use crate::codec::stream::Stream;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Error, Write};

pub mod error;
pub mod format;
pub mod primitive;
pub mod sections;
pub mod shared;
pub mod stream;

pub trait Encodable<'a>
where
    Self: Sized,
{
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError>;
    fn write<T: Write>(&self, stream: T) -> Result<(), Error>;
}

impl<'a> Encodable<'a> for i32 {
    fn parse(data: &mut Stream) -> Result<Self, ParseError> {
        data.read_i32()
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_i32::<LittleEndian>(*self)
    }
}

impl<'a> Encodable<'a> for u32 {
    fn parse(data: &mut Stream) -> Result<Self, ParseError> {
        data.read_u32()
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_u32::<LittleEndian>(*self)
    }
}

impl<'a> Encodable<'a> for u8 {
    fn parse(data: &mut Stream) -> Result<Self, ParseError> {
        Ok(data.read_u8()?)
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_u8(*self)
    }
}

impl<'a> Encodable<'a> for f32 {
    fn parse(data: &mut Stream) -> Result<Self, ParseError> {
        Ok(data.read_f32::<LittleEndian>()?)
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_f32::<LittleEndian>(*self)
    }
}

impl<'a, T> Encodable<'a> for Vec<T>
where
    T: Encodable<'a>,
{
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        let len = data.read_u32()?;
        let mut res = Vec::with_capacity(len as _);
        for _ in 0..len {
            res.push(T::parse(data)?);
        }
        Ok(res)
    }

    fn write<T1: Write>(&self, mut stream: T1) -> Result<(), Error> {
        stream.write_u32::<LittleEndian>(self.len() as u32)?;
        for i in self {
            i.write(&mut stream)?;
        }
        Ok(())
    }
}

impl<'a, T: Encodable<'a>, const N: usize> Encodable<'a> for [T; N] {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        std::array::try_from_fn(|_| T::parse(data))
    }

    fn write<T1: Write>(&self, mut stream: T1) -> Result<(), Error> {
        for i in self {
            i.write(&mut stream)?
        }
        Ok(())
    }
}
