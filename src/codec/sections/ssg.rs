use std::io::{Error, Read, Write};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{assert_section, skip};
use crate::codec::Encodable;
use crate::codec::error::ParseError;
use crate::codec::sections::entity_file::EntityFile;
use crate::codec::sections::esh::Esh;
use crate::codec::stream::Stream;

const HEADER: &str = "<SSG>\0";

#[derive(Debug)]
pub struct SSG {
    pub entity_file: EntityFile,
    pub values: Vec<SSGEntry>,
}

#[derive(Debug)]
pub struct SSGEntry {
    pub l: i32,
    pub flag: i16,
    pub data: Option<Esh>,
}

impl<'a> Encodable<'a> for SSG {
    fn parse(data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        skip!(data, 0x16);

        let entity_file = EntityFile::parse(data)?;

        let esh_count = data.read_i16::<LittleEndian>()?;
        let _tmp = data.read_u32()?;
        let entries = (0..esh_count - 1).map(|_| -> Result<_, ParseError> {
            let l = data.read_i32()?;
            let flag = data.read_i16::<LittleEndian>()?;
            let data = if flag != -1 {
                Some(Esh::parse(data)?)
            } else {
                None
            };
            Ok(SSGEntry {
                l,
                flag,
                data
            })
        }).collect::<Result<_, _>>()?;

        Ok(Self {
            entity_file,
            values: entries
        })
    }

    fn write<T: Write>(&self, _stream: T) -> Result<(), Error> {
        todo!()
    }
}