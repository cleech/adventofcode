use std::collections::{HashSet, HashMap};
use std::str::FromStr;

extern crate permutohedron;

const DATA: &'static str = include_str!("input.txt");

pub fn main() -> Vec<String> {
    let mut table = Table::build(DATA).unwrap();
    let s1 = table.find_best_setting().unwrap();
    table.add_me();
    let s2 = table.find_best_setting().unwrap();
    vec![s1.to_string(), s2.to_string()]
}

#[derive(Eq, PartialEq, Debug)]
struct Rule {
    guests: (String, String),
    target: String,
    delta: i32,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(rule: &str) -> Result<Rule, ()> {
        let pattern = "{} would {} {d} happiness units by sitting next to {}.";
        if let (Some(name), Some(gain_or_lose), Some(mut delta), Some(other)) = scan_fmt!(rule,
                                                                                          pattern,
                                                                                          String,
                                                                                          String,
                                                                                          i32,
                                                                                          String) {
            match gain_or_lose.as_ref() {
                "gain" => {}
                "lose" => delta = -delta,
                _ => return Err(()),
            }
            Ok(Rule {
                target: name.clone(),
                guests: (name, other),
                delta: delta,
            })
        } else {
            Err(())
        }
    }
}

struct Table {
    guests: HashSet<String>,
    rules: HashMap<(String, String), (String, i32)>,
}

impl Table {
    fn build(input: &str) -> Option<Table> {
        let mut guestlist = HashSet::new();
        let mut rules = HashMap::new();

        for line in input.lines() {
            if let Ok(r) = line.parse::<Rule>() {
                guestlist.insert(r.target.clone());
                rules.insert(r.guests, (r.target, r.delta));
            } else {
                return None;
            }
        }
        Some(Table {
            guests: guestlist,
            rules: rules,
        })
    }

    fn happiness_effect(&self, settings: &[&String]) -> i32 {
        if settings.is_empty() {
            return 0;
        }
        // unwrap on first and last is safe, as is_empty has been checked
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

    fn find_best_setting(&self) -> Option<i32> {
        let mut data = self.guests.iter().collect::<Vec<_>>();
        let heap = permutohedron::Heap::new(&mut data);
        heap.map(|p| self.happiness_effect(&p[..])).max()
    }

    fn add_me(&mut self) {
        for guest in &self.guests {
            self.rules.insert(("me".to_owned(), guest.clone()), ("me".to_owned(), 0));
            self.rules.insert((guest.clone(), "me".to_owned()), (guest.clone(), 0));
        }
        self.guests.insert("me".to_owned());
    }
}

#[cfg(test)]
mod test {
    use super::Table;

    #[test]
    fn examples() {
        let example_data = ["Alice would gain 54 happiness units by sitting next to Bob.",
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
                            "David would gain 41 happiness units by sitting next to Carol."];
        let table = Table::build(&example_data.join("\n")).unwrap();
        assert_eq!(table.find_best_setting(), Some(330));
    }
}
