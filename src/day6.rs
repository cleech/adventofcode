extern crate pcre;
use self::pcre::Pcre;

const DATA: &'static str = include_str!("../data/input_6.txt");

pub fn main() -> Vec<String> {
    let s1 = light_show(DATA);
    let s2 = bright_show(DATA);
    vec![s1.to_string(), s2.to_string()]
}

fn doit<F, L>(f: &F, state: &mut [[L; 1000]; 1000], a: (usize, usize), b: (usize, usize))
    where F: Fn(&mut L)
{
    for x in a.0..(b.0 + 1) {
        for y in a.1..(b.1 + 1) {
            f(&mut state[x][y]);
        }
    }
}

fn light_show(input: &str) -> usize {
    let coords = Pcre::compile(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut state = box [[false; 1000]; 1000];

    let on = |light: &mut bool| *light = true;
    let off = |light: &mut bool| *light = false;
    let toggle = |light: &mut bool| *light = !*light;

    for line in input.lines() {
        let m = coords.exec(line).unwrap();
        let a = m.group(1).parse::<usize>().unwrap();
        let b = m.group(2).parse::<usize>().unwrap();
        let c = m.group(3).parse::<usize>().unwrap();
        let d = m.group(4).parse::<usize>().unwrap();

        match &line[0..7] {
            "turn on" => doit(&on, &mut state, (a, b), (c, d)),
            "turn of" => doit(&off, &mut state, (a, b), (c, d)),
            "toggle " => doit(&toggle, &mut state, (a, b), (c, d)),
            _ => panic!("invalid input"),
        }
    }
    state.iter()
         .flat_map(|inner| inner.iter())
         .filter(|&&on| on == true)
         .count()
}

#[test]
fn test_light_show() {
    assert_eq!(light_show("turn on 0,0 through 999,999"), 1000 * 1000);
    assert_eq!(light_show("toggle 0,0 through 999,0"), 1000);
    assert_eq!(light_show("turn off 499,499 through 500,500"), 0);
}

fn bright_show(input: &str) -> u32 {
    let coords = Pcre::compile(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut state = box [[0u32; 1000]; 1000];

    let on = |light: &mut u32| *light += 1;
    let off = |light: &mut u32| {
        if *light > 0 {
            *light -= 1
        }
    };
    let toggle = |light: &mut u32| *light += 2;

    for line in input.lines() {
        let m = coords.exec(line).unwrap();
        let a = m.group(1).parse::<usize>().unwrap();
        let b = m.group(2).parse::<usize>().unwrap();
        let c = m.group(3).parse::<usize>().unwrap();
        let d = m.group(4).parse::<usize>().unwrap();

        match &line[0..7] {
            "turn on" => doit(&on, &mut state, (a, b), (c, d)),
            "turn of" => doit(&off, &mut state, (a, b), (c, d)),
            "toggle " => doit(&toggle, &mut state, (a, b), (c, d)),
            _ => panic!("invalid input"),
        }
    }
    state.iter()
         .flat_map(|inner| inner.iter())
         .sum()
}

#[test]
fn test_bright_show() {
    assert_eq!(bright_show("turn on 0,0 through 0,0"), 1);
    assert_eq!(bright_show("toggle 0,0 through 999,999"), 2 * 1000 * 1000);
}
