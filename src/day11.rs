use std::ops::Not;

extern crate itertools;
use self::itertools::Itertools;

pub fn main(input: &str) -> Vec<String> {
    let s1 = next_password(input.trim());
    let s2 = next_password(&s1);
    vec![s1, s2]
}

fn strbump(old: &str) -> String {
    old.chars()
       .rev()
       .scan(1, |carry, c| {
           let old = (c as u8) - ('a' as u8);
           let next = (old + *carry) % 26;
           if next < old {
               *carry = 1;
           } else {
               *carry = 0;
           }
           Some((next + ('a' as u8)) as char)
       })
       .collect::<String>()
       .chars()
       .rev()
       .collect()
}

fn has_straight(input: &str) -> bool {
    input.as_bytes().windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2])
}

fn no_confusing_letters(input: &str) -> bool {
    input.chars().any(|c| c == 'i' || c == 'l' || c == 'o').not()
}

fn has_two_pairs(input: &str) -> bool {
    input.as_bytes().windows(2).filter(|w| w[0] == w[1]).unique().count() >= 2
}

fn next_password(passwd: &str) -> String {
    let mut next = strbump(passwd);
    while !(has_straight(&next) && no_confusing_letters(&next) && has_two_pairs(&next)) {
        next = strbump(&next)
    }
    return next;
}

#[test]
fn test_strbmp() {
    assert_eq!(strbump("xx"), "xy");
    assert_eq!(strbump("xy"), "xz");
    assert_eq!(strbump("xz"), "ya");
    assert_eq!(strbump("ya"), "yb");
    assert_eq!(has_straight("zabc"), true);
    assert_eq!(has_straight("zcde"), true);
    assert_eq!(has_straight("aabd"), false);
    assert_eq!(next_password("abcdefgh"), "abcdffaa");
    assert_eq!(next_password("ghijklmn"), "ghjaabcc");
}
