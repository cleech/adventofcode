pub fn main() -> Vec<String> {
    let index = coord_to_index(2981, 3075);
    let code = 20151125 * mod_exp(252533, index as u64, 33554393) % 33554393;
    vec![code.to_string()]
}

fn coord_to_index(row: usize, col: usize) -> usize {
    (1..(row + col - 1)).sum::<usize>() + col - 1
}

// calculation using modular exponentiation

fn mod_exp(base: u64, exponent: u64, modulus: u64) -> u64 {
    match exponent {
        0 => 1,
        1 => base,
        e if e % 2 == 0 => {
            let n = mod_exp(base, e >> 1, modulus);
            n * n % modulus
        }
        e => mod_exp(base, e - 1, modulus) * base % modulus,
    }
}

#[cfg(iterator)]
// this was my original iterator approach, much slower but generates all of the intermediate values
mod iterator {
    struct CodeGen {
        running: bool,
        last: u64,
    }

    impl CodeGen {
        const FIRST: u64 = 20151125;
        const MULT: u64 = 252533;
        const DIV: u64 = 33554393;

        fn new() -> CodeGen {
            CodeGen {
                running: false,
                last: CodeGen::FIRST,
            }
        }
    }

    impl Iterator for CodeGen {
        type Item = u64;

        fn next(&mut self) -> Option<u64> {
            if self.running {
                self.last = (self.last * CodeGen::MULT) % CodeGen::DIV;
            } else {
                self.running = true;
            }
            Some(self.last)
        }
    }
}
