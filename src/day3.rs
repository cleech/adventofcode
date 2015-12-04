use std::iter;

extern crate itertools;
use self::itertools::Itertools;

pub fn main(input: &str) -> Vec<String> {
    let s1 = multi_santa(&input, 1);
    let s2 = multi_santa(&input, 2);
    vec![s1.to_string(), s2.to_string()]
}

fn directions(c: char) -> (i32, i32) {
    match c {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, 1),
        'v' => (0, -1),
        _ => panic!("invalid input"),
    }
}

fn visit_houses<'a>(input: &'a str) -> Box<Iterator<Item = (i32, i32)> + 'a> {
    Box::new(input.chars()
                  .map(directions)
                  .scan((0, 0), |&mut (ref mut x, ref mut y), (dx, dy)| {
                      *x += dx;
                      *y += dy;
                      Some((*x, *y))
                  }))
    // .chain(iter::once((0, 0))))
}

fn multi_santa(input: &str, how_many: usize) -> usize {
    let santas = (0..how_many)
                     .map(|n| input.chars().skip(n).step(how_many).collect())
                     .collect::<Vec<String>>();
    return santas.iter()
                 .fold(Box::new(iter::once((0, 0))) as Box<Iterator<Item = (i32, i32)>>,
                       |itr, directions| Box::new(itr.chain(visit_houses(&directions))))
                 .unique()
                 .count();
}

#[test]
fn test_house_count() {
    assert_eq!(multi_santa(">", 1), 2);
    assert_eq!(multi_santa("^>v<", 1), 4);
    assert_eq!(multi_santa("^v^v^v^v^v", 1), 2);
}

#[test]
fn test_robo_santa() {
    assert_eq!(multi_santa("^v", 2), 3);
    assert_eq!(multi_santa("^>v<", 2), 3);
    assert_eq!(multi_santa("^v^v^v^v^v", 2), 11);
}
