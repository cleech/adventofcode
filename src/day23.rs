const DATA: &'static str = include_str!("../data/input_23.txt");

pub fn main() -> Vec<String> {
    let mut part1 = Program::compile(DATA).unwrap();
    part1.run();

    let mut part2 = Program::compile(DATA).unwrap();
    part2.a = 1;
    part2.run();

    vec![part1.b.to_string(), part2.b.to_string()]
}

#[cfg(test)]
mod test {
    use super::Program;

    #[test]
    fn test_day23() {
        let source = ["inc a", "jio a, +2", "tpl a", "inc a"].join("\n");
        let mut p = Program::compile(&source).unwrap();
        p.run();
        assert_eq!(p.a, 2);
    }
}

#[derive(Debug)]
enum Register {
    A,
    B,
}

#[derive(Debug)]
enum Instr {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

#[derive(Debug)]
struct Program {
    a: u32,
    b: u32,
    pc: i32,
    instr: Vec<Instr>,
}

impl Program {
    fn new(instr: Vec<Instr>) -> Program {
        Program {
            a: 0,
            b: 0,
            pc: 0,
            instr: instr,
        }
    }

    fn compile(source: &str) -> Result<Program, &'static str> {
        source.lines()
              .map(|l| {
                  let mut iter = l.split_whitespace();
                  match iter.next() {
                      Some("hlf") => {
                          iter.next()
                              .and_then(|r| {
                                  match r { 
                                      "a" => Some(Instr::Hlf(Register::A)),
                                      "b" => Some(Instr::Hlf(Register::B)),
                                      _ => None,
                                  }
                              })
                      }
                      Some("tpl") => {
                          iter.next()
                              .and_then(|r| {
                                  match r { 
                                      "a" => Some(Instr::Tpl(Register::A)),
                                      "b" => Some(Instr::Tpl(Register::B)),
                                      _ => None,
                                  }
                              })
                      }
                      Some("inc") => {
                          iter.next()
                              .and_then(|r| {
                                  match r { 
                                      "a" => Some(Instr::Inc(Register::A)),
                                      "b" => Some(Instr::Inc(Register::B)),
                                      _ => None,
                                  }
                              })
                      }
                      Some("jmp") => {
                          iter.next()
                              .and_then(|o| o.parse::<i32>().ok())
                              .map(Instr::Jmp)
                      }
                      Some("jie") => {
                          iter.next()
                              .and_then(|r| {
                                  match r { 
                                      "a," => Some(Register::A),
                                      "b," => Some(Register::B),
                                      _ => None,
                                  }
                              })
                              .and_then(|r| {
                                  iter.next()
                                      .and_then(|o| o.parse::<i32>().ok())
                                      .map(|o| Instr::Jie(r, o))
                              })
                      }
                      Some("jio") => {
                          iter.next()
                              .and_then(|r| {
                                  match r { 
                                      "a," => Some(Register::A),
                                      "b," => Some(Register::B),
                                      _ => None,
                                  }
                              })
                              .and_then(|r| {
                                  iter.next()
                                      .and_then(|o| o.parse::<i32>().ok())
                                      .map(|o| Instr::Jio(r, o))
                              })
                      }
                      _ => None,
                  }
              })
              .collect::<Option<Vec<_>>>()
              .ok_or("bad program")
              .map(Program::new)
    }

    fn run(&mut self) {
        loop {
            let i = self.instr.get(self.pc as usize);
            if i.is_none() {
                return;
            }
            // println!("pc: {}, a: {} b: {}", self.pc, self.a, self.b);
            match *i.unwrap() {
                Instr::Hlf(Register::A) => {
                    self.a = self.a >> 1;
                    self.pc += 1;
                }
                Instr::Hlf(Register::B) => {
                    self.b = self.b >> 1;
                    self.pc += 1;
                }
                Instr::Tpl(Register::A) => {
                    self.a = self.a * 3;
                    self.pc += 1;
                }
                Instr::Tpl(Register::B) => {
                    self.b = self.b * 3;
                    self.pc += 1;
                }
                Instr::Inc(Register::A) => {
                    self.a += 1;
                    self.pc += 1;
                }
                Instr::Inc(Register::B) => {
                    self.b += 1;
                    self.pc += 1;
                }
                Instr::Jmp(o) => self.pc += o,
                Instr::Jie(Register::A, o) => {
                    if (self.a & 1) == 0 {
                        self.pc += o
                    } else {
                        self.pc += 1
                    }
                }
                Instr::Jie(Register::B, o) => {
                    if (self.b & 1) == 0 {
                        self.pc += o
                    } else {
                        self.pc += 1
                    }
                }
                Instr::Jio(Register::A, o) => {
                    if self.a == 1 {
                        self.pc += o
                    } else {
                        self.pc += 1
                    }
                }
                Instr::Jio(Register::B, o) => {
                    if self.b == 1 {
                        self.pc += o
                    } else {
                        self.pc += 1
                    }
                }
            }
        }
    }
}
