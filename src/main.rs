#![feature(iter_arith)] // sum is not stable
#![feature(box_syntax)]

use std::io;

#[macro_use]
extern crate scan_fmt;
#[macro_use]
extern crate nom;

extern crate clap;
use clap::{Arg, App};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;

const LATEST: u8 = 21;

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
    let f: fn() -> Vec<String> = match day {
        0 => panic!("don't do that"),
        1 => day01::main,
        2 => day02::main,
        3 => day03::main,
        4 => day04::main,
        5 => day05::main,
        6 => day06::main,
        7 => day07::main,
        8 => day08::main,
        9 => day09::main,
        10 => day10::main,
        11 => day11::main,
        12 => day12::main,
        13 => day13::main,
        14 => day14::main,
        15 => day15::main,
        16 => day16::main,
        17 => day17::main,
        18 => day18::main,
        19 => day19::main,
        20 => day20::main,
        21 => day21::main,
        _ => panic!("not there yet"),
    };

    let results = f();
    println!("Day {}", day);
    for output in &results {
        println!("  {}", output);
    }
    Ok(results)
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
    assert_eq!(run_one(12).unwrap(), ["191164", "87842"]);
    assert_eq!(run_one(13).unwrap(), ["733", "725"]);
    assert_eq!(run_one(14).unwrap(), ["2640", "1102"]);
    assert_eq!(run_one(15).unwrap(), ["222870", "117936"]);
    assert_eq!(run_one(16).unwrap(), ["[40]", "[241]"]);
    assert_eq!(run_one(17).unwrap(), ["654", "57"]);
    assert_eq!(run_one(18).unwrap(), ["768", "781"]);
    assert_eq!(run_one(19).unwrap(), ["535", "212"]);
    assert_eq!(run_one(20).unwrap(), ["665280", "705600"]);
    assert_eq!(run_one(21).unwrap(), ["111", "188"]);
}
