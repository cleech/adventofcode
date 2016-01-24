use std::iter;

extern crate itertools;
use self::itertools::Itertools;

const DATA: &'static str = include_str!("../data/input_3.txt");

pub fn main() -> Vec<String> {
    let s1 = multi_santa(DATA, 1);
    let s2 = multi_santa(DATA, 2);
    vec![s1.to_string(), s2.to_string()]
}

fn multi_santa(input: &str, how_many: usize) -> usize {
    (0..how_many)
        .map(|n| {
            input.chars()
                 .skip(n)
                 .step(how_many)
                 .map(|c| {
                     match c {
                         '>' => (1, 0),
                         '<' => (-1, 0),
                         '^' => (0, 1),
                         'v' => (0, -1),
                         _ => panic!("invalid input"),
                     }
                 })
                 .scan((0, 0), |&mut (ref mut x, ref mut y), (dx, dy)| {
                     *x += dx;
                     *y += dy;
                     Some((*x, *y))
                 })
        })
        .fold(Box::new(iter::once((0, 0))) as Box<Iterator<Item = (i32, i32)>>,
              |itr, next_santa| Box::new(itr.chain(next_santa)))
        .unique()
        .count()
}

#[cfg(test)]
mod test {
    use super::multi_santa;

    #[test]
    fn examples_1() {
        assert_eq!(multi_santa(">", 1), 2);
        assert_eq!(multi_santa("^>v<", 1), 4);
        assert_eq!(multi_santa("^v^v^v^v^v", 1), 2);
    }

    #[test]
    fn examples_2() {
        assert_eq!(multi_santa("^v", 2), 3);
        assert_eq!(multi_santa("^>v<", 2), 3);
        assert_eq!(multi_santa("^v^v^v^v^v", 2), 11);
    }
}
