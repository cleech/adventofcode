extern crate itertools;
use self::itertools::{Itertools, Unfold};
use std::iter;

pub fn main(input: &str) -> Vec<String> {
    let s1 = part1(input);
    let s2 = part2(input);
    vec![s1.to_string(), s2.to_string()]
}

fn part1(input: &str) -> usize {
    // chars().count() instead of .len() becuase
    // 'u8 as char' is creating some 2 byte chars in the utf8 string
    // so we want the character count not the byte length
    input.lines().map(|s| s.len() - decode(s).chars().count()).sum()
}

fn part2(input: &str) -> usize {
    // chars().count() instead of .len() becuase
    // 'u8 as char' is creating some 2 byte chars in the utf8 string
    // so we want the character count not the byte length
    input.lines().map(|s| encode(s).chars().count() - s.len()).sum()
}

fn decode(input: &str) -> String {
    let mut chrs = input.chars();

    if !(chrs.next().map_or(false, |c| c == '"') && chrs.next_back().map_or(false, |c| c == '"')) {
        panic!("bad string, not double quoted");
    }

    chrs.batching(|mut it| {
            match it.next() {
                Some('\\') => {
                    match it.next() {
                        c @ Some('\\') | c @ Some('"') => c,
                        Some('x') => {
                            let a = match it.next().unwrap() {
                                c @ '0'...'9' => c as u8 - '0' as u8,
                                c @ 'a'...'f' => c as u8 - 'a' as u8 + 10,
                                _ => panic!("bad first hex char"),
                            };
                            let b = match it.next().unwrap() {
                                c @ '0'...'9' => c as u8 - '0' as u8,
                                c @ 'a'...'f' => c as u8 - 'a' as u8 + 10,
                                _ => panic!("bad second hex char"),
                            };
                            Some((16 * a + b) as char)
                        }
                        _ => panic!("bad escape"),
                    }
                }
                c @ Some(_) => c,
                None => None,
            }
        })
        .collect()
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
                Some(c) if c > 127 as char => {
                    let a = match c as u8 >> 4 {
                        n @ 0...9 => (n + '0' as u8) as char,
                        n @ 0xa...0xf => ((n - 0xa) + 'a' as u8) as char,
                        _ => panic!("this should never happen"),
                    };
                    let b = match c as u8 & 0x0f {
                        n @ 0...9 => (n + '0' as u8) as char,
                        n @ 0xa...0xf => ((n - 0xa) + 'a' as u8) as char,
                        _ => panic!("this should never happen"),
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

#[test]
fn test_day8() {
    assert_eq!(decode(r#""Hello""#), r"Hello");
    assert_eq!(decode(r#""\\""#), r"\");
    assert_eq!(decode(r#""\"""#), r#"""#);
    assert_eq!(decode(r#""\x27""#), "'");
    assert_eq!(part1(&([r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#].join("\n"))), 12);
    assert_eq!(part2(&([r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#].join("\n"))), 19);
    assert_eq!(decode(r#""\xbb""#), "\u{bb}");
    assert_eq!(encode("\u{bb}"), r#""\xbb""#);
}
