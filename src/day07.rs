use std::str::FromStr;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Source {
    Pattern(u16),
    Wire(String),
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
                Ok(Source::Wire(s.to_owned()))
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
        shift: Source,
        out: Source,
    },
    RShift {
        src: Source,
        shift: Source,
        out: Source,
    },
}

impl FromStr for Gate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Gate, Self::Err> {
        match &s.split_whitespace().collect::<Vec<_>>()[..] { 
            [a, "->", o] => {
                a.parse::<Source>()
                 .and_then(|a| o.parse::<Source>().map(|o| Gate::Pass { src: a, out: o }))
            }
            ["NOT", a, "->", o] => {
                a.parse::<Source>()
                 .and_then(|a| o.parse::<Source>().map(|o| Gate::Not { src: a, out: o }))
            }
            [a, op @ "AND", b, "->", o] |
            [a, op @ "OR", b, "->", o] |
            [a, op @ "LSHIFT", b, "->", o] |
            [a, op @ "RSHIFT", b, "->", o] => {
                a.parse::<Source>().and_then(|a| {
                    b.parse::<Source>().and_then(|b| {
                        o.parse::<Source>().map(|o| {
                            match op {
                                "AND" => {
                                    Gate::And {
                                        a: a,
                                        b: b,
                                        out: o,
                                    }
                                }
                                "OR" => {
                                    Gate::Or {
                                        a: a,
                                        b: b,
                                        out: o,
                                    }
                                }
                                "LSHIFT" => {
                                    Gate::LShift {
                                        src: a,
                                        shift: b,
                                        out: o,
                                    }
                                }
                                "RSHIFT" => {
                                    Gate::RShift {
                                        src: a,
                                        shift: b,
                                        out: o,
                                    }
                                }
                                _ => unreachable!(),
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
            Gate::LShift{ ref src, ref shift, .. } => c.eval(src).map(|v| v << c.eval(shift).unwrap()),
            Gate::RShift{ ref src, ref shift, .. } => c.eval(src).map(|v| v >> c.eval(shift).unwrap()),
        }
    }
}

struct Circuit {
    gates: HashMap<Source, Gate>,
    cache: HashMap<Source, u16>,
}

impl FromStr for Circuit {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Circuit, Self::Err> {
        let mut circuit = HashMap::new();
        let cache = HashMap::new();

        for line in input.lines() {
            if let Ok(gate) = Gate::from_str(line) {
                circuit.insert(gate.out(), gate);
            } else {
                return Err("gate parse error");
            }
        }
        Ok(Circuit {
            gates: circuit,
            cache: cache,
        })
    }
}

impl Circuit {
    fn eval(&mut self, wire: &Source) -> Option<u16> {
        match *wire {
            Source::Pattern(p) => Some(p),
            ref s @ Source::Wire(_) => {
                if self.cache.contains_key(s) {
                    self.cache.get(s).cloned()
                } else {
                    if let Some(v) = self.gates.get(s).cloned().and_then(|g| g.eval(self)) {
                        self.cache.insert(s.clone(), v);
                        Some(v)
                    } else {
                        None
                    }
                }
            }
        }
    }

    fn force(&mut self, s: &Source, v: u16) {
        self.cache.clear();
        self.cache.insert(s.clone(), v);
    }
}

fn run_circuit(input: &str, target: &str) -> Option<u16> {
    if let Ok(mut c) = Circuit::from_str(input) {
        c.eval(&Source::Wire(target.to_owned()))
    } else {
        None
    }
}

fn part2(input: &str, target: &str, force: &str, v: u16) -> Option<u16> {
    if let Ok(mut c) = Circuit::from_str(input) {
        c.force(&Source::Wire(force.to_owned()), v);
        c.eval(&Source::Wire(target.to_owned()))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::{Circuit, Source};

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
        let mut c = Circuit::from_str(&cs).unwrap();
        assert_eq!(c.eval(&Source::Wire("d".to_owned())), Some(72));
        assert_eq!(c.eval(&Source::Wire("e".to_owned())), Some(507));
        assert_eq!(c.eval(&Source::Wire("f".to_owned())), Some(492));
        assert_eq!(c.eval(&Source::Wire("g".to_owned())), Some(114));
        assert_eq!(c.eval(&Source::Wire("h".to_owned())), Some(65412));
        assert_eq!(c.eval(&Source::Wire("i".to_owned())), Some(65079));
        assert_eq!(c.eval(&Source::Wire("x".to_owned())), Some(123));
        assert_eq!(c.eval(&Source::Wire("y".to_owned())), Some(456));
    }
}

const DATA: &'static str = include_str!("../data/input_7.txt");

pub fn main() -> Vec<String> {
    let s1 = run_circuit(DATA, "a").unwrap();
    let s2 = part2(DATA, "a", "b", 956).unwrap();
    vec![s1.to_string(), s2.to_string()]
}
