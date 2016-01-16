extern crate itertools;
use self::itertools::{Itertools, Unfold};
use std::iter;

const DATA: &'static str = include_str!("../data/input_8.txt");

pub fn main() -> Vec<String> {
    let s1 = part1(DATA).unwrap();
    let s2 = part2(DATA);
    vec![s1.to_string(), s2.to_string()]
}

fn part1(input: &str) -> Result<usize, &str> {
    // chars().count() instead of len() becuase we want the character count not the byte length
    // and decode of hex escapes produce 8-bit codepoints of some sort that become multi-byte
    // sequences in a UTF-8 string
    input.lines()
         .map(|s| decode(s).map(|d| s.chars().count() - d.chars().count()))
         .collect::<Result<Vec<_>, _>>()
         .map(|v| {
             v.iter()
              .sum()
         })
}

fn part2(input: &str) -> usize {
    input.lines().map(|s| encode(s).chars().count() - s.chars().count()).sum()
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
                            let a = match it.next() {
                                Some(c @ '0'...'9') => Ok(c as u8 - '0' as u8),
                                Some(c @ 'a'...'f') => Ok(c as u8 - 'a' as u8 + 10),
                                _ => Err("invalid char in hex escape"),
                            };
                            let b = match it.next() {
                                Some(c @ '0'...'9') => Ok(c as u8 - '0' as u8),
                                Some(c @ 'a'...'f') => Ok(c as u8 - 'a' as u8 + 10),
                                _ => Err("invalid char in hex escape"),
                            };
                            Some(a.and_then(|a| b.map(|b| (16 * a + b) as char)))
                        }
                        _ => Some(Err("invalid escape")),
                    }
                }
                // non-escaped character
                Some(c @ _) => Some(Ok(c)),
                // end of input
                None => None,
            }
        })
        .collect::<Result<String, _>>()
}

fn encode(input: &str) -> String {
    let escaped = Unfold::new((input.chars(), Vec::<char>::new()), |state| {
        let (ref mut it, ref mut backlog) = *state;

        backlog.pop().or_else(|| {
            match it.next() {
                Some('\\') => {
                    backlog.push('\\');
                    Some('\\')
                }
                Some('"') => {
                    backlog.push('"');
                    Some('\\')
                }
                // Not really needed for part 2, but why not
                // hex encode to 7-bit clean ASCII
                Some(c) if c > 127 as char => {
                    let a = match c as u8 >> 4 {
                        n @ 0...9 => (n + '0' as u8) as char,
                        n @ 0xa...0xf => ((n - 0xa) + 'a' as u8) as char,
                        _ => unreachable!(),
                    };
                    let b = match c as u8 & 0x0f {
                        n @ 0...9 => (n + '0' as u8) as char,
                        n @ 0xa...0xf => ((n - 0xa) + 'a' as u8) as char,
                        _ => unreachable!(),
                    };
                    backlog.push(b);
                    backlog.push(a);
                    backlog.push('x');
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

    #[test]
    fn test_day8() {
        assert_eq!(decode(r#""Hello""#), Ok(r"Hello".to_owned()));
        assert_eq!(decode(r#""\\""#), Ok(r"\".to_owned()));
        assert_eq!(decode(r#""\"""#), Ok(r#"""#.to_owned()));
        assert_eq!(decode(r#""\x27""#), Ok("'".to_owned()));
        assert_eq!(part1(&([r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#].join("\n"))),
                   Ok(12));
        assert_eq!(part2(&([r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#].join("\n"))),
                   19);
        assert_eq!(decode(r#""\xbb""#), Ok("\u{bb}".to_owned()));
        assert_eq!(encode("\u{bb}"), r#""\xbb""#);
    }
}
