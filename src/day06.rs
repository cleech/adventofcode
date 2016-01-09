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

fn light_show(input: &str) -> usize {
    let mut state = box [[false; 1000]; 1000];

    let on = |light: &mut bool| *light = true;
    let off = |light: &mut bool| *light = false;
    let toggle = |light: &mut bool| *light = !*light;

    for line in input.lines() {
        let mut words = line.split_whitespace();

        let f: &(Fn(&mut bool)) = match words.next() {
            Some("turn") => {
                match words.next() {
                    Some("on") => &on,
                    Some("off") => &off,
                    _ => panic!("bad input"),
                }
            }
            Some("toggle") => &toggle,
            _ => panic!("bad input"),
        };

        if let (Some(a), Some(b), Some(c), Some(d)) = scan_fmt!(&words.collect::<String>(),
                                                                "{d},{d} through {d},{d}",
                                                                usize,
                                                                usize,
                                                                usize,
                                                                usize) {
            doit(f, &mut state, (a, b), (c, d));
        } else {
            panic!("bad input");
        }
    }
    state.iter()
         .flat_map(|inner| inner.iter())
         .filter(|&&on| on == true)
         .count()
}

fn bright_show(input: &str) -> u32 {
    let mut state = box [[0u32; 1000]; 1000];

    let on = |light: &mut u32| *light += 1;
    let off = |light: &mut u32| {
        if *light > 0 {
            *light -= 1
        }
    };
    let toggle = |light: &mut u32| *light += 2;

    for line in input.lines() {
        let mut words = line.split_whitespace();

        let f: &(Fn(&mut u32)) = match words.next() {
            Some("turn") => {
                match words.next() {
                    Some("on") => &on,
                    Some("off") => &off,
                    _ => panic!("bad input"),
                }
            }
            Some("toggle") => &toggle,
            _ => panic!("bad input"),
        };

        if let (Some(a), Some(b), Some(c), Some(d)) = scan_fmt!(&words.collect::<String>(),
                                                                "{d},{d} through {d},{d}",
                                                                usize,
                                                                usize,
                                                                usize,
                                                                usize) {
            doit(f, &mut state, (a, b), (c, d));
        } else {
            panic!("bad input");
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
