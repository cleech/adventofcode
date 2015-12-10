#![feature(iter_arith)] // sum is not stable
#![feature(box_syntax)]

use std::io;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate scan_fmt;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

fn main() {
    try_main().unwrap()
}

fn print_results(day: usize, results: Vec<String>) {
    println!("Day {}", day);
    for output in results {
        println!("  {}", output);
    }
}

fn try_main() -> io::Result<()> {
    let s = try!(run_from_file("data/input_1.txt", day1::main));
    print_results(1, s);

    let s = try!(run_from_file("data/input_2.txt", day2::main));
    print_results(2, s);

    let s = try!(run_from_file("data/input_3.txt", day3::main));
    print_results(3, s);

    let s = day4::main("yzbqklnj");
    print_results(4, s);

    let s = try!(run_from_file("data/input_5.txt", day5::main));
    print_results(5, s);

    let s = try!(run_from_file("data/input_6.txt", day6::main));
    print_results(6, s);

    let s = try!(run_from_file("data/input_7.txt", day7::main));
    print_results(7, s);

    let s = try!(run_from_file("data/input_8.txt", day8::main));
    print_results(8, s);

    let s = try!(run_from_file("data/input_9.txt", day9::main));
    print_results(9, s);

    let s = day10::main("3113322113");
    print_results(10, s);

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
#[ignore]
fn verify_my_answers() {
    assert_eq!(run_from_file("data/input_1.txt", day1::main).unwrap(),
               ["74", "1795"]);
    assert_eq!(run_from_file("data/input_2.txt", day2::main).unwrap(),
               ["1586300", "3737498"]);
    assert_eq!(run_from_file("data/input_3.txt", day3::main).unwrap(),
               ["2565", "2639"]);
    assert_eq!(day4::main("yzbqklnj"), ["282749", "9962624"]);
    assert_eq!(run_from_file("data/input_5.txt", day5::main).unwrap(),
               ["236", "51"]);
    assert_eq!(run_from_file("data/input_6.txt", day6::main).unwrap(),
               ["569999", "17836115"]);
    assert_eq!(run_from_file("data/input_7.txt", day7::main).unwrap(),
               ["956", "40149"]);
    assert_eq!(run_from_file("data/input_8.txt", day8::main).unwrap(),
               ["1350", "2085"]);
    assert_eq!(run_from_file("data/input_9.txt", day9::main).unwrap(),
               ["117", "909"]);
}
