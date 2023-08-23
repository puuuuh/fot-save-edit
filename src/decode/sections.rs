use std::io::{Read, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use crate::decode::error::ParseError;
use crate::decode::primitive::FOTString;

const SDG_HEADER: &str = "<sgd>";

#[derive(Debug)]
pub struct SDG {
    pub names: Vec<FOTString>,
    pub replicas: Vec<Vec<FOTString>>,
}

impl SDG {
    pub fn read(mut data: impl Read) -> Result<Self, ParseError> {
        let mut buf = [0; SDG_HEADER.len()];
        data.read_exact(&mut buf)?;
        if buf != SDG_HEADER.as_bytes() {
            return Err(ParseError::InvalidSection(String::from_utf8_lossy(&buf).into_owned()))
        }

        let mut skip_buf = [0; 2 + 0x4e - 5];

        data.read_exact(&mut skip_buf)?;

        let cnt = data.read_u32::<LittleEndian>()?;
        let names = (0..cnt).map(|_| FOTString::read(&mut data)).collect::<Result<Vec<_>, std::io::Error>>()?;

        let cnt = data.read_u32::<LittleEndian>()?;
        let replicas = (0..cnt).map(|_| -> Result<Vec<_>, std::io::Error> {
            let cnt = data.read_u32::<LittleEndian>()?;
            (0..cnt).map(|_| FOTString::read(&mut data)).collect()
        }).collect::<Result<Vec<_>, std::io::Error>>()?;

        Ok(Self {
            names,
            replicas
        })
    }
}

#[derive(Debug)]
pub struct SSG {
    pub names: Vec<FOTString>,
    pub replicas: Vec<Vec<FOTString>>,
}

impl SSG {
    pub fn read(mut data: impl Read) -> Result<Self, ParseError> {
        let mut buf = [0; SDG_HEADER.len()];
        data.read_exact(&mut buf)?;
        if buf != SDG_HEADER.as_bytes() {
            return Err(ParseError::InvalidSection(String::from_utf8_lossy(&buf).into_owned()))
        }

        let mut skip_buf = [0; 2 + 0x4e - 5];

        data.read_exact(&mut skip_buf)?;

        let cnt = data.read_u32::<LittleEndian>()?;
        let names = (0..cnt).map(|_| FOTString::read(&mut data)).collect::<Result<Vec<_>, std::io::Error>>()?;

        let cnt = data.read_u32::<LittleEndian>()?;
        let replicas = (0..cnt).map(|_| -> Result<Vec<_>, std::io::Error> {
            let cnt = data.read_u32::<LittleEndian>()?;
            (0..cnt).map(|_| FOTString::read(&mut data)).collect()
        }).collect::<Result<Vec<_>, std::io::Error>>()?;

        Ok(Self {
            names,
            replicas
        })
    }
}
