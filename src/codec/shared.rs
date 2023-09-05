#[macro_export]
macro_rules! assert_section {
    ($data: ident, $s: ident) => {
        let mut buf = [0; $s.len()];
        $data.read_exact(&mut buf)?;
        if buf != $s.as_bytes() {
            return Err(ParseError::InvalidSection(
                $s,
                String::from_utf8_lossy(&buf).into_owned(),
            ));
        }
    };
}
#[macro_export]
macro_rules! skip {
    ($data: ident, $s: literal) => {{
        let mut skip_buf = [0; $s];
        $data.read_exact(&mut skip_buf)?;
    }};
}

#[macro_export]
macro_rules! read_primitive_vec {
    ($data: ident, $t: ty, $len: expr) => {{
        let mut res: Vec<$t> = vec![0; $len as usize];
        let b = unsafe {
            core::slice::from_raw_parts_mut(
                res.as_mut_ptr() as *mut u8,
                ($len as usize) * std::mem::size_of::<$t>(),
            )
        };
        $data.read_exact(b)?;
        res
    }};
}
#[macro_export]
macro_rules! dbg_str {
    ($data: ident, $s: expr) => {{
        let mut buf = vec![0; $s as usize];
        $data.read_exact(&mut buf);
        dbg!(String::from_utf8_lossy(&buf))
    }};
}
