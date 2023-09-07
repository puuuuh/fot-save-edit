#![allow(clippy::size_of_in_element_count)]
#![feature(pointer_byte_offsets)]
#![feature(array_try_from_fn)]

mod codec;
mod files;

use crate::codec::sections::campaign_save::CampaignSave;
use crate::codec::sections::saveh::Saveh;
use crate::codec::sections::world::World;
use crate::codec::stream::Stream;
use crate::codec::Encodable;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    let save_path = args.get(1).cloned().unwrap_or("test1.sav".to_owned());

    let save_buf = fs::read(save_path).unwrap();

    let cursor = save_buf.as_slice();
    let mut cursor = Stream::new(cursor);
    let _svh = Saveh::parse(&mut cursor).unwrap();
    for w in &CampaignSave::parse(&mut cursor).unwrap().files {
        let path = Path::new(&*w.path);
        fs::write(path.file_name().unwrap(), &w.data).unwrap();
        match &*path.extension().unwrap_or_default().to_string_lossy() {
            "cam" => {
                dbg!(files::cam::Cam::parse(&mut Stream::new(&w.data)).unwrap());
            }
            "sav" => {
                files::sav::Sav::parse(&mut Stream::new(&w.data)).unwrap();
            }
            _ => {
                todo!()
            }
        }
    }
}
