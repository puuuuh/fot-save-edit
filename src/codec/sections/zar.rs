use crate::assert_section;
use crate::codec::error::ParseError;
use crate::codec::stream::Stream;
use crate::codec::Encodable;
use byteorder::ReadBytesExt;
use derive_debug::Dbg;
use std::ffi::CString;
use std::io::Read;
use std::io::{Error, Write};

const HEADER: &str = "<zar>\0";

#[derive(Dbg)]
pub struct Zar {
    pub magic: CString,
    pub h: i32,
    pub w: i32,
    pub data: Option<ZarSub>,
    #[dbg(formatter = "crate::codec::format::fmt_blob")]
    pub unknown: Vec<u8>,
}

#[derive(Dbg)]
pub struct ZarSub {
    #[dbg(placeholder = "...")]
    pub img: Vec<i32>,
    pub flag: u8,
}

impl<'a> Encodable<'a> for Zar {
    #[allow(clippy::size_of_in_element_count)]
    fn parse(data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        let magic = data.read_cstr()?.to_owned();
        let h = data.read_i32()?;
        let w = data.read_i32()?;
        let flag = data.read_u8()?;
        let opt = if flag != 0 {
            let img = <Vec<i32>>::parse(data)?;
            let flag = data.read_u8()?;
            Some(ZarSub { img, flag })
        } else {
            None
        };
        let unknown = <Vec<u8>>::parse(data)?;

        Ok(Self {
            magic,
            h,
            w,
            data: opt,
            unknown,
        })
    }

    fn write<T: Write>(&self, _stream: T) -> Result<(), Error> {
        todo!()
    }
}
