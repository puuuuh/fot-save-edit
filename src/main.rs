mod decode;

use inflate::inflate_bytes_zlib;
use memmem::Searcher;
use std::env;
use std::fs;

const WORLD_HEADER: &[u8] = b"<world>";

fn main() {
    let args: Vec<_> = env::args().collect();
    let save_path = args.get(1).cloned().unwrap_or("test1.sav".to_owned());

    let save_buf = fs::read(save_path).unwrap();
    let searcher = memmem::TwoWaySearcher::new(b"<world>");
    let next_section_search = memmem::TwoWaySearcher::new(b"<SSG>");

    let mut start = 0;
    while let Some(i) = searcher.search_in(&save_buf[start..]) {
        let i = start + i;
        start = i + WORLD_HEADER.len();

        println!("Found offset 0x{:x}", i);
        match inflate_bytes_zlib(&save_buf[i + 0x13..]) {
            Ok(out_buf) => {
                fs::write(&format!("world_{i}.bin"), &out_buf).expect("out_buf");
                let mut data = out_buf.as_slice();
                decode::primitive::FOTString::read(&mut data).unwrap(); // HEADER
                dbg!(decode::sections::SDG::read(&mut data).unwrap());
                dbg!(decode::sections::SDG::read(&mut data).unwrap());
            }
            Err(e) => eprintln!("{}", e),
        };
    }
}
