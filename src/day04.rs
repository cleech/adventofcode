extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

const DATA: &'static str = include_str!("../data/input_4.txt");

pub fn main() -> Vec<String> {
    let s1 = find_number(DATA.trim(), 5);
    let s2 = find_number(DATA.trim(), 6);
    vec![s1.to_string(), s2.to_string()]
}

fn find_number(key: &str, zeros: usize) -> u32 {
    let mut md5 = Md5::new();
    let prefix = format!("{:01$}", 0, zeros);

    (1..)
        .find(|n| {
            md5.reset();
            md5.input_str(&key);
            md5.input_str(&n.to_string());
            md5.result_str().starts_with(&prefix)
        })
        .unwrap()
}

#[test]
#[ignore]
fn test_find_number() {
    assert_eq!(find_number("abcdef", 5), 609043);
    assert_eq!(find_number("pqrstuv", 5), 1048970);
}
