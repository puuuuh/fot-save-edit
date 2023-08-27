use crate::decode::error::ParseError;
use crate::decode::stream::Stream;
use crate::{assert_section};
use std::io::{ErrorKind, Read};
use flate2::FlushDecompress;
use crate::decode::primitive::FOTString;
use crate::decode::sections::sdg::SSG;
use crate::decode::sections::sgd::SDG;

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

        let uncompressed_length = data.read_u32()? as usize;
        data.read_u32()?; // second len

        let mut result = vec![0; uncompressed_length];
        let mut decomp = flate2::Decompress::new(true);
        decomp.decompress(data.clone().read_slice(data.len() - data.pos())?, &mut result, FlushDecompress::Finish).unwrap();
        data.skip(decomp.total_in() as _)?;

        let world_data = result;

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
