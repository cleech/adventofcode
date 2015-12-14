#![feature(iter_arith)] // sum is not stable
#![feature(box_syntax)]

use std::io;
use std::fs::File;
use std::io::prelude::*;

#[macro_use]
extern crate scan_fmt;

extern crate clap;
use clap::{Arg, App};

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
mod day11;
mod day12;
mod day13;
mod day14;

const LATEST: u8 = 14;

fn main() {
    let args = App::new("AdventOfCode")
                   .arg(Arg::with_name("DAY")
                            .short("d")
                            .long("day")
                            .takes_value(true))
                   .get_matches();

    let day = args.value_of("DAY")
                  .unwrap_or("0")
                  .parse::<u8>()
                  .expect("Invalid value for day");

    try_main(day).unwrap()
}

fn try_main(day: u8) -> io::Result<()> {
    match day {
        0 => {
            for day in 1..(LATEST + 1) {
                try!(run_one(day));
            }
        }
        1...LATEST => {
            try!(run_one(day));
        }
        _ => {}
    };
    Ok(())
}

fn run_one(day: u8) -> io::Result<Vec<String>> {
    let input = format!("data/input_{}.txt", day);
    let f: fn(&str) -> Vec<String> = match day {
        0 => panic!("don't do that"),
        1 => day1::main,
        2 => day2::main,
        3 => day3::main,
        4 => day4::main,
        5 => day5::main,
        6 => day6::main,
        7 => day7::main,
        8 => day8::main,
        9 => day9::main,
        10 => day10::main,
        11 => day11::main,
        12 => day12::main,
        13 => day13::main,
        14 => day14::main,
        _ => panic!("not there yet"),
    };

    let results = try!(run_from_file(&input, f));
    println!("Day {}", day);
    for output in &results {
        println!("  {}", output);
    }
    Ok(results)
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
    assert_eq!(run_one(1).unwrap(), ["74", "1795"]);
    assert_eq!(run_one(2).unwrap(), ["1586300", "3737498"]);
    assert_eq!(run_one(3).unwrap(), ["2565", "2639"]);
    assert_eq!(run_one(4).unwrap(), ["282749", "9962624"]);
    assert_eq!(run_one(5).unwrap(), ["236", "51"]);
    assert_eq!(run_one(6).unwrap(), ["569999", "17836115"]);
    assert_eq!(run_one(7).unwrap(), ["956", "40149"]);
    assert_eq!(run_one(8).unwrap(), ["1350", "2085"]);
    assert_eq!(run_one(9).unwrap(), ["117", "909"]);
    assert_eq!(run_one(10).unwrap(), ["439880", "6230578"]);
    assert_eq!(run_one(11).unwrap(), ["vzbxxyzz", "vzcaabcc"]);
    assert_eq!(run_one(12).unwrap(),  ["191164","87842"]);
    assert_eq!(run_one(13).unwrap(), ["733", "725"]);
    assert_eq!(run_one(14).unwrap(), ["2640", "1102"]);
}
