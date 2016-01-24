use std::collections::VecDeque;
use std::iter;
use std::char;

extern crate itertools;
use self::itertools::{Itertools, Unfold};

const DATA: &'static str = include_str!("input.txt");

pub fn main() -> Vec<String> {
    let s1 = part1(DATA).unwrap();
    let s2 = part2(DATA);
    vec![s1.to_string(), s2.to_string()]
}

fn part1(input: &str) -> Result<usize, &str> {
    // chars().count() instead of len() becuase we want the character count not the byte length
    // and decode of hex escapes produce code points above 127 that become multi-byte sequences
    // in a UTF-8 string
    input.lines()
         .map(|s| decode(s).map(|d| s.chars().count() - d.chars().count()))
         .collect::<Result<Vec<_>, _>>()
         .map(|v| {
             v.iter()
              .sum()
         })
}

fn part2(input: &str) -> usize {
    input.lines()
         .map(|s| encode(s).chars().count() - s.chars().count())
         .sum()
}

fn decode(input: &str) -> Result<String, &str> {
    let mut chrs = input.chars();

    if !(chrs.next().map_or(false, |c| c == '"') && chrs.next_back().map_or(false, |c| c == '"')) {
        return Err("bad string, not double quoted");
    }

    chrs.batching(|mut it| {
            // this bit gets a little hard to read as is returns an Option<Result<char, &str>>
            // The outer Option is required to indicate the end of the iterator (None),
            // but in the event of a parsing error we pass on Some(Err(_))
            // and the expected output is Some(Ok(char))
            match it.next() {
                // escaped character
                Some('\\') => {
                    match it.next() {
                        Some(c @ '\\') | Some(c @ '"') => Some(Ok(c)),
                        Some('x') => {
                            let a = it.next().and_then(|c| c.to_digit(16));
                            let b = it.next().and_then(|c| c.to_digit(16));
                            Some(a.and_then(|a| b.and_then(|b| char::from_u32(16 * a + b)))
                                  .ok_or("error in hex escape"))
                        }
                        _ => Some(Err("invalid escape")),
                    }
                }
                // non-escaped character
                Some(c) => Some(Ok(c)),
                // end of input
                None => None,
            }
        })
        .collect::<Result<String, _>>()
}

fn encode(input: &str) -> String {
    let escaped = Unfold::new((input.chars(), VecDeque::<char>::new()), |state| {
        let (ref mut it, ref mut backlog) = *state;

        backlog.pop_front().or_else(|| {
            match it.next() {
                Some('\\') => {
                    backlog.push_back('\\');
                    Some('\\')
                }
                Some('"') => {
                    backlog.push_back('"');
                    Some('\\')
                }
                // Not really needed for part 2, but why not
                // hex encode to 7-bit clean ASCII
                Some(c) if c > 127 as char => {
                    for c in format!("x{:x}", c as u32).chars() {
                        backlog.push_back(c);
                    }
                    Some('\\')
                }
                Some(c) => Some(c),
                None => None,
            }
        })
    });

    iter::once('"')
        .chain(escaped)
        .chain(iter::once('"'))
        .collect()
}

#[cfg(test)]
mod test {
    use super::{decode, encode, part1, part2};

    const EXAMPLE_DATA: [&'static str; 4] = [r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#];

    #[test]
    fn examples_1() {
        assert_eq!(decode(EXAMPLE_DATA[0]).map(|s| s.chars().count()), Ok(0));
        assert_eq!(decode(EXAMPLE_DATA[1]).map(|s| s.chars().count()), Ok(3));
        assert_eq!(decode(EXAMPLE_DATA[2]).map(|s| s.chars().count()), Ok(7));
        assert_eq!(decode(EXAMPLE_DATA[3]).map(|s| s.chars().count()), Ok(1));
        assert_eq!(part1(&EXAMPLE_DATA.join("\n")), Ok(12));
    }

    #[test]
    fn examples_2() {
        assert_eq!(encode(EXAMPLE_DATA[0]).chars().count(), 6);
        assert_eq!(encode(EXAMPLE_DATA[1]).chars().count(), 9);
        assert_eq!(encode(EXAMPLE_DATA[2]).chars().count(), 16);
        assert_eq!(encode(EXAMPLE_DATA[3]).chars().count(), 11);
        assert_eq!(part2(&EXAMPLE_DATA.join("\n")), 19);
    }
}
