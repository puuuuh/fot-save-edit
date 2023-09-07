use std::ffi::CStr;
use crate::codec::error::ParseError;
use crate::codec::stream::Stream;
use crate::{assert_section};
use std::io::{Error, Read, Write};
use byteorder::{LittleEndian, WriteBytesExt};
use derive_debug::Dbg;
use flate2::{Compression, FlushCompress, FlushDecompress};
use crate::codec::Encodable;
use crate::codec::primitive::FOTString;
use crate::codec::sections::ssg::SSG;
use crate::codec::sections::sgd::SDG;

const HEADER: &str = "<world>\0";

#[derive(Dbg)]
pub struct World<'a> {
    pub magic: &'a CStr,
    pub path: FOTString,
    pub sdg: SDG,
    pub ssg: SSG,
    #[dbg(formatter = "crate::codec::format::fmt_blob")]
    pub tail: Vec<u8>,
}

impl<'a> Encodable<'a> for World<'a> {
    fn parse(data: &mut Stream<'a>) -> Result<Self, ParseError> {
        assert_section!(data, HEADER);
        let magic = data.read_cstr()?;

        let uncompressed_length = data.read_u32()? as usize;
        data.read_u32()?; // second len

        let mut result = vec![0; uncompressed_length];
        let mut decomp = flate2::Decompress::new(true);
        decomp.decompress(data.clone().read_slice(data.len() - data.pos())?, &mut result, FlushDecompress::Finish).unwrap();
        data.skip(decomp.total_in() as _)?;

        let world_data = result;

        let mut stream = Stream::new(&world_data);
        let path = FOTString::parse(&mut stream)?; // HEADER
        let sdg = SDG::parse(&mut stream)?;
        let ssg = SSG::parse(&mut stream)?;

        Ok(Self {
            magic,
            path,
            sdg,
            ssg,
            tail: stream.read_slice(stream.len() - stream.pos())?.to_vec()
        })
    }

    fn write<T: Write>(&self, mut stream: T) -> Result<(), Error> {
        let mut world_data = Vec::new();
        self.path.write(&mut world_data)?;
        self.sdg.write(&mut world_data)?;
        self.ssg.write(&mut world_data)?;
        world_data.extend_from_slice(&self.tail);
        let mut result = Vec::with_capacity(world_data.len());
        let mut comp = flate2::Compress::new(Compression::best(), true);
        comp.compress_vec(&world_data, &mut result, FlushCompress::Finish).unwrap();

        stream.write_all(HEADER.as_bytes())?;
        stream.write_all(self.magic.to_bytes_with_nul())?;
        stream.write_u32::<LittleEndian>(world_data.len() as _)?;
        stream.write_u32::<LittleEndian>(world_data.len() as _)?;
        stream.write_all(&result)?;
        Ok(())
    }
}
