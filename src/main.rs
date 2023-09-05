#![allow(clippy::size_of_in_element_count)]
#![feature(pointer_byte_offsets)]
#![feature(array_try_from_fn)]

mod codec;
mod files;

use std::env;
use std::fs;
use crate::codec::Encodable;
use crate::codec::sections::campaign_save::CampaignSave;
use crate::codec::sections::saveh::Saveh;

fn main() {
    let args: Vec<_> = env::args().collect();
    let save_path = args.get(1).cloned().unwrap_or("test1.sav".to_owned());

    let save_buf = fs::read(save_path).unwrap();

    let cursor = save_buf.as_slice();
    let mut cursor = codec::stream::Stream::new(cursor);
    let _svh = Saveh::parse(&mut cursor).unwrap();
    for w in &CampaignSave::parse(&mut cursor).unwrap().files {
        let mut t = w.data.clone();
        fs::write(w.path.file_name().unwrap(), t.read_slice(t.len()).unwrap()).unwrap();
        match &*w.path.extension().unwrap_or_default().to_string_lossy() {
            "cam" => {
                files::cam::Cam::parse(&mut w.data.clone());
            }
            "sav" => {
                files::sav::Sav::parse(&mut w.data.clone());
            }
            _ => {
                todo!()
            }
        }
    }
}
