use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};

const SDG_HEADER: &str = "<sgd>";
const SSG_HEADER: &str = "<SSG>";
const ENTITYFILE_HEADER: &str = "<entity_file>";
const ESH_HEADER: &str = "<esh>";

macro_rules! assert_section {
    ($data: ident, $s: ident) => {
        let mut buf = [0; $s.len()];
        $data.read_exact(&mut buf)?;
        if buf != $s.as_bytes() {
            dbg_str!($data, 10);
            return Err(ParseError::InvalidSection(
                String::from_utf8_lossy(&buf).into_owned(),
            ));
        }
    };
}

macro_rules! skip {
    ($data: ident, $s: literal) => {{
        let mut skip_buf = [0; $s];
        $data.read_exact(&mut skip_buf)?;
    }};
}

macro_rules! dbg_str {
    ($data: ident, $s: literal) => {{
        let mut buf = [0; $s];
        $data.read_exact(&mut buf)?;
        dbg!(String::from_utf8_lossy(&buf))
    }};
}

#[derive(Debug)]
pub struct SDG {
    pub names: Vec<FOTString>,
    pub replicas: Vec<Vec<FOTString>>,
}

impl SDG {
    pub fn read(mut data: impl Read) -> Result<Self, ParseError> {
        assert_section!(data, SDG_HEADER);
        skip!(data, 0x4B);

        let cnt = data.read_u32::<LittleEndian>()?;
        let names = (0..cnt)
            .map(|_| FOTString::read(&mut data))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        let cnt = data.read_u32::<LittleEndian>()?;
        let replicas = (0..cnt)
            .map(|_| -> Result<Vec<_>, std::io::Error> {
                let cnt = data.read_u32::<LittleEndian>()?;
                (0..cnt).map(|_| FOTString::read(&mut data)).collect()
            })
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        Ok(Self { names, replicas })
    }
}

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

#[derive(Debug)]
pub struct ESH {
    pub values: Vec<ESHEntry>,
}

#[derive(Debug)]
pub struct ESHEntry {
    pub name: FOTString,
    pub t: u32,
    pub data: Vec<u8>
}

impl ESH {
    fn read(mut data: impl Read) -> Result<Self, ParseError> {
        assert_section!(data, ESH_HEADER);
        skip!(data, 0x03);

        let values = (0..data.read_u32::<LittleEndian>()?).map(|_| -> Result<_, ParseError> {
            let name = FOTString::read(&mut data)?;
            let t = data.read_u32::<LittleEndian>()?;

            let data_len = data.read_u32::<LittleEndian>()? as usize;
            let mut data_buf = vec![0; data_len];
            data.read_exact(&mut data_buf)?;
            Ok(ESHEntry {
                name,
                t,
                data: data_buf
            })
        }).collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            values
        })
    }
}

#[derive(Debug)]
pub struct EntityFile {
    pub data: Vec<FOTString>,
}

impl EntityFile {
    fn read(mut data: impl Read) -> Result<Self, ParseError> {
        assert_section!(data, ENTITYFILE_HEADER);
        skip!(data, 0x03);

        Ok(Self {
            data: (0..data.read_u32::<LittleEndian>()?)
                .map(|_| FOTString::read(&mut data))
                .collect::<Result<_, _>>()?,
        })
    }
}
