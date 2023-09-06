use std::ffi::CString;
use std::io::{Error, Read, Write};
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
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        let magic = data.read_cstr()?.to_owned();
        let unknown = data.read_slice(0x48)?.to_vec();

        let names = <Vec<FOTString>>::parse(data)?;
        let replicas = <Vec<Vec<FOTString>>>::parse(data)?;

        Ok(Self { magic, unknown, names, replicas })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_all(HEADER.as_bytes())?;
        stream.write_all(self.magic.to_bytes_with_nul())?;
        stream.write_all(&self.unknown)?;
        self.names.write(&mut stream)?;
        self.replicas.write(&mut stream)?;
        Ok(())
    }
}