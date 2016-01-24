use std::str::FromStr;

extern crate itertools;
use self::itertools::Itertools;

const DATA: &'static str = include_str!("../data/input_19.txt");

pub fn main() -> Vec<String> {
    if let Ok(ctx) = DATA.parse::<Context>() {
        let s1 = ctx.all_replacements().count().to_string();
        let s2 = ctx.find_production(&ctx.medicine, 0).unwrap().to_string();
        vec![s1, s2]
    } else {
        vec![]
    }
}

#[derive(Debug)]
struct Context {
    replacements: Vec<(String, String)>,
    medicine: String,
}

impl FromStr for Context {
    type Err = ();

    fn from_str(data: &str) -> Result<Context, ()> {
        let mut input = data.lines();
        let mut replacements = Vec::new();

        while let Some(r) = input.next() {
            if r.trim().is_empty() {
                break;
            }
            let mut tokens = r.split(" => ");
            if let Some((l, r)) = tokens.next().and_then(|l| tokens.next().map(|r| (l, r))) {
                replacements.push((l.to_owned(), r.to_owned()))
            } else {
                return Err(());
            }
        }
        if let Some(medicine) = input.next() {
            // for part 2, we want to work backwards trying longest first
            // as we reduce the medicine molecule towards "e"
            // part 1 does not care about ordering of the replacement rules
            replacements.sort_by(|&(_, ref a), &(_, ref b)| b.len().cmp(&a.len()));

            Ok(Context {
                replacements: replacements,
                medicine: medicine.to_owned(),
            })
        } else {
            Err(())
        }
    }
}

impl<'a> Context {
    fn all_replacements(&'a self) -> Box<Iterator<Item = String> + 'a> {
        box self.replacements
                .iter()
                .flat_map(move |&(ref from, ref to)| {
                    (&self.medicine[..]).single_replacements(from, to).into_iter()
                })
                .unique()
    }

    fn find_production(&self, input: &str, depth: usize) -> Option<usize> {
        if input == "e" {
            return Some(depth);
        }
        for next_step in self.replacements
                             .iter()
                             .flat_map(|&(ref from, ref to)| {
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

#[cfg(test)]
mod test {
    use super::Context;

    #[test]
    fn examples_1() {
        let example = "H => HO\nH => OH\nO => HH\n\nHOH";
        let ctx = example.parse::<Context>().ok();
        let soln = ctx.map(|ctx| ctx.all_replacements().count());
        assert_eq!(soln, Some(4));

        let example = "H => HO\nH => OH\nO => HH\n\nHOHOHO";
        let ctx = example.parse::<Context>().ok();
        let soln = ctx.map(|ctx| ctx.all_replacements().count());
        assert_eq!(soln, Some(7));
    }

    #[test]
    fn examples_2() {
        let example = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOH";
        let ctx = example.parse::<Context>().ok();
        let soln = ctx.and_then(|ctx| ctx.find_production(&ctx.medicine, 0));
        assert_eq!(soln, Some(3));

        let example = "e => H\ne => O\nH => HO\nH => OH\nO => HH\n\nHOHOHO";
        let ctx = example.parse::<Context>().ok();
        let soln = ctx.and_then(|ctx| ctx.find_production(&ctx.medicine, 0));
        assert_eq!(soln, Some(6));
    }
}
