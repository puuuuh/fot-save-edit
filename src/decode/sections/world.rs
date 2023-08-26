use crate::decode::error::ParseError;
use crate::decode::stream::Stream;
use crate::{assert_section};
use std::io::{ErrorKind, Read};
use crate::decode::primitive::FOTString;
use crate::decode::sections::sdg::SSG;
use crate::decode::sections::ssg::SDG;

const HEADER: &str = "<world>\0";

#[derive(Debug)]
pub struct World {
    pub path: FOTString,
    pub sdg: SDG,
    pub ssg: SSG,
    pub tail: Vec<u8>,
}

impl World {
    pub fn read(data: &mut Stream) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        data.read_cstr()?;
        let uncompressed_length = data.read_u32()?;
        data.read_u32()?;
        let world_data = inflate::inflate_bytes_zlib(data.read_slice(data.len() - data.pos())?)
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;
        assert_eq!(uncompressed_length as usize, world_data.len());

        let mut stream = Stream::new(&world_data);
        let path = FOTString::read(&mut stream).unwrap(); // HEADER
        let sdg = SDG::read(&mut stream).unwrap();
        let ssg = SSG::read(&mut stream).unwrap();

        Ok(Self {
            path,
            sdg,
            ssg,
            tail: stream.read_slice(stream.len() - stream.pos())?.to_vec()
        })
    }
}
