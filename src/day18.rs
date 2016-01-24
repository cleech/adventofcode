use std::fmt;
use std::mem;
use std::str::FromStr;

const DATA: &'static str = include_str!("../data/input_18.txt");

pub fn main() -> Vec<String> {
    let mut life = DATA.parse::<Life>().unwrap();
    let lcount = life.nth(100).unwrap().lcount();

    let mut life = DATA.parse::<Life>().unwrap();
    life.stick(0, 0);
    life.stick(0, 99);
    life.stick(99, 0);
    life.stick(99, 99);
    let lcount2 = life.nth(100).unwrap().lcount();

    vec![lcount.to_string(), lcount2.to_string()]
}

struct Life {
    size: (usize, usize),
    arr: Box<[bool]>,
    stuck: Vec<(usize, usize)>,
}

impl Life {
    fn new(x: usize, y: usize) -> Life {
        Life {
            size: (x, y),
            arr: vec![false; x * y].into_boxed_slice(),
            stuck: Vec::new(),
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<bool> {
        if (x < self.size.0) && (y < self.size.1) {
            self.arr.get(x * self.size.0 + y).map(&bool::to_owned)
        } else {
            None
        }
    }

    fn stick(&mut self, x: usize, y: usize) {
        self.arr[x * self.size.0 + y] = true;
        self.stuck.push((x, y));
    }

    fn to_string(&self) -> String {
        self.arr
            .chunks(self.size.0)
            .map(|l| {
                l.iter()
                 .map(|v| {
                     if *v {
                         '#'
                     } else {
                         '.'
                     }
                 })
                 .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        vec![(x - 1, y - 1),
             (x - 1, y),
             (x - 1, y + 1),
             (x, y - 1),
             (x, y + 1),
             (x + 1, y - 1),
             (x + 1, y),
             (x + 1, y + 1)]
    }

    fn lcount(&self) -> usize {
        self.arr
            .iter()
            .filter(|&&b| b == true)
            .count()
    }
}

impl FromStr for Life {
    type Err = ();

    fn from_str(data: &str) -> Result<Life, ()> {
        let mut buff = vec![false; data.len()];

        let mut lines = data.lines();
        let x = lines.next().map_or(0, str::len);
        if !lines.all(|s| s.len() == x) {
            return Err(());
        }
        let mut y = 0;

        for index in data.lines()
                         .inspect(|_| y = y + 1)
                         .enumerate()
                         .flat_map(|(l, s)| {
                             s.chars()
                              .enumerate()
                              .filter(|&(_, z)| z == '#')
                              .map(move |(c, _)| l * s.len() + c)
                         }) {
            buff[index] = true;
        }

        Ok(Life {
            arr: {
                buff.resize(x * y, false);
                buff.into_boxed_slice()
            },
            ..Life::new(x, y)
        })
    }
}

impl fmt::Display for Life {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&self.to_string(), f)
    }
}

impl Iterator for Life {
    type Item = Life;

    fn next(&mut self) -> Option<Life> {
        let mut next = vec![false; self.size.0 * self.size.1].into_boxed_slice();

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let count = self.neighbors(x, y)
                                .into_iter()
                                .map(|(x, y)| self.get(x, y).unwrap_or(false))
                                .filter(|b| *b == true)
                                .count();
                match (self.arr[x * self.size.0 + y], count) {
                    (true, 2) | (true, 3) => next[x * self.size.0 + y] = true,
                    (true, _) => next[x * self.size.0 + y] = false,
                    (false, 3) => next[x * self.size.0 + y] = true,
                    (false, _) => next[x * self.size.0 + y] = false,
                }
            }
        }
        for &(x, y) in &self.stuck {
            next[x * self.size.0 + y] = true;
        }

        mem::swap(&mut self.arr, &mut next);

        Some(Life {
            size: self.size,
            arr: next,
            stuck: self.stuck.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::Life;
    const EXAMPLE_DATA: [&'static str; 6] = [".#.#.#", "...##.", "#....#",
                                             "..#...", "#.#..#", "####.."];

    #[test]
    fn examples_1() {
        let life = EXAMPLE_DATA.join("\n").parse::<Life>().unwrap();
        let lcount = life.inspect(|l| println!("\n{}", l))
                         .nth(4)
                         .unwrap()
                         .lcount();
        assert_eq!(lcount, 4);
    }

    #[test]
    fn examples_2() {
        let mut life = EXAMPLE_DATA.join("\n").parse::<Life>().unwrap();
        life.stick(0, 0);
        life.stick(0, 5);
        life.stick(5, 0);
        life.stick(5, 5);
        let lcount = life.inspect(|l| println!("\n{}", l))
                         .nth(5)
                         .unwrap()
                         .lcount();
        assert_eq!(lcount, 17);
    }
}
