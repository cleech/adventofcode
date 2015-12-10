extern crate itertools;
use self::itertools::{Itertools, Unfold};

pub fn main(input: &str) -> Vec<String> {
    let s1 = look_and_say_iterations(input, 40).len();
    let s2 = look_and_say_iterations(input, 50).len();
    vec![s1.to_string(), s2.to_string()]
}

fn look_and_say(input: &str) -> String {
    input.chars()
         .batching(|mut it| {
             match it.next() {
                 None => None,
                 Some(c) => {
                     let n: usize = it.take_while_ref(|&next| next == c).count() + 1;
                     let mut s = n.to_string();
                     s.push(c);
                     Some(s)
                 }
             }
         })
         .collect::<String>()
}

fn look_and_say_iterations(input: &str, count: usize) -> String {
    let seq = Unfold::new(input.to_owned(), |state| {
        let next = look_and_say(state);
        *state = next.clone();
        Some(next)
    });
    seq.skip(count - 1).next().unwrap()
}

#[test]
fn test_look_and_say() {
    assert_eq!(look_and_say("1"), "11");
    assert_eq!(look_and_say("11"), "21");
    assert_eq!(look_and_say("21"), "1211");
    assert_eq!(look_and_say("1211"), "111221");
    assert_eq!(look_and_say("111221"), "312211");
}

