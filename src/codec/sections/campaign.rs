use crate::assert_section;
use crate::codec::error::ParseError;
use crate::codec::primitive::FOTString;
use crate::codec::stream::Stream;
use crate::codec::Encodable;
use derive_debug::Dbg;
use std::io::{Error, Read, Write};

const HEADER: &str = "<campaign>\0";

#[derive(Dbg)]
pub struct Campaign<'a> {
    #[dbg(placeholder = "...")]
    pub raw: &'a [u8],
    pub world_file: FOTString,
}

impl<'a> Encodable<'a> for Campaign<'a> {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        let d = data.clone().read_slice(data.remain())?;
        //let _magic = data.read_cstr()?;
        data.skip(0x22BA)?;
        let world_file = data.read_string()?;
        data.remain();
        Ok(Self { raw: d, world_file })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_all(HEADER.as_bytes())?;
        stream.write_all(self.raw)?;

        Ok(())
    }
}
