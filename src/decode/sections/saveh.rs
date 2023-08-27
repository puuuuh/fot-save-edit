use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};
use derive_debug::Dbg;
use crate::{assert_section, read_primitive_vec, skip};
use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;
use crate::decode::stream::Stream;

const HEADER: &str = "<saveh>\0";

#[derive(Dbg)]
pub struct Saveh {
    pub version: i8,
    pub strings: Vec<FOTString>,
    pub tmp: Vec<Test>,
    #[dbg(placeholder = "...")]
    pub ints: Vec<u32>,
}

#[derive(Dbg)]
pub struct Test {
    pub f1: i32,
    pub f2: i32,
    pub f4: Option<TestSub>,
    #[dbg(formatter = "crate::decode::format::fmt_blob")]
    pub shit: Vec<u8>
}

#[derive(Dbg)]
pub struct TestSub {
    #[dbg(placeholder = "...")]
    pub shit: Vec<i32>,
    pub flag: u8,
}

impl Test {
    #[allow(clippy::size_of_in_element_count)]
    pub fn read(mut data: impl Read) -> Result<Self, ParseError> {
        skip!(data, 8);
        let f1 = data.read_i32::<LittleEndian>()?;
        let f2 = data.read_i32::<LittleEndian>()?;
        let flag = data.read_u8()?;
        let opt = if flag != 0 {
            let cnt = data.read_i32::<LittleEndian>()?;
            let shit = read_primitive_vec!(data, i32, cnt);

            let flag = data.read_u8()?;
            Some(TestSub {
                shit,
                flag,
            })
        } else {
            None
        };
        let cnt = data.read_i32::<LittleEndian>()?;
        let shit = read_primitive_vec!(data, u8, cnt);

        Ok(Self {
            f1,
            f2,
            f4: opt,
            shit
        })

    }
}

impl Saveh {
    pub fn read(mut data: &mut Stream) -> Result<Saveh, ParseError> {
        assert_section!(data, HEADER);
        data.read_cstr()?;
        let some_ver = data.read_i8()?;

        Ok(Saveh {
            version: some_ver,
            strings: (0..5).map(|_| {
                FOTString::read(&mut data)
            }).collect::<Result<_, _>>()?,
            tmp: (0..8).map(|_| {
                Test::read(&mut data)
            }).collect::<Result<_, _>>()?,
            ints: read_primitive_vec!(data, u32, 6)
        })
    }
}