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
    let mut vec = input.chars()
                       .map(directions)
                       .scan((0, 0), |&mut (ref mut x, ref mut y), (dx, dy)| {
                           *x += dx;
                           *y += dy;
                           Some((*x, *y))
                       })
                       .collect::<Vec<(i32, i32)>>();
    // .inspect(|&(x, y)| println!("{}, {}", x, y))
    vec.push((0, 0));
    vec.sort();
    vec.dedup();
    vec.len()
}

#[test]
fn test_house_count() {
    assert_eq!(house_count(">"), 2);
    assert_eq!(house_count("^>v<"), 4);
    assert_eq!(house_count("^v^v^v^v^v"), 2);
}

fn robo_santa(input: &str) -> usize {
    let map = input.chars()
                   .map(directions)
                   .scan((0 as usize, 0 as i32, 0 as i32),
                         |&mut (ref mut index, _, _), (dx, dy)| {
                             *index += 1;
                             Some((*index, dx, dy))
                         });

    let (even, odd): (Vec<(usize, i32, i32)>, Vec<(usize, i32, i32)>) =
        map.partition(|&(index, _, _)| index % 2 == 0);

    let mut vec = even.iter()
                      .map(|&(_, x, y)| (x, y))
                      .scan((0, 0), |&mut (ref mut x, ref mut y), (dx, dy)| {
                          *x += dx;
                          *y += dy;
                          Some((*x, *y))
                      })
                      .collect::<Vec<(i32, i32)>>();

    let vec2 = odd.iter()
                  .map(|&(_, x, y)| (x, y))
                  .scan((0, 0), |&mut (ref mut x, ref mut y), (dx, dy)| {
                      *x += dx;
                      *y += dy;
                      Some((*x, *y))
                  })
                  .collect::<Vec<(i32, i32)>>();

    vec.push_all(&vec2);
    vec.push((0, 0));
    vec.sort();
    vec.dedup();
    vec.len()
}

#[test]
fn test_robo_santa() {
    assert_eq!(robo_santa("^v"), 3);
    assert_eq!(robo_santa("^>v<"), 3);
    assert_eq!(robo_santa("^v^v^v^v^v"), 11);
}
