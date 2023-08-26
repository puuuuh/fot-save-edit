#![allow(clippy::size_of_in_element_count)]
#![feature(pointer_byte_offsets)]
use std::io::Read;
use byteorder::{LittleEndian, ReadBytesExt};

mod decode;
use memmem::Searcher;
use std::env;
use std::fs;


fn main() {
    let args: Vec<_> = env::args().collect();
    let save_path = args.get(1).cloned().unwrap_or("test1.sav".to_owned());

    let save_buf = fs::read(save_path).unwrap();

    let cursor = save_buf.as_slice();
    let mut cursor = decode::stream::Stream::new(cursor);
    decode::saveh::Saveh::read(&mut cursor).unwrap();
    decode::campaign_save::CampaignSave::read(&mut cursor).unwrap();
    unsafe { save_buf.len() - cursor.pos() };
}
