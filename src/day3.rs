use std::iter;

extern crate itertools;
use self::itertools::Itertools;

pub fn main(input: &str) -> Vec<String> {
    let s1 = house_count(&input);
    let s2 = robo_santa(&input);
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

fn house_count(input: &str) -> usize {
    input.chars()
         .map(directions)
         .scan((0, 0), |&mut (ref mut x, ref mut y), (dx, dy)| {
             *x += dx;
             *y += dy;
             Some((*x, *y))
         })
         .chain(iter::once((0, 0)))
         .unique()
         .count()
}

#[test]
fn test_house_count() {
    assert_eq!(house_count(">"), 2);
    assert_eq!(house_count("^>v<"), 4);
    assert_eq!(house_count("^v^v^v^v^v"), 2);
}

fn robo_santa(input: &str) -> usize {
    let odd = input.chars().step(2);
    let even = input.chars().skip(1).step(2);

    odd.map(directions)
       .scan((0, 0), |&mut (ref mut x, ref mut y), (dx, dy)| {
           *x += dx;
           *y += dy;
           Some((*x, *y))
       })
       .chain(even.map(directions)
                  .scan((0, 0), |&mut (ref mut x, ref mut y), (dx, dy)| {
                      *x += dx;
                      *y += dy;
                      Some((*x, *y))
                  }))
       .chain(iter::once((0, 0)))
       .unique()
       .count()
}

#[test]
fn test_robo_santa() {
    assert_eq!(robo_santa("^v"), 3);
    assert_eq!(robo_santa("^>v<"), 3);
    assert_eq!(robo_santa("^v^v^v^v^v"), 11);
}
