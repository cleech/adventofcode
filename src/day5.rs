extern crate pcre;
use self::pcre::Pcre;

const DATA: &'static str = include_str!("../data/input_5.txt");

pub fn main() -> Vec<String> {
    let s1 = nice_count(DATA);
    let s2 = new_nice_count(DATA);
    vec![s1.to_string(), s2.to_string()]
}

fn nice_count(input: &str) -> usize {
    let vowels = Pcre::compile(r"[aeiou]").unwrap();
    let has_double = Pcre::compile(r"(.)\1").unwrap();
    let bad_pairs = Pcre::compile(r"(ab|cd|pq|xy)").unwrap();

    input.lines()
         .filter(|line| vowels.matches(line).count() >= 3)
         .filter(|line| has_double.exec(line).is_some())
         .filter(|line| bad_pairs.exec(line).is_none())
         .count()
}

#[test]
fn test_nice_string() {
    assert_eq!(nice_count("ugknbfddgicrmopn"), 1);
    assert_eq!(nice_count("aaa"), 1);
    assert_eq!(nice_count("jchzalrnumimnmhp"), 0);
    assert_eq!(nice_count("haegwjzuvuyypxyu"), 0);
    assert_eq!(nice_count("dvszwmarrgswjxmb"), 0);
}

fn new_nice_count(input: &str) -> usize {
    let pairs = Pcre::compile(r"(..).*\1").unwrap();
    let sandwich = Pcre::compile(r"(.).\1").unwrap();

    input.lines()
         .filter(|line| pairs.exec(line).is_some())
         .filter(|line| sandwich.exec(line).is_some())
         .count()
}

#[test]
fn test_new_nice_string() {
    assert_eq!(new_nice_count("qjhvhtzxzqqjkmpb"), 1);
    assert_eq!(new_nice_count("xxyxx"), 1);
    assert_eq!(new_nice_count("uurcxstgmygtbstg"), 0);
    assert_eq!(new_nice_count("eodomkazucvgmuy"), 0);
}
