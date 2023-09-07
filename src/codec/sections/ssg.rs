use crate::assert_section;
use crate::codec::error::ParseError;
use crate::codec::sections::entity_file::EntityFile;
use crate::codec::sections::esh::Esh;
use crate::codec::stream::Stream;
use crate::codec::Encodable;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Error, Read, Write};

const HEADER: &str = "<SSG>\0";

#[derive(Debug)]
pub struct SSG {
    pub unknown: [u8; 0x16],
    pub entity_file: EntityFile,
    pub unknown1: u32,
    pub values: Vec<SSGEntry>,
}

#[derive(Debug)]
pub struct SSGEntry {
    pub id: i32,
    pub flag: i16,
    pub data: Option<Esh>,
}

impl<'a> Encodable<'a> for SSGEntry {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        let id = data.read_i32()?;
        let flag = data.read_i16::<LittleEndian>()?;
        let data = if flag != -1 {
            Some(Esh::parse(data)?)
        } else {
            None
        };
        Ok(SSGEntry { id, flag, data })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        stream.write_i32::<LittleEndian>(self.id)?;
        stream.write_i16::<LittleEndian>(self.flag)?;
        if self.flag != -1 {
            if let Some(esh) = &self.data {
                esh.write(stream)?
            }
        }
        Ok(())
    }
}

impl<'a> Encodable<'a> for SSG {
    fn parse(data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        let unknown = <_>::parse(data)?;

        let entity_file = EntityFile::parse(data)?;

        let esh_count = data.read_i16::<LittleEndian>()?;
        let unknown1 = data.read_u32()?;
        let entries = (0..esh_count - 1)
            .map(|_| -> Result<_, ParseError> {
                let res = SSGEntry::parse(data);
                res
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            unknown,
            unknown1,
            entity_file,
            values: entries,
        })
    }

    fn write<T: Write>(&self, mut _stream: T) -> Result<(), Error> {
        _stream.write_all(HEADER.as_bytes())?;
        _stream.write_all(&self.unknown)?;
        self.entity_file.write(&mut _stream)?;
        _stream.write_i16::<LittleEndian>((self.values.len() + 1) as i16)?;
        _stream.write_u32::<LittleEndian>(self.unknown1)?;
        for e in &self.values {
            e.write(&mut _stream)?;
        }
        Ok(())
    }
}
