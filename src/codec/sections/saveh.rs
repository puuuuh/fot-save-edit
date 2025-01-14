use crate::assert_section;
use crate::codec::error::ParseError;
use crate::codec::primitive::FOTString;
use crate::codec::sections::zar::Zar;
use crate::codec::stream::Stream;
use crate::codec::Encodable;
use byteorder::{ReadBytesExt, WriteBytesExt};
use derive_debug::Dbg;
use std::ffi::CStr;
use std::io::{Error, Read, Write};

const HEADER: &str = "<saveh>\0";

#[derive(Dbg)]
pub struct Saveh<'a> {
    pub magic: &'a CStr,
    pub version: i8,
    pub strings: [FOTString; 5],
    pub tmp: [Zar; 8],
    #[dbg(placeholder = "...")]
    pub ints: [u32; 6],
}

impl<'a> Encodable<'a> for Saveh<'a> {
    fn parse(data: &mut Stream<'a>) -> Result<Saveh<'a>, ParseError> {
        assert_section!(data, HEADER);
        let magic = data.read_cstr()?;
        let some_ver = data.read_i8()?;

        Ok(Saveh {
            magic,
            version: some_ver,
            strings: <_>::parse(data)?,
            tmp: <_>::parse(data)?,
            ints: <_>::parse(data)?,
        })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_all(HEADER.as_bytes())?;
        stream.write_all(self.magic.to_bytes_with_nul())?;
        stream.write_i8(self.version)?;
        self.strings.write(&mut stream)?;
        self.tmp.write(&mut stream)?;
        self.ints.write(&mut stream)?;
        Ok(())
    }
}
