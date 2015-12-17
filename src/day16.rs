use std::str;
use std::collections::HashMap;
use nom::{IResult, is_alphabetic, is_digit};

const DATA: &'static str = include_str!("../data/input_16.txt");
const MFCSAM: [&'static str; 10] = ["children: 3",
                                    "cats: 7",
                                    "samoyeds: 2",
                                    "pomeranians: 3",
                                    "akitas: 0",
                                    "vizslas: 0",
                                    "goldfish: 5",
                                    "trees: 3",
                                    "cars: 2",
                                    "perfumes: 1"];

pub fn main() -> Vec<String> {
    vec![format!("{:?}", find_aunt(&filter_1)), format!("{:?}", find_aunt(&filter_2))]
}

fn filter_1(sue: &Aunt, compound: &(String, u32)) -> bool {
    match sue.tags.get(&compound.0) {
        None => true,
        Some(v) => *v == compound.1,
    }
}

fn filter_2(sue: &Aunt, compound: &(String, u32)) -> bool {
    match sue.tags.get(&compound.0) {
        None => true,
        Some(v) => {
            match &compound.0[..] {
                "cats" | "trees" => *v > compound.1,
                "pomeranians" | "goldfish" => *v < compound.1,
                _ => *v == compound.1,
            }
        }
    }
}

fn find_aunt<F>(f: &F) -> Vec<u32>
    where F: Fn(&Aunt, &(String, u32)) -> bool
{
    let mut aunt_sues = DATA.lines().map(|line| Aunt::from_str(line)).collect::<Vec<_>>();
    for c in MFCSAM.iter().map(|l| compound(l.as_bytes())) {
        if let IResult::Done(_, compound) = c {
            aunt_sues = aunt_sues.into_iter()
                                 .filter(|s| f(s, &compound))
                                 .collect();
        }
    }
    aunt_sues.iter().map(|a| a.id).collect()
}

named!(compound<&[u8], (String,u32)>,
    chain!(
        name: take_while1!(is_alphabetic) ~ 
        tag!(": ") ~ 
        value: take_while1!(is_digit), 
        || (String::from_utf8(name.to_owned()).unwrap(), 
            str::from_utf8(value).unwrap().parse().unwrap())
    )
);

named!(compound_map<&[u8], HashMap<String,u32> >,
    map!(
        separated_list!(tag!(", "), compound),
        |v| {
            let mut map = HashMap::new();
            for (s,n) in v {
                map.insert(s,n);
            }
            map
        }
    )
);

struct Aunt {
    id: u32,
    tags: HashMap<String, u32>,
}

named!(aunt_sue<&[u8], Aunt>,
    chain!(
        tag!("Sue ") ~
        id: take_while1!(is_digit) ~
        tag!(": ") ~
        tags: compound_map,
        || Aunt { id: str::from_utf8(id).unwrap().parse().unwrap(),
                  tags: tags }
    )
);

impl Aunt {
    fn from_str(input: &str) -> Aunt {
        if let IResult::Done(_, sue) = aunt_sue(input.as_bytes()) {
            sue
        } else {
            panic!("invalid input")
        }
    }
}
