use std::ffi::CString;
use std::io::{Error, Read, Write};
use byteorder::{LittleEndian, WriteBytesExt};
use crate::{assert_section};
use crate::codec::Encodable;
use crate::codec::error::ParseError;
use crate::codec::primitive::FOTString;
use crate::codec::stream::Stream;

const HEADER: &str = "<sgd>\0";

#[derive(Debug)]
pub struct SDG {
    pub magic: CString,
    pub unknown: Vec<u8>,
    pub names: Vec<FOTString>,
    pub replicas: Vec<Vec<FOTString>>,
}



impl<'a> Encodable<'a> for SDG {
    fn parse(mut data: &mut Stream<'a>) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        let magic = data.read_cstr()?.to_owned();
        let unknown = data.read_slice(0x48)?.to_vec();

        let cnt = data.read_u32()?;
        let names = (0..cnt)
            .map(|_| FOTString::parse(&mut data))
            .collect::<Result<Vec<_>, ParseError>>()?;

        let cnt = data.read_u32()?;
        let replicas = (0..cnt)
            .map(|_| -> Result<Vec<_>, ParseError> {
                let cnt = data.read_u32()?;
                (0..cnt).map(|_| FOTString::parse(&mut data)).collect()
            })
            .collect::<Result<Vec<_>, ParseError>>()?;

        Ok(Self { magic, unknown, names, replicas })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_all(HEADER.as_bytes())?;
        stream.write_all(self.magic.to_bytes_with_nul())?;
        stream.write_all(&self.unknown)?;
        stream.write_u32::<LittleEndian>(self.names.len() as u32)?;
        for name in &self.names {
            name.write(&mut stream)?;
        }

        stream.write_u32::<LittleEndian>(self.names.len() as u32)?;
        for repl in &self.replicas {
            stream.write_u32::<LittleEndian>(repl.len() as u32)?;
            for r in repl {
                r.write(&mut stream)?;
            }
        }
        Ok(())
    }
}