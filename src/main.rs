#![allow(clippy::size_of_in_element_count)]
#![feature(pointer_byte_offsets)]
use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};

mod decode;
use memmem::Searcher;
use std::env;
use std::fs;

const WORLD_HEADER: &[u8] = b"<world>";

fn main() {
    let args: Vec<_> = env::args().collect();
    let save_path = args.get(1).cloned().unwrap_or("test1.sav".to_owned());

    let save_buf = fs::read(save_path).unwrap();
    let searcher = memmem::TwoWaySearcher::new(b"<world>");

    let mut start = 0;
    let mut cursor = save_buf.as_slice();
    let start = cursor.as_ptr();
    decode::saveh::Saveh::read(&mut cursor).unwrap();
    dbg!(unsafe { cursor.as_ptr().byte_offset_from(start) });
}
