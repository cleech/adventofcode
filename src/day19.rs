extern crate itertools;
use self::itertools::Itertools;

const DATA: &'static str = include_str!("../data/input_19.txt");

pub fn main() -> Vec<String> {
    let ctx = Context::from_str(DATA);
    let s1 = ctx.all_replacements().count().to_string();
    let s2 = ctx.find_production(ctx.medicine, 0).unwrap().to_string();
    vec![s1, s2]
}

#[derive(Debug)]
struct Context {
    replacements: Vec<(&'static str, &'static str)>,
    medicine: &'static str,
}

impl<'a> Context {
    fn from_str(data: &'static str) -> Context {
        let mut input = data.lines();
        let mut replacements = Vec::new();

        while let Some(r) = input.next() {
            if r.trim().is_empty() {
                break;
            }
            let mut tokens = r.split(" => ");
            replacements.push((tokens.next().unwrap(), tokens.next().unwrap()));
        }
        let medicine = input.next().unwrap();
        assert_eq!(input.next(), None);

        // for part 2, we want to work backwords trying longest first
        // as we reduce the medicine molecule towards "e"
        // part 1 does not care about ordering of the replacement rules
        replacements.sort_by(|&(_, a), &(_, b)| b.len().cmp(&a.len()));

        Context {
            replacements: replacements,
            medicine: medicine,
        }
    }

    fn all_replacements(&'a self) -> Box<Iterator<Item = String> + 'a> {
        box self.replacements
                .iter()
                .flat_map(move |&(from, to)| {
                    self.medicine.single_replacements(from, to).into_iter()
                })
                .unique()
    }

    fn find_production(&self, input: &str, depth: usize) -> Option<usize> {
        if input == "e" {
            return Some(depth);
        }
        for next_step in self.replacements
                             .iter()
                             .flat_map(|&(from, to)| {
                                 input.single_replacements(to, from).into_iter()
                             })
                             .unique() {
            if let Some(count) = self.find_production(&next_step, depth + 1) {
                return Some(count);
            }
        }
        None
    }
}

trait SingleReplacements {
    // like std::str::replace but returning a Vec<String> with one replacement in each
    fn single_replacements(&self, from: &str, to: &str) -> Vec<String>;
}

impl<'a> SingleReplacements for &'a str {
    fn single_replacements(&self, from: &str, to: &str) -> Vec<String> {
        let mut results = Vec::new();
        for (start, part) in self.match_indices(from) {
            let mut string = String::new();
            string.push_str(unsafe { self.slice_unchecked(0, start) });
            string.push_str(to);
            string.push_str(unsafe { self.slice_unchecked(start + part.len(), self.len()) });
            results.push(string);
        }
        results
    }
}
