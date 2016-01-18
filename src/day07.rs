use std::str::FromStr;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Wire(pub String);

#[derive(PartialEq, Eq, Hash, Clone)]
enum Source {
    Pattern(u16),
    Wire(Wire),
}

impl FromStr for Source {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Source, Self::Err> {
        match s {
            s if s.is_empty() => Err("empty identifier"),
            s if (|s: &str| s.chars().all(|c| c.is_digit(10)))(s) => {
                s.parse::<u16>().map_err(|_| "parse error").map(Source::Pattern)
            }
            s if (|s: &str| s.chars().all(|c| c.is_alphabetic()))(s) => {
                return Ok(Source::Wire(Wire(s.to_owned())))
            }
            _ => Err("unknown identifier format"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Gate {
    Pass {
        src: Source,
        out: Source,
    },
    Not {
        src: Source,
        out: Source,
    },
    And {
        a: Source,
        b: Source,
        out: Source,
    },
    Or {
        a: Source,
        b: Source,
        out: Source,
    },
    LShift {
        src: Source,
        shift: u16,
        out: Source,
    },
    RShift {
        src: Source,
        shift: u16,
        out: Source,
    },
}

impl FromStr for Gate {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Gate, Self::Err> {
        match &s.split_whitespace().collect::<Vec<_>>()[..] { 
            [a, "->", o] => {
                a.parse::<Source>()
                 .and_then(|a| {
                     o.parse::<Source>()
                      .map(|o| Gate::Pass { src: a, out: o })
                 })
            }
            ["NOT", a, "->", o] => {
                a.parse::<Source>()
                 .and_then(|a| {
                     o.parse::<Source>()
                      .map(|o| Gate::Not { src: a, out: o })
                 })
            }
            [a, "AND", b, "->", o] => {
                a.parse::<Source>()
                 .and_then(|a| {
                     b.parse::<Source>()
                      .and_then(|b| {
                          o.parse::<Source>()
                           .map(|o| {
                               Gate::And {
                                   a: a,
                                   b: b,
                                   out: o,
                               }
                           })
                      })
                 })
            }
            [a, "OR", b, "->", o] => {
                a.parse::<Source>()
                 .and_then(|a| {
                     b.parse::<Source>()
                      .and_then(|b| {
                          o.parse::<Source>()
                           .map(|o| {
                               Gate::Or {
                                   a: a,
                                   b: b,
                                   out: o,
                               }
                           })
                      })
                 })
            }
            [a, "LSHIFT", n, "->", o] => {
                a.parse::<Source>()
                 .and_then(|a| {
                     n.parse::<u16>()
                      .map_err(|_| "parse error")
                      .and_then(|n| {
                          o.parse::<Source>()
                           .map(|o| {
                               Gate::LShift {
                                   src: a,
                                   shift: n,
                                   out: o,
                               }
                           })
                      })
                 })
            }
            [a, "RSHIFT", n, "->", o] => {
                a.parse::<Source>()
                 .and_then(|a| {
                     n.parse::<u16>()
                      .map_err(|_| "parse error")
                      .and_then(|n| {
                          o.parse::<Source>()
                           .map(|o| {
                               Gate::RShift {
                                   src: a,
                                   shift: n,
                                   out: o,
                               }
                           })
                      })
                 })
            }
            _ => Err("bad input"),
        }
    }
}

impl Gate {
    fn out(&self) -> Source {
        match *self {
            Gate::Pass{ ref out, ..} => out,
            Gate::Not{ ref out, ..} => out,
            Gate::And{ ref out, ..} => out,
            Gate::Or{ ref out, ..} => out,
            Gate::LShift{ ref out, ..} => out,
            Gate::RShift{ ref out, ..} => out,
        }
        .clone()
    }

    fn eval(&self, c: &mut Circuit) -> Option<u16> {
        match *self {
            Gate::Pass{ ref src, .. } => c.eval(src),
            Gate::Not{ ref src, .. } => c.eval(src).map(|v| !v),
            Gate::And{ ref a, ref b, .. } => {
                match (c.eval(a), c.eval(b)) {
                    (None, _) => None,
                    (_, None) => None,
                    (Some(lhs), Some(rhs)) => Some(lhs & rhs),
                }
            }
            Gate::Or{ ref a, ref b, .. } => {
                match (c.eval(a), c.eval(b)) {
                    (None, _) => None,
                    (_, None) => None,
                    (Some(lhs), Some(rhs)) => Some(lhs | rhs),
                }
            }
            Gate::LShift{ ref src, shift, .. } => c.eval(src).map(|v| v << shift),
            Gate::RShift{ ref src, shift, .. } => c.eval(src).map(|v| v >> shift),
        }
    }
}

struct Circuit {
    gates: HashMap<Source, Gate>,
    cache: HashMap<Source, u16>,
}

impl Circuit {
    fn from_str(input: &str) -> Circuit {
        let mut circuit = HashMap::new();
        let cache = HashMap::new();

        for line in input.lines() {
            let gate = Gate::from_str(line).unwrap();
            circuit.insert(gate.out(), gate);
        }
        Circuit {
            gates: circuit,
            cache: cache,
        }
    }

    fn eval(&mut self, wire: &Source) -> Option<u16> {
        match *wire {
            Source::Pattern(p) => Some(p),
            ref s @ Source::Wire(_) => {
                if self.cache.contains_key(s) {
                    self.cache.get(s).cloned()
                } else {
                    let v = self.gates.get(s).cloned().and_then(|g| g.eval(self));
                    self.cache.insert(s.clone(), v.unwrap());
                    v
                }
            }
        }
    }

    fn force(&mut self, s: &Source, v: u16) {
        self.cache.clear();
        self.cache.insert(s.clone(), v);
    }
}

fn run_circuit(input: &str, target: &str) -> u16 {
    let mut c = Circuit::from_str(input);
    c.eval(&Source::Wire(Wire(target.to_owned()))).unwrap()
}

fn part2(input: &str, target: &str, force: &str, v: u16) -> u16 {
    let mut c = Circuit::from_str(input);
    c.force(&Source::Wire(Wire(force.to_owned())), v);
    c.eval(&Source::Wire(Wire(target.to_owned()))).unwrap()
}

#[cfg(test)]
mod test {
    use super::{Circuit, Source, Wire};

    #[test]
    fn test_circuit() {
        let cs = ["123 -> x",
                  "456 -> y",
                  "x AND y -> d",
                  "x OR y -> e",
                  "x LSHIFT 2 -> f",
                  "y RSHIFT 2 -> g",
                  "NOT x -> h",
                  "NOT y -> i"]
                     .join("\n");
        let mut c = Circuit::from_str(&cs);
        assert_eq!(c.eval(&Source::Wire(Wire("d".to_owned()))), Some(72));
        assert_eq!(c.eval(&Source::Wire(Wire("e".to_owned()))), Some(507));
        assert_eq!(c.eval(&Source::Wire(Wire("f".to_owned()))), Some(492));
        assert_eq!(c.eval(&Source::Wire(Wire("g".to_owned()))), Some(114));
        assert_eq!(c.eval(&Source::Wire(Wire("h".to_owned()))), Some(65412));
        assert_eq!(c.eval(&Source::Wire(Wire("i".to_owned()))), Some(65079));
        assert_eq!(c.eval(&Source::Wire(Wire("x".to_owned()))), Some(123));
        assert_eq!(c.eval(&Source::Wire(Wire("y".to_owned()))), Some(456));
    }
}

const DATA: &'static str = include_str!("../data/input_7.txt");

pub fn main() -> Vec<String> {
    let s1 = run_circuit(DATA, "a");
    let s2 = part2(DATA, "a", "b", 956);
    vec![s1.to_string(), s2.to_string()]
}
