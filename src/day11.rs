use std::ops::Not;

extern crate itertools;
use self::itertools::Itertools;

const DATA: &'static str = include_str!("../data/input_11.txt");

pub fn main() -> Vec<String> {
    let s1 = next_password(DATA.trim());
    let s2 = next_password(&s1);
    vec![s1, s2]
}

fn strbump(old: &str) -> String {
    let mut v = old.chars()
                   .rev()
                   .scan(1, |carry, c| {
                       let old = (c as u8) - ('a' as u8);
                       let next = (old + *carry) % 26;
                       if next < old {
                           *carry = 1;
                       } else {
                           *carry = 0;
                       }
                       Some(next + ('a' as u8))
                   })
                   .collect::<Vec<_>>();
    v.reverse();
    unsafe { String::from_utf8_unchecked(v) }
}

fn has_straight(input: &str) -> bool {
    input.chars()
         .collect::<Vec<_>>()
         .windows(3)
         .any(|w| (w[0] as u32 + 1 == w[1] as u32) && (w[1] as u32 + 1 == w[2] as u32))
}

fn no_confusing_letters(input: &str) -> bool {
    input.chars()
         .any(|c| c == 'i' || c == 'l' || c == 'o')
         .not()
}

fn has_two_pairs(input: &str) -> bool {
    input.chars()
         .collect::<Vec<_>>()
         .windows(2)
         .filter(|w| w[0] == w[1])
         .unique()
         .count() >= 2
}

fn next_password(passwd: &str) -> String {
    let mut next = strbump(passwd);
    while !(has_straight(&next) && no_confusing_letters(&next) && has_two_pairs(&next)) {
        next = strbump(&next)
    }
    next
}

#[cfg(test)]
mod test {
    use super::{has_straight, no_confusing_letters, has_two_pairs, next_password};

    #[test]
    fn examples() {
        assert_eq!(has_straight("hijklmmn"), true);
        assert_eq!(no_confusing_letters("hijklmmn"), false);
        assert_eq!(has_straight("abbceffg"), false);
        assert_eq!(has_two_pairs("abbceffg"), true);
        assert_eq!(has_two_pairs("abbcegjk"), false);
        assert_eq!(next_password("abcdefgh"), "abcdffaa");
        assert_eq!(next_password("ghijklmn"), "ghjaabcc");
    }
}
