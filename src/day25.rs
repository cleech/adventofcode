const DATA: &'static str = include_str!("../data/input_25.txt");
const SEED: u64 = 20151125;
const BASE: u64 = 252533;
const MODULUS: u64 = 33554393;

pub fn main() -> Vec<String> {
    if let Some((row, col)) = parse_input(DATA.trim()) {
        vec![code_lookup(row, col).to_string()]
    } else {
        vec![]
    }
}

fn parse_input(input: &str) -> Option<(u64, u64)> {
    match &input.split_whitespace().collect::<Vec<_>>()[..] {
        [.., "row", r, "column", c] => {
            r.trim_matches(',')
             .parse::<u64>()
             .and_then(|r| {
                 c.trim_matches('.')
                  .parse::<u64>()
                  .map(|c| (r, c))
             })
             .ok()
        }
        _ => None,
    }
}

fn code_lookup(row: u64, col: u64) -> u64 {
    let index = coord_to_index(row, col);
    SEED * mod_exp(BASE, index, MODULUS) % MODULUS
}

fn coord_to_index(row: u64, col: u64) -> u64 {
    (1..(row + col - 1)).sum::<u64>() + col - 1
}

// modular exponentiation by repeated squaring
fn mod_exp(base: u64, exponent: u64, modulus: u64) -> u64 {
    // tail recursive version, with accumulating multiplier argument
    fn mod_exp_inner(base: u64, exponent: u64, modulus: u64, mult: u64) -> u64 {
        match exponent {
            0 => mult,
            1 => base * mult % modulus,
            e if e % 2 == 0 => mod_exp_inner(base * base % modulus, e >> 1, modulus, mult),
            e => {
                mod_exp_inner(base * base % modulus,
                              (e - 1) >> 1,
                              modulus,
                              base * mult % modulus)
            }
        }
    }
    mod_exp_inner(base, exponent, modulus, 1)
}

#[cfg(test)]
mod test {
    use super::{coord_to_index, code_lookup};

    #[test]
    fn examples() {
        assert_eq!(coord_to_index(1, 1), 0);
        assert_eq!(code_lookup(1, 1), 20151125);
        assert_eq!(coord_to_index(2, 1), 1);
        assert_eq!(code_lookup(2, 1), 31916031);
        assert_eq!(coord_to_index(1, 2), 2);
        assert_eq!(code_lookup(1, 2), 18749137);
    }
}

// this was my original iterator approach, much slower but generates all of the intermediate values
#[cfg(day25_iterator)]
mod iterator {
    use super::{SEED, BASE, MODULUS};

    struct CodeGen {
        running: bool,
        last: u64,
    }

    impl CodeGen {
        fn new() -> CodeGen {
            CodeGen {
                running: false,
                last: SEED,
            }
        }
    }

    impl Iterator for CodeGen {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            if self.running {
                self.last = (self.last * BASE) % MODULUS;
            } else {
                self.running = true;
            }
            Some(self.last)
        }
    }
}
