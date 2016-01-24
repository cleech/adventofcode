use std::str::FromStr;

const DATA: &'static str = include_str!("input.txt");

pub fn main() -> Vec<String> {
    let instructions = parse_program(DATA).unwrap();

    let mut machine = Machine::new();
    machine.run(&instructions);
    let s1 = machine.b.to_string();

    let mut machine = Machine { a: 1, ..Machine::new() };
    machine.run(&instructions);
    let s2 = machine.b.to_string();

    vec![s1, s2]
}

#[derive(Clone,Copy)]
enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Register, ()> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            _ => Err(()),
        }
    }
}

enum Instr {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

impl FromStr for Instr {
    type Err = ();

    fn from_str(s: &str) -> Result<Instr, ()> {
        match &s.split_whitespace().collect::<Vec<_>>()[..] {
            ["hlf", r] => r.parse::<Register>().map(Instr::Hlf),
            ["tpl", r] => r.parse::<Register>().map(Instr::Tpl),
            ["inc", r] => r.parse::<Register>().map(Instr::Inc),
            ["jmp", o ] => o.parse::<i32>().map_err(|_| ()).map(Instr::Jmp),
            ["jie", r, o ] => {
                r.trim_right_matches(',')
                 .parse::<Register>()
                 .and_then(|r| {
                     o.parse::<i32>()
                      .map_err(|_| ())
                      .map(|o| Instr::Jie(r, o))
                 })
            }
            ["jio", r, o ] => {
                r.trim_right_matches(',')
                 .parse::<Register>()
                 .and_then(|r| {
                     o.parse::<i32>()
                      .map_err(|_| ())
                      .map(|o| Instr::Jio(r, o))
                 })
            }
            _ => Err(()),
        }
    }
}

type Program = Vec<Instr>;

fn parse_program(s: &str) -> Result<Program, ()> {
    s.lines()
     .map(|l| l.parse::<Instr>())
     .collect::<Result<Vec<_>, _>>()
}

struct Machine {
    a: u32,
    b: u32,
    pc: i32,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            a: 0,
            b: 0,
            pc: 0,
        }
    }

    // apply a function to a register
    fn rmap<F, T>(&mut self, r: Register, f: F) -> T
        where F: Fn(&mut u32) -> T
    {
        match r {
            Register::A => f(&mut self.a),
            Register::B => f(&mut self.b),
        }
    }

    fn run(&mut self, instr: &[Instr]) {
        loop {
            if let Some(i) = instr.get(self.pc as usize) {
                match *i {
                    Instr::Hlf(r) => {
                        self.rmap(r, |r| *r = *r >> 1);
                        self.pc += 1;
                    }
                    Instr::Tpl(r) => {
                        self.rmap(r, |r| *r = *r * 3);
                        self.pc += 1;
                    }
                    Instr::Inc(r) => {
                        self.rmap(r, |r| *r = *r + 1);
                        self.pc += 1;
                    }
                    Instr::Jmp(o) => self.pc += o,
                    Instr::Jie(r, o) => {
                        if self.rmap(r, |r| *r & 1 == 0) {
                            self.pc += o
                        } else {
                            self.pc += 1
                        }
                    }
                    Instr::Jio(r, o) => {
                        if self.rmap(r, |r| *r == 1) {
                            self.pc += o
                        } else {
                            self.pc += 1
                        }
                    }
                }
            } else {
                return;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Machine, parse_program};

    #[test]
    fn examples() {
        let source = ["inc a", "jio a, +2", "tpl a", "inc a"].join("\n");
        let instructions = parse_program(&source).unwrap();
        let mut machine = Machine::new();
        machine.run(&instructions);
        assert_eq!(machine.a, 2);
    }
}
