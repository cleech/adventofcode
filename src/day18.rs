use std::fmt;
use std::mem;

const DATA: &'static str = include_str!("../data/input_18.txt");

pub fn main() -> Vec<String> {
    let mut life = Life::from_str(DATA);
    let lcount = life.nth(100).unwrap().lcount();

    let mut life = Life::from_str(DATA);
    life.stick(0, 0);
    life.stick(0, 99);
    life.stick(99, 0);
    life.stick(99, 99);
    let lcount2 = life.nth(100).unwrap().lcount();

    vec![lcount.to_string(), lcount2.to_string()]
}

struct Life {
    size: (usize, usize),
    arr: [bool; 100 * 100],
    stuck: Vec<(usize, usize)>,
}

impl Life {
    fn new() -> Life {
        Life {
            size: (100, 100),
            arr: [false; 100 * 100],
            stuck: Vec::new(),
        }
    }

    fn stick(&mut self, x: usize, y: usize) {
        self.stuck.push((x, y));
    }

    fn from_str(data: &str) -> Life {
        let mut life = Life::new();
        {
            for (l, c) in data.lines()
                              .enumerate()
                              .flat_map(move |(l, s)| {
                                  s.chars()
                                   .enumerate()
                                   .filter(|&(_, z)| z == '#')
                                   .map(move |(c, _)| (l, c))
                              }) {
                life.arr[l * life.size.0 + c] = true;
            }
        }
        life
    }

    fn to_string(&self) -> String {
        self.arr
            .chunks(self.size.0)
            .map(|l| {
                l.iter()
                 .map(|v| {
                     match *v {
                         true => '#',
                         false => '.',
                     }
                 })
                 .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut n = Vec::new();
        if x != 0 {
            if y != 0 {
                n.push((x - 1, y - 1));
            }
            n.push((x - 1, y));
            if y != 99 {
                n.push((x - 1, y + 1));
            }
        }
        if y != 0 {
            n.push((x, y - 1));
        }
        if y != 99 {
            n.push((x, y + 1));
        }
        if x != 99 {
            if y != 0 {
                n.push((x + 1, y - 1));
            }
            n.push((x + 1, y));
            if y != 99 {
                n.push((x + 1, y + 1));
            }
        }
        n
    }

    fn lcount(&self) -> usize {
        self.arr
            .iter()
            .filter(|&&b| b == true)
            .count()
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
        let mut next = [false; 100 * 100];

        for x in 0..100 {
            for y in 0..100 {
                let count = Life::neighbors(x, y)
                                .into_iter()
                                .map(|(x, y)| {
                                    let b = self.arr.get(x * self.size.0 + y);
                                    b.unwrap_or(&false).to_owned()
                                })
                                .filter(|b| *b == true)
                                .count();
                match (self.arr[x * self.size.0 + y], count) {
                    (true, 2) | (true, 3) => next[x * 100 + y] = true,
                    (true, _) => next[x * 100 + y] = false,
                    (false, 3) => next[x * 100 + y] = true,
                    (false, _) => next[x * 100 + y] = false,
                }
            }
        }
        for &(x, y) in self.stuck.iter() {
            next[x * 100 + y] = true;
        }

        mem::swap(&mut self.arr, &mut next);

        Some(Life {
            size: self.size,
            arr: next,
            stuck: self.stuck.clone(),
        })
    }
}
