extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::md5::Md5;

pub fn main(input: &str) -> Vec<String> {
    let s1 = find_number(input, 5);
    let s2 = find_number(input, 6);
    vec![s1.to_string(), s2.to_string()]
}

fn find_number(key: &str, zeros: usize) -> u32 {
    let mut md5 = Md5::new();
    let prefix = format!("{:01$}", 0, zeros);

    (1..)
        .map(|n| {
            let input = format!("{}{:?}", key, n);
            md5.reset();
            md5.input_str(&input);
            let output = md5.result_str();
            (n, output)
        })
        .find(|&(_, ref output)| output.starts_with(&prefix))
        .unwrap()
        .0
}

#[test]
#[ignore]
fn test_find_number() {
    assert_eq!(find_number("abcdef", 5), 609043);
    assert_eq!(find_number("pqrstuv", 5), 1048970);
}
