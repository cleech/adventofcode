extern crate itertools;
use self::itertools::Itertools;

const DATA: &'static str = include_str!("../data/input_10.txt");

pub fn main() -> Vec<String> {
    let s1 = look_and_say_iterations(DATA.trim(), 40).len();
    let s2 = look_and_say_iterations(DATA.trim(), 50).len();
    vec![s1.to_string(), s2.to_string()]
}

fn look_and_say(input: &str) -> String {
    // worst case growth is x2 (one one, one two, etc.)
    let mut raw = Vec::with_capacity(input.len() * 2);
    for (n, c) in input.chars()
                       .batching(|mut it| {
                           match it.next() {
                               None => None,
                               Some(c) => {
                                   let n = it.take_while_ref(|&next| next == c).count() + 1;
                                   Some((n, c))
                               }
                           }
                       }) {
        // n should never be more than 3 (just part of how the maths work out)
        // so this is correct as it only ever generates one char
        raw.push(n as u8 + '0' as u8);
        raw.push(c as u8);
    }
    raw.shrink_to_fit();
    // trust that we generated valid utf8 :)
    unsafe { String::from_utf8_unchecked(raw) }
}

fn look_and_say_iterations(input: &str, count: usize) -> String {
    let mut tmp = input.to_owned();
    for _ in 0..count {
        tmp = look_and_say(&tmp);
    }
    tmp
}

#[cfg(test)]
mod test {
    use super::look_and_say;

    #[test]
    fn examples() {
        assert_eq!(look_and_say("1"), "11");
        assert_eq!(look_and_say("11"), "21");
        assert_eq!(look_and_say("21"), "1211");
        assert_eq!(look_and_say("1211"), "111221");
        assert_eq!(look_and_say("111221"), "312211");
    }
}
