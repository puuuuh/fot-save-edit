#![allow(clippy::size_of_in_element_count)]
#![feature(pointer_byte_offsets)]
mod decode;
use std::env;
use std::fs;
use crate::decode::sections::campaign_save::CampaignSave;
use crate::decode::sections::saveh::Saveh;


fn main() {
    let args: Vec<_> = env::args().collect();
    let save_path = args.get(1).cloned().unwrap_or("test1.sav".to_owned());

    let save_buf = fs::read(save_path).unwrap();

    let cursor = save_buf.as_slice();
    let mut cursor = decode::stream::Stream::new(cursor);
    Saveh::read(&mut cursor).unwrap();
    dbg!(CampaignSave::read(&mut cursor).unwrap());

}
