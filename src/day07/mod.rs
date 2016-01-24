use std::str::FromStr;
use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, Shl, Shr, Not};

const DATA: &'static str = include_str!("input.txt");

pub fn main() -> Vec<String> {
    let s1 = run_circuit(DATA, "a").unwrap();
    let s2 = part2(DATA, "a", "b", 956).unwrap();
    vec![s1.to_string(), s2.to_string()]
}

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
    UnaryOp {
        src: Source,
        out: Source,
        f: fn(u16) -> u16,
    },
    BinaryOp {
        a: Source,
        b: Source,
        out: Source,
        f: fn(u16, u16) -> u16,
    },
}

impl FromStr for Gate {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Gate, Self::Err> {

        fn id<T>(x: T) -> T {
            x
        };

        fn unary_from_str(a: &str, o: &str, f: fn(u16) -> u16) -> Result<Gate, &'static str> {
            a.parse::<Source>().and_then(|a| {
                o.parse::<Source>().map(|o| {
                    Gate::UnaryOp {
                        src: a,
                        out: o,
                        f: f,
                    }
                })
            })
        }

        fn binary_from_str(a: &str,
                           b: &str,
                           o: &str,
                           f: fn(u16, u16) -> u16)
                           -> Result<Gate, &'static str> {
            a.parse::<Source>().and_then(|a| {
                b.parse::<Source>().and_then(|b| {
                    o.parse::<Source>().map(|o| {
                        Gate::BinaryOp {
                            a: a,
                            b: b,
                            out: o,
                            f: f,
                        }
                    })
                })
            })
        }

        match &s.split_whitespace().collect::<Vec<_>>()[..] { 
            [a, "->", o] => unary_from_str(a, o, id),
            ["NOT", a, "->", o] => unary_from_str(a, o, Not::not),
            [a, "AND", b, "->", o] => binary_from_str(a, b, o, BitAnd::bitand),
            [a, "OR", b, "->", o] => binary_from_str(a, b, o, BitOr::bitor),
            [a, "LSHIFT", b, "->", o] => binary_from_str(a, b, o, Shl::shl),
            [a, "RSHIFT", b, "->", o] => binary_from_str(a, b, o, Shr::shr),
            _ => Err("bad input"),
        }
    }
}

impl Gate {
    fn out(&self) -> Source {
        match *self {
            Gate::UnaryOp{ ref out, ..} => out,
            Gate::BinaryOp{ ref out, ..} => out,
        }
        .clone()
    }

    fn eval(&self, c: &mut Circuit) -> Option<u16> {
        match *self {
            Gate::UnaryOp{ ref src, ref f, .. } => c.eval(src).map(f),
            Gate::BinaryOp{ ref a, ref b, ref f, .. } => {
                c.eval(a).and_then(|a| c.eval(b).map(|b| f(a, b)))
            }
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
            if let Ok(gate) = line.parse::<Gate>() {
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
    if let Ok(mut c) = input.parse::<Circuit>() {
        c.eval(&Source::Wire(target.to_owned()))
    } else {
        None
    }
}

fn part2(input: &str, target: &str, force: &str, v: u16) -> Option<u16> {
    if let Ok(mut c) = input.parse::<Circuit>() {
        c.force(&Source::Wire(force.to_owned()), v);
        c.eval(&Source::Wire(target.to_owned()))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::{Circuit, Source};

    #[test]
    fn examples() {
        let cs = ["123 -> x",
                  "456 -> y",
                  "x AND y -> d",
                  "x OR y -> e",
                  "x LSHIFT 2 -> f",
                  "y RSHIFT 2 -> g",
                  "NOT x -> h",
                  "NOT y -> i"]
                     .join("\n");
        let mut c = cs.parse::<Circuit>().unwrap();
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
