#![feature(iter_arith)] // sum is not stable
#![feature(vec_push_all)]

use std::io;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate scan_fmt;

mod day1;
mod day2;
mod day3;

fn main() {
    try_main().unwrap()
}

fn try_main() -> io::Result<()> {
    let s = try!(run_from_file("input_1.txt", day1::main));
    s.iter().map(|s| println!("{}", s)).collect::<Vec<()>>();

    let s = try!(run_from_file("input_2.txt", day2::main));
    s.iter().map(|s| println!("{}", s)).collect::<Vec<()>>();

    let s = try!(run_from_file("input_3.txt", day3::main));
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
    assert_eq!(run_from_file("input_1.txt", day1::main).unwrap(),
               vec!["74", "1795"]);
    assert_eq!(run_from_file("input_2.txt", day2::main).unwrap(),
               vec!["1586300", "3737498"]);
    assert_eq!(run_from_file("input_3.txt", day3::main).unwrap(),
               vec!["2565", "2639"]);
}
