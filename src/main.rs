#![feature(pointer_byte_offsets)]

mod decode;

use inflate::inflate_bytes_zlib;
use memmem::Searcher;
use std::env;
use std::fs;
use std::process::exit;

const WORLD_HEADER: &[u8] = b"<world>";

fn main() {
    let args: Vec<_> = env::args().collect();
    let save_path = args.get(1).cloned().unwrap_or("test1.sav".to_owned());

    let save_buf = fs::read(save_path).unwrap();
    let searcher = memmem::TwoWaySearcher::new(b"<world>");

    let mut start = 0;
    let mut cursor = save_buf.as_slice();
    decode::saveh::Saveh::read(&mut cursor).unwrap();
}
