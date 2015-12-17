use std::collections::{HashSet, HashMap};

extern crate permutohedron;

const DATA: &'static str = include_str!("../data/input_13.txt");

pub fn main() -> Vec<String> {
    let mut table = Table::build(DATA);
    let s1 = table.find_best_setting();
    table.add_me();
    let s2 = table.find_best_setting();
    vec![s1.to_string(), s2.to_string()]
}

#[derive(Eq, PartialEq, Debug)]
struct Rule {
    guests: (String, String),
    target: String,
    delta: i32,
}

impl Rule {
    fn from_str(rule: &str) -> Rule {
        let pattern = "{} would {} {d} happiness units by sitting next to {}.";
        let (name, gain_or_lose, mut delta, other) = scan_fmt!(rule,
                                                               pattern,
                                                               String,
                                                               String,
                                                               i32,
                                                               String);
        match gain_or_lose.unwrap().as_ref() {
            "gain" => {}
            "lose" => delta = Some(-delta.unwrap()),
            _ => panic!("bad input"),
        }
        Rule {
            target: name.clone().unwrap(),
            guests: (name.unwrap(), other.unwrap()),
            delta: delta.unwrap(),
        }
    }
}

struct Table {
    guests: HashSet<String>,
    rules: HashMap<(String, String), (String, i32)>,
}

impl Table {
    fn build(input: &str) -> Table {
        let mut guestlist = HashSet::new();
        let mut rules = HashMap::new();

        for line in input.lines() {
            let r = Rule::from_str(line);
            guestlist.insert(r.target.clone());
            rules.insert(r.guests, (r.target, r.delta));
        }
        Table {
            guests: guestlist,
            rules: rules,
        }
    }

    fn happiness_effect(&self, settings: &[&String]) -> i32 {
        let first: String = (*settings.first().unwrap()).to_owned();
        let last: String = (*settings.last().unwrap()).to_owned();
        settings.windows(2)
                .map(|w| {
                    self.rules.get(&(w[0].to_owned(), w[1].to_owned())).unwrap().1 +
                    self.rules.get(&(w[1].to_owned(), w[0].to_owned())).unwrap().1
                })
                .sum::<i32>() +
        self.rules
            .get(&(first.clone(), last.clone()))
            .unwrap()
            .1 +
        self.rules
            .get(&(last.clone(), first.clone()))
            .unwrap()
            .1
    }

    fn find_best_setting(&self) -> i32 {
        let mut data = self.guests.iter().collect::<Vec<_>>();
        let heap = permutohedron::Heap::new(&mut data);
        heap.map(|p| self.happiness_effect(&p[..])).max().unwrap()
    }

    fn add_me(&mut self) {
        for guest in &self.guests {
            self.rules.insert(("me".to_owned(), guest.clone()), ("me".to_owned(), 0));
            self.rules.insert((guest.clone(), "me".to_owned()), (guest.clone(), 0));
        }
        self.guests.insert("me".to_owned());
    }
}

#[test]
fn test_day9() {
    let rules = ["Alice would gain 54 happiness units by sitting next to Bob.",
                 "Alice would lose 79 happiness units by sitting next to Carol.",
                 "Alice would lose 2 happiness units by sitting next to David.",
                 "Bob would gain 83 happiness units by sitting next to Alice.",
                 "Bob would lose 7 happiness units by sitting next to Carol.",
                 "Bob would lose 63 happiness units by sitting next to David.",
                 "Carol would lose 62 happiness units by sitting next to Alice.",
                 "Carol would gain 60 happiness units by sitting next to Bob.",
                 "Carol would gain 55 happiness units by sitting next to David.",
                 "David would gain 46 happiness units by sitting next to Alice.",
                 "David would lose 7 happiness units by sitting next to Bob.",
                 "David would gain 41 happiness units by sitting next to Carol."]
                    .join("\n");
    let table = Table::build(&rules);
    assert_eq!(table.find_best_setting(), 330);
}
