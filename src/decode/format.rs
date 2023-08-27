pub fn fmt_blob(v: &Vec<u8>) -> String {
    format!("blob {{len: {}}}", v.len())
}