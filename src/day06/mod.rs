use std::str::FromStr;

const DATA: &'static str = include_str!("input.txt");

pub fn main() -> Vec<String> {
    let s1 = light_show(DATA);
    let s2 = bright_show(DATA);
    vec![s1.to_string(), s2.to_string()]
}

fn doit<F, L>(f: F, state: &mut [[L; 1000]; 1000], a: (usize, usize), b: (usize, usize))
    where F: Fn(&mut L)
{
    for x in a.0..(b.0 + 1) {
        for y in a.1..(b.1 + 1) {
            f(&mut state[x][y]);
        }
    }
}

struct Coord((usize, usize), (usize, usize));

impl FromStr for Coord {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Coord, &'static str> {
        if let (Some(a), Some(b), Some(c), Some(d)) = scan_fmt!(&line,
                                                                "{d},{d} through {d},{d}",
                                                                usize,
                                                                usize,
                                                                usize,
                                                                usize) {
            Ok(Coord((a, b), (c, d)))
        } else {
            Err("error parsing coordinates")
        }
    }
}

enum Cmnd {
    On((usize, usize), (usize, usize)),
    Off((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
}

impl FromStr for Cmnd {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Cmnd, &'static str> {
        let mut words = line.split_whitespace();

        match words.next() {
            Some("turn") => {
                match words.next() {
                    Some("on") => {
                        let c = words.collect::<String>().parse::<Coord>();
                        c.map(|c| Cmnd::On(c.0, c.1))
                    }
                    Some("off") => {
                        let c = words.collect::<String>().parse::<Coord>();
                        c.map(|c| Cmnd::Off(c.0, c.1))
                    }
                    _ => Err("error parsing command"),
                }
            }
            Some("toggle") => {
                let c = words.collect::<String>().parse::<Coord>();
                c.map(|c| Cmnd::Toggle(c.0, c.1))
            }
            _ => Err("error parsing command"),
        }
    }
}

fn light_show(input: &str) -> usize {
    let mut state = box [[false; 1000]; 1000];

    for line in input.lines() {
        if let Ok(cmnd) = line.parse() {
            match cmnd {
                Cmnd::On(a, b) => doit(|light: &mut bool| *light = true, &mut state, a, b),
                Cmnd::Off(a, b) => doit(|light: &mut bool| *light = false, &mut state, a, b),
                Cmnd::Toggle(a, b) => doit(|light: &mut bool| *light = !*light, &mut state, a, b),
            }
        }
    }
    state.iter()
         .flat_map(|inner| inner.iter())
         .filter(|&&on| on == true)
         .count()
}

fn bright_show(input: &str) -> u32 {
    let mut state = box [[0u32; 1000]; 1000];

    for line in input.lines() {
        if let Ok(cmnd) = line.parse() {
            match cmnd {
                Cmnd::On(a, b) => doit(|light: &mut u32| *light += 1, &mut state, a, b),
                Cmnd::Off(a, b) => {
                    doit(|light: &mut u32| {
                             if *light > 0 {
                                 *light -= 1
                             }
                         },
                         &mut state,
                         a,
                         b)
                }
                Cmnd::Toggle(a, b) => doit(|light: &mut u32| *light += 2, &mut state, a, b),
            }
        }
    }
    state.iter()
         .flat_map(|inner| inner.iter())
         .sum()
}

#[cfg(test)]
mod test {
    use super::{light_show, bright_show};

    #[test]
    fn examples_1() {
        assert_eq!(light_show("turn on 0,0 through 999,999"), 1_000_000);
        assert_eq!(light_show("toggle 0,0 through 999,0"), 1_000);
        assert_eq!(light_show("turn off 499,499 through 500,500"), 0);
    }

    #[test]
    fn examples_2() {
        assert_eq!(bright_show("turn on 0,0 through 0,0"), 1);
        assert_eq!(bright_show("toggle 0,0 through 999,999"), 2_000_000);
    }
}
