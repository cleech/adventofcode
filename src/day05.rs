use std::str;

const DATA: &'static str = include_str!("../data/input_5.txt");

pub fn main() -> Vec<String> {
    let s1 = nice_count(DATA);
    let s2 = nicer_count(DATA);
    vec![s1.to_string(), s2.to_string()]
}

fn nice(s: &str) -> bool {
    let has_three_vowels = |s: &str| {
        s.chars()
         .filter(|&c| {
             match c {
                 'a' | 'e' | 'i' | 'o' | 'u' => true,
                 _ => false,
             }
         })
         .count() >= 3
    };
    let has_doubles = |s: &str| {
        s.as_bytes()
         .windows(2)
         .any(|w| w[0] == w[1])
    };
    let has_bad_pairs = |s: &str| {
        s.as_bytes()
         .windows(2)
         .any(|w| {
             match unsafe { str::from_utf8_unchecked(w) } {
                 "ab" | "cd" | "pq" | "xy" => true,
                 _ => false,
             }
         })
    };
    has_three_vowels(s) && has_doubles(s) && !has_bad_pairs(s)
}

fn nice_count(input: &str) -> usize {
    input.lines()
         .filter(|line| nice(line))
         .count()
}

fn nicer(s: &str) -> bool {
    let has_double_pairs = |s: &str| {
        let bs = s.as_bytes();
        (0..bs.len() - 3).any(|n| {
            bs[n + 2..]
                .windows(2)
                .any(|w| w[0] == bs[n] && w[1] == bs[n + 1])
        })
    };
    let has_sandwich = |s: &str| {
        s.as_bytes()
         .windows(3)
         .any(|w| w[0] == w[2])
    };
    has_double_pairs(s) && has_sandwich(s)
}

fn nicer_count(input: &str) -> usize {
    input.lines()
         .filter(|line| nicer(line))
         .count()
}

#[cfg(test)]
mod test {
    use super::{nice_count, nicer_count};

    #[test]
    fn test_nice_string() {
        assert_eq!(nice_count("ugknbfddgicrmopn"), 1);
        assert_eq!(nice_count("aaa"), 1);
        assert_eq!(nice_count("jchzalrnumimnmhp"), 0);
        assert_eq!(nice_count("haegwjzuvuyypxyu"), 0);
        assert_eq!(nice_count("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn test_nicer_string() {
        assert_eq!(nicer_count("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(nicer_count("xxyxx"), 1);
        assert_eq!(nicer_count("uurcxstgmygtbstg"), 0);
        assert_eq!(nicer_count("eodomkazucvgmuy"), 0);
    }
}
