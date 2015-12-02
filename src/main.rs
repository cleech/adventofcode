#![feature(iter_arith)] // sum is not stable?

use std::io;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate scan_fmt;

fn main() {
    try_main().unwrap()
}

fn try_main() -> io::Result<()> {
    let s = try!(run_from_file("input_1.txt", day1::main));
    s.iter().map(|s| println!("{}", s)).collect::<Vec<()>>();

    let s = try!(run_from_file("input_2.txt", day2::main));
    s.iter().map(|s| println!("{}", s)).collect::<Vec<()>>();

    Ok(())
}

fn run_from_file<F>(file: &str, it: F) -> io::Result<Vec<String>>
    where F: Fn(&str) -> Vec<String>
{
    let mut f = try!(File::open(file));
    let mut input = String::new();
    try!(f.read_to_string(&mut input));
    Ok(it(&input))
}

#[test]
fn verify_my_answers() {
    assert!(run_from_file("input_1.txt", day1::main).unwrap() == vec!["74", "1795"]);
    assert!(run_from_file("input_2.txt", day2::main).unwrap() == vec!["1586300", "3737498"]);
}

mod day1 {
    pub fn main(input: &str) -> Vec<String> {
        let s1 = adv_one_one(&input);
        let s2 = adv_one_two(&input, -1);
        vec![s1.to_string(), s2.to_string()]
    }

    fn inmap(c: char) -> i32 {
        match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("invalid input"),
        }
    }

    fn adv_one_one(input: &str) -> i32 {
        input.chars().map(inmap).sum()
    }

    #[test]
    fn one_one_test() {
        assert!(adv_one_one("(())") == 0);
        assert!(adv_one_one("()()") == 0);
        assert!(adv_one_one("(((") == 3);
        assert!(adv_one_one("(()(()(") == 3);
        assert!(adv_one_one("))(((((") == 3);
        assert!(adv_one_one("())") == -1);
        assert!(adv_one_one("))(") == -1);
        assert!(adv_one_one(")))") == -3);
        assert!(adv_one_one(")())())") == -3);
    }

    fn adv_one_two(input: &str, target: i32) -> i32 {
        input.chars()
             .map(inmap)
             .scan(0, |floor, direction| {
                 *floor += direction;
                 Some(*floor)
             })
             .position(|floor| floor == target)
             .unwrap() as i32 + 1
    }

    #[test]
    fn one_two_test() {
        assert!(adv_one_two(")", -1) == 1);
        assert!(adv_one_two("()())", -1) == 5);
    }
}

mod day2 {
    pub fn main(input: &str) -> Vec<String> {
        let s1 = adv_two_one(&input);
        let s2 = adv_two_two(&input);
        vec![s1.to_string(), s2.to_string()]
    }

    fn adv_two_one(input: &str) -> u32 {
        fn pkg(l: u32, w: u32, h: u32) -> u32 {
            let sides = vec![l * w, w * h, h * l];
            let extra = sides.iter().min().unwrap();
            2 * l * w + 2 * w * h + 2 * h * l + extra
        }

        input.lines()
             .map(|line| {
                 let (l, w, h) = scan_fmt!(line, "{d}x{d}x{d}", u32, u32, u32);
                 pkg(l.unwrap(), w.unwrap(), h.unwrap())
             })
             .sum()
    }

    #[test]
    fn two_one_test() {
        assert!(adv_two_one("2x3x4") == 58);
        assert!(adv_two_one("1x1x10") == 43);
    }

    fn adv_two_two(input: &str) -> u32 {
        fn pkg(l: u32, w: u32, h: u32) -> u32 {
            let sides = vec![2 * l + 2 * w, 2 * w + 2 * h, 2 * h + 2 * l];
            let ribbon = sides.iter().min().unwrap();
            let bow = l * w * h;
            ribbon + bow
        }

        input.lines()
             .map(|line| {
                 let (l, w, h) = scan_fmt!(line, "{d}x{d}x{d}", u32, u32, u32);
                 pkg(l.unwrap(), w.unwrap(), h.unwrap())
             })
             .sum()
    }

    #[test]
    fn two_two_test() {
        assert!(adv_two_two("2x3x4") == 34);
        assert!(adv_two_two("1x1x10") == 14);
    }
}
