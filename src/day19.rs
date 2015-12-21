extern crate itertools;
use self::itertools::Itertools;

const DATA: &'static str = include_str!("../data/input_19.txt");

pub fn main() -> Vec<String> {
    let ctx = Context::from_str(DATA);
    vec![part1(ctx).to_string()]
}

fn part1(ctx: Context) -> usize {
    ctx.replacements
       .iter()
       .flat_map(|&(from, to)| ctx.medicine.single_replacements(from, to).into_iter())
       .unique()
       .count()
}

#[derive(Debug)]
struct Context {
    replacements: Vec<(&'static str, &'static str)>,
    medicine: &'static str,
}

impl Context {
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

        Context {
            replacements: replacements,
            medicine: medicine,
        }
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
