extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

pub fn main(input: &str) -> Vec<String> {
    let s1 = find_number(input, 5);
    let s2 = find_number(input, 6);
    vec![s1.to_string(), s2.to_string()]
}

fn find_number(key: &str, zeros: usize) -> u32 {
    let mut sh = Md5::new();
    for n in 1.. {
        let input = format!("{}{}", key, n);

        sh.input_str(&input);
        let output = sh.result_str();
        sh.reset();

        if &output[..zeros] == format!("{:01$}", 0, zeros) {
            return n;
        }
    }
    0
}

#[test]
#[ignore]
fn test_find_number() {
    assert_eq!(find_number("abcdef", 5), 609043);
    assert_eq!(find_number("pqrstuv", 5), 1048970);
}
