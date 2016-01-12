const DATA: &'static str = include_str!("../data/input_6.txt");

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

enum Cmnd {
    On((usize, usize), (usize, usize)),
    Off((usize, usize), (usize, usize)),
    Toggle((usize, usize), (usize, usize)),
}

fn parse_coords(line: &str) -> Result<((usize, usize), (usize, usize)), &'static str> {
    if let (Some(a), Some(b), Some(c), Some(d)) = scan_fmt!(&line,
                                                            "{d},{d} through {d},{d}",
                                                            usize,
                                                            usize,
                                                            usize,
                                                            usize) {
        Ok(((a, b), (c, d)))
    } else {
        Err("error parsing coordinates")
    }
}

fn parse_cmnd(line: &str) -> Result<Cmnd, &'static str> {
    let mut words = line.split_whitespace();

    match words.next() {
        Some("turn") => {
            match words.next() {
                Some("on") => {
                    let c = parse_coords(&words.collect::<String>());
                    c.map(|c| Cmnd::On(c.0, c.1))
                }
                Some("off") => {
                    let c = parse_coords(&words.collect::<String>());
                    c.map(|c| Cmnd::Off(c.0, c.1))
                }
                _ => Err("error parsing command"),
            }
        }
        Some("toggle") => {
            let c = parse_coords(&words.collect::<String>());
            c.map(|c| Cmnd::Toggle(c.0, c.1))
        }
        _ => Err("error parsing command"),
    }
}

fn light_show(input: &str) -> usize {
    let mut state = box [[false; 1000]; 1000];

    for line in input.lines() {
        if let Ok(cmnd) = parse_cmnd(line) {
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
        if let Ok(cmnd) = parse_cmnd(line) {
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
    fn test_light_show() {
        assert_eq!(light_show("turn on 0,0 through 999,999"), 1000 * 1000);
        assert_eq!(light_show("toggle 0,0 through 999,0"), 1000);
        assert_eq!(light_show("turn off 499,499 through 500,500"), 0);
    }

    #[test]
    fn test_bright_show() {
        assert_eq!(bright_show("turn on 0,0 through 0,0"), 1);
        assert_eq!(bright_show("toggle 0,0 through 999,999"), 2 * 1000 * 1000);
    }
}
