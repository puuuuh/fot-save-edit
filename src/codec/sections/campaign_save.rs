use std::ffi::{CStr};
use std::io::{Error, Read, Write};
use std::path::{PathBuf};
use crate::{assert_section};
use crate::codec::Encodable;
use crate::codec::error::ParseError;
use crate::codec::primitive::FOTString;
use crate::codec::stream::Stream;

const HEADER: &str = "<campaign_save>\0";

#[derive(Debug)]
pub struct CampaignSave<'a> {
    pub magic: &'a CStr,
    pub files: Vec<CampaignFile<'a>>,
}

#[derive(Debug)]
pub struct CampaignFile<'a> {
    pub path: PathBuf,
    pub data: Stream<'a>
    /*

     */
}



impl<'a> Encodable<'a> for CampaignSave<'a> {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        let magic = data.read_cstr()?;
        let cnt = data.read_u32()?;
        let mut res = vec![];
        for _ in 0..cnt {
            let path = PathBuf::from(&*FOTString::parse(data)?);
            let len = data.read_u32()?;
            let data = data.substream(len as _)?;
            res.push(CampaignFile {
                path,
                data,
            })
        }
        Ok(Self {
            magic,
            files: res,
        })
    }

    fn write<T: Write>(&self, _stream: T) -> Result<(), Error> {
        todo!();
    }
}