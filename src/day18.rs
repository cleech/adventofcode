use std::fmt;
use std::mem;

const DATA: &'static str = include_str!("../data/input_18.txt");

pub fn main() -> Vec<String> {

    let mut life = Life::from_str(DATA);
    let lcount = life.nth(100).unwrap().lcount();
    vec![format!("{}", lcount)]
}

struct Life([[bool; 100]; 100]);

impl Life {
    fn new() -> Life {
        Life([[false; 100]; 100])
    }

    fn from_str(data: &str) -> Life {
        let mut life = Life::new();
        {
            let Life(ref mut arr) = life;

            for (l, c) in data.lines()
                              .enumerate()
                              .flat_map(move |(l, s)| {
                                  s.chars()
                                   .enumerate()
                                   .filter(|&(_, z)| z == '#')
                                   .map(move |(c, _)| (l, c))
                              }) {
                arr[l][c] = true;
            }
        }
        life
    }

    fn to_string(&self) -> String {
        let &Life(ref arr) = self;
        arr.iter()
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
        let &Life(ref arr) = self;
        arr.iter()
           .flat_map(|x| x.iter().filter(|&&b| b == true))
           .count()
    }
}

impl Clone for Life {
    fn clone(&self) -> Life {
        let &Life(ref arr) = self;
        Life(*arr)
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
        let &mut Life(ref mut arr) = self;
        let mut next = [[false; 100]; 100];

        for x in 0..100 {
            for y in 0..100 {
                let count = Life::neighbors(x, y)
                                .into_iter()
                                .map(|(x, y)| {
                                    let a = arr.get(x);
                                    let b = a.and_then(|a| a.get(y));
                                    b.unwrap_or(&false).to_owned()
                                })
                                .filter(|b| *b == true)
                                .count();
                match (arr[x][y], count) {
                    (true, 2) | (true, 3) => next[x][y] = true,
                    (true, _) => next[x][y] = false,
                    (false, 3) => next[x][y] = true,
                    (false, _) => next[x][y] = false,
                }
            }
        }
        // force corners on
        {
            next[0][0] = true;
            next[0][99] = true;
            next[99][0] = true;
            next[99][99] = true;
        }
        mem::swap(arr, &mut next);
        //mem::replace(arr, next);
        Some(Life(next))
    }
}
