extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

use std::io::Write;

const DATA: &'static str = include_str!("../data/input_4.txt");

pub fn main() -> Vec<String> {
    let s1 = find_number(DATA.trim(), 5);
    let s2 = find_number(DATA.trim(), 6);
    vec![s1.to_string(), s2.to_string()]
}

fn leading_zeros(buf: &[u8], count: usize) -> bool {
    let (bytes, nibble) = (count / 2, count % 2);
    buf[..bytes].iter().all(|b| *b == 0) &&
    if nibble != 0 {
        (buf[bytes] & 0xf0) == 0
    } else {
        true
    }
}

fn find_number(key: &str, zeros: usize) -> u32 {
    let mut md5 = Md5::new();
    let k = key.as_bytes();
    let mut buf = Vec::with_capacity(256);
    let mut output = [0u8; 16];

    (1..)
        .find(|n| {
            // it would be nice if I could just roll-back to this state
            // instead of inputting the key each time
            md5.reset();
            md5.input(k);

            // use write! to a pre-allocated buffer
            buf.clear();
            write!(&mut buf, "{}", n).unwrap();
            md5.input(&buf);

            md5.result(&mut output);
            leading_zeros(&output, zeros)
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::find_number;

    #[test]
    fn examples() {
        assert_eq!(find_number("abcdef", 5), 609043);
        assert_eq!(find_number("pqrstuv", 5), 1048970);
    }
}
