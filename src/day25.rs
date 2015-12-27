pub fn main() -> Vec<String> {
    let mut cg = CodeGen::new();
    let index = coord_to_index(2981, 3075);
    let code = cg.nth(index).unwrap();
    vec![code.to_string()]
}

fn coord_to_index(row: usize, col: usize) -> usize {
    (1..(row + col - 1)).sum::<usize>() + col - 1
}

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
