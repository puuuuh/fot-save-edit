use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};
use crate::{assert_section, skip};
use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;
use crate::decode::sections::entity_file::EntityFile;
use crate::decode::sections::esh::ESH;

const SSG_HEADER: &str = "<SSG>";

#[derive(Debug)]
pub struct SSG {
    pub entity_file: EntityFile,
    pub values: Vec<SSGEntry>,
}

#[derive(Debug)]
pub struct SSGEntry {
    pub l: i32,
    pub f: i16,
    pub data: Option<ESH>,
}

impl SSG {
    pub fn read(mut data: impl Read) -> Result<Self, ParseError> {
        assert_section!(data, SSG_HEADER);
        skip!(data, 0x17);

        let entity_file = EntityFile::read(&mut data)?;

        let esh_count = data.read_i16::<LittleEndian>()?;
        let tmp = data.read_u32::<LittleEndian>()?;
        let entries = (0..esh_count - 1).map(|_| -> Result<_, ParseError> {
            let l = data.read_i32::<LittleEndian>()?;
            let flag = data.read_i16::<LittleEndian>()?;
            let data = if flag == -1 {
                None
            } else {
                Some(ESH::read(&mut data)?)
            };
            Ok(SSGEntry {

                l,
                f: flag,
                data
            })
        }).collect::<Result<_, _>>()?;

        Ok(Self {
            entity_file,
            values: entries
        })
    }
}
