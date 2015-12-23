use std::collections::HashMap;

extern crate pcre;
use self::pcre::Pcre;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Wire(pub String);

#[derive(PartialEq, Eq, Hash, Clone)]
enum Source {
    Pattern(u16),
    Wire(Wire),
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Gate {
    Source {
        sig: u16,
        out: Wire,
    },
    Pass {
        src: Wire,
        out: Wire,
    },
    Not {
        src: Wire,
        out: Wire,
    },
    And {
        a: Source,
        b: Wire,
        out: Wire,
    },
    Or {
        a: Wire,
        b: Wire,
        out: Wire,
    },
    LShift {
        src: Wire,
        shift: u16,
        out: Wire,
    },
    RShift {
        src: Wire,
        shift: u16,
        out: Wire,
    },
}

impl Gate {
    fn from_str(input: &str) -> Gate {
        let mut set = Pcre::compile(r"^(\d+) -> ([a-z]+)$").unwrap();
        let mut pass = Pcre::compile(r"^([a-z]+) -> ([a-z]+)$").unwrap();
        let mut not = Pcre::compile(r"^NOT ([a-z]+) -> ([a-z]+)$").unwrap();
        let mut and =
            Pcre::compile(r"^((?<pat>\d+)|(?<wire>[a-z]+)) AND (?<rhs>[a-z]+) -> (?<out>[a-z]+)$")
                .unwrap();
        let mut or = Pcre::compile(r"^([a-z]+) OR ([a-z]+) -> ([a-z]+)$").unwrap();
        let mut lshift = Pcre::compile(r"^([a-z]+) LSHIFT (\d+) -> ([a-z]+)$").unwrap();
        let mut rshift = Pcre::compile(r"^([a-z]+) RSHIFT (\d+) -> ([a-z]+)$").unwrap();

        if let Some(m) = set.exec(input) {
            Gate::Source {
                sig: m.group(1).parse::<u16>().unwrap(),
                out: Wire(m.group(2).to_string()),
            }
        } else if let Some(m) = pass.exec(input) {
            Gate::Pass {
                src: Wire(m.group(1).to_string()),
                out: Wire(m.group(2).to_string()),
            }
        } else if let Some(m) = not.exec(input) {
            Gate::Not {
                src: Wire(m.group(1).to_string()),
                out: Wire(m.group(2).to_string()),
            }
        } else if let Some(m) = and.exec(input) {
            if m.group_len(2) > 0 {
                Gate::And {
                    a: Source::Pattern(m.group(2).parse::<u16>().unwrap()),
                    b: Wire(m.group(4).to_string()),
                    out: Wire(m.group(5).to_string()),
                }
            } else {
                Gate::And {
                    a: Source::Wire(Wire(m.group(3).to_string())),
                    b: Wire(m.group(4).to_string()),
                    out: Wire(m.group(5).to_string()),
                }
            }
        } else if let Some(m) = or.exec(input) {
            Gate::Or {
                a: Wire(m.group(1).to_string()),
                b: Wire(m.group(2).to_string()),
                out: Wire(m.group(3).to_string()),
            }
        } else if let Some(m) = lshift.exec(input) {
            Gate::LShift {
                src: Wire(m.group(1).to_string()),
                shift: m.group(2).parse::<u16>().unwrap(),
                out: Wire(m.group(3).to_string()),
            }
        } else if let Some(m) = rshift.exec(input) {
            Gate::RShift {
                src: Wire(m.group(1).to_string()),
                shift: m.group(2).parse::<u16>().unwrap(),
                out: Wire(m.group(3).to_string()),
            }
        } else {
            panic!("invalid input: {}", input)
        }
    }

    fn out(&self) -> Wire {
        match *self {
            Gate::Source{ ref out, ..} => out,
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
            Gate::Source{ sig, .. } => Some(sig),
            Gate::Pass{ ref src, .. } => c.eval(src),
            Gate::Not{ ref src, .. } => c.eval(src).map(|v| !v),
            Gate::And{ ref a, ref b, .. } => {
                match a {
                    &Source::Wire(ref w) => {
                        match (c.eval(w), c.eval(b)) {
                            (None, _) => None,
                            (_, None) => None,
                            (Some(lhs), Some(rhs)) => Some(lhs & rhs),
                        }
                    }
                    &Source::Pattern(p) => {
                        match c.eval(b) {
                            None => None,
                            Some(rhs) => Some(p & rhs),
                        }
                    }
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
    gates: HashMap<Wire, Gate>,
    cache: HashMap<Wire, u16>,
}

impl Circuit {
    fn from_str(input: &str) -> Circuit {
        let mut circuit = HashMap::new();
        let cache = HashMap::new();

        for line in input.lines() {
            let gate = Gate::from_str(line);
            circuit.insert(gate.out(), gate);
        }
        Circuit {
            gates: circuit,
            cache: cache,
        }
    }

    fn eval(&mut self, wire: &Wire) -> Option<u16> {
        if self.cache.contains_key(wire) {
            self.cache.get(wire).cloned()
        } else {
            let v = self.gates.get(wire).cloned().and_then(|g| g.eval(self));
            self.cache.insert(wire.clone(), v.unwrap());
            return v;
        }
    }

    fn force(&mut self, wire: &Wire, v: u16) {
        self.cache.clear();
        self.cache.insert(wire.clone(), v);
    }
}

fn run_circuit(input: &str, target: &str) -> u16 {
    let mut c = Circuit::from_str(input);
    c.eval(&Wire(target.to_string())).unwrap()
}

fn part2(input: &str, target: &str, force: &str, v: u16) -> u16 {
    let mut c = Circuit::from_str(input);
    c.force(&Wire(force.to_owned()), v);
    c.eval(&Wire(target.to_string())).unwrap()
}

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
    assert_eq!(c.eval(&Wire("d".to_string())), Some(72));
    assert_eq!(c.eval(&Wire("e".to_string())), Some(507));
    assert_eq!(c.eval(&Wire("f".to_string())), Some(492));
    assert_eq!(c.eval(&Wire("g".to_string())), Some(114));
    assert_eq!(c.eval(&Wire("h".to_string())), Some(65412));
    assert_eq!(c.eval(&Wire("i".to_string())), Some(65079));
    assert_eq!(c.eval(&Wire("x".to_string())), Some(123));
    assert_eq!(c.eval(&Wire("y".to_string())), Some(456));
}

const DATA: &'static str = include_str!("../data/input_7.txt");

pub fn main() -> Vec<String> {
    let s1 = run_circuit(DATA, "a");
    let s2 = part2(DATA, "a", "b", 956);
    vec![s1.to_string(), s2.to_string()]
}
