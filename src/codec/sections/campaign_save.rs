use std::ffi::{CStr};
use std::io::{Error, Read, Write};
use std::path::{PathBuf};
use byteorder::{LittleEndian, WriteBytesExt};
use crate::{assert_section};
use crate::codec::Encodable;
use crate::codec::error::ParseError;
use crate::codec::primitive::FOTString;
use crate::codec::stream::Stream;

const HEADER: &str = "<campaign_save>\0";

#[derive(Debug)]
pub struct CampaignSave<'a> {
    pub magic: &'a CStr,
    pub files: Vec<CampaignFile>,
}

#[derive(Debug)]
pub struct CampaignFile {
    pub path: FOTString,
    pub data: Vec<u8>,
}

impl<'a> Encodable<'a> for CampaignFile {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        let path = FOTString::parse(data)?;
        let len = data.read_u32()?;
        let data = data.read_slice(len as _)?.to_vec();
        Ok(CampaignFile {
            path,
            data,
        })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        self.path.write(&mut stream)?;
        stream.write_u32::<LittleEndian>(self.data.len() as _)?;
        stream.write_all(&self.data)?;
        Ok(())
    }
}

impl<'a> Encodable<'a> for CampaignSave<'a> {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        let magic = data.read_cstr()?;

        let files = <_>::parse(data)?;
        Ok(Self {
            magic,
            files,
        })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_all(HEADER.as_bytes())?;
        stream.write_all(self.magic.to_bytes_with_nul())?;
        self.files.write(stream)?;
        Ok(())
    }
}