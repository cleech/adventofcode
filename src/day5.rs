use std::str::pattern::Pattern;

extern crate itertools;
use self::itertools::Itertools;

pub fn main(input: &str) -> Vec<String> {
    let s1 = nice_count(&input);
    vec![s1.to_string()]
}

fn nice_count(input: &str) -> usize {
    input.lines()
         .filter(|line| line.chars().filter(|c| c.is_contained_in("aeiou")).count() >= 3)
         .filter(|line| {
             line.chars()
                 .fold((false, None),
                       |(double, last), c| (double || last == Some(c), Some(c)))
                 .0
         })
         .filter(|line| {
             line.chars()
                 .fold((true, None), |(no_badpair, last), c| {
                     (no_badpair && (last != Some('x') || Some(c) != Some('y')) &&
                      (last != Some('a') || Some(c) != Some('b')) &&
                      (last != Some('c') || Some(c) != Some('d')) &&
                      (last != Some('p') || Some(c) != Some('q')),
                      Some(c))
                 })
                 .0
         })
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
    input.lines()
         .filter(|line| {
             line.chars()
                 .fold((false, None),
                       |(double, last), c| (double || last == Some(c), Some(c)))
                 .0
         })
         .count()
}

#[test]
fn test_new_nice_string() {
    assert_eq!(new_nice_count("qjhvhtzxzqqjkmpb"), 1);
    assert_eq!(new_nice_count("xxyxx"), 1);
    assert_eq!(new_nice_count("uurcxstgmygtbstg"), 0);
    assert_eq!(new_nice_count("eodomkazucvgmuy"), 0);
}
