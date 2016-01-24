const DATA: &'static str = include_str!("input.txt");

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
        s.chars()
         .collect::<Vec<_>>()
         .windows(2)
         .any(|w| w[0] == w[1])
    };
    let has_bad_pairs = |s: &str| {
        s.chars()
         .collect::<Vec<_>>()
         .windows(2)
         .any(|w| {
             match w {
                 ['a', 'b'] | ['c', 'd'] | ['p','q'] | ['x', 'y'] => true,
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
        let bs = s.chars().collect::<Vec<_>>();
        (0..bs.len() - 3).any(|n| {
            bs[n + 2..]
                .windows(2)
                .any(|w| w[0] == bs[n] && w[1] == bs[n + 1])
        })
    };
    let has_sandwich = |s: &str| {
        s.chars()
         .collect::<Vec<_>>()
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
    use super::{nice, nicer};

    #[test]
    fn examples_1() {
        assert_eq!(nice("ugknbfddgicrmopn"), true);
        assert_eq!(nice("aaa"), true);
        assert_eq!(nice("jchzalrnumimnmhp"), false);
        assert_eq!(nice("haegwjzuvuyypxyu"), false);
        assert_eq!(nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn examples_2() {
        assert_eq!(nicer("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(nicer("xxyxx"), true);
        assert_eq!(nicer("uurcxstgmygtbstg"), false);
        assert_eq!(nicer("eodomkazucvgmuy"), false);
    }
}
