mod day01;
mod day02;
mod day03;
mod day04;

use crate::day01::{day1_1, day1_2};
use crate::day02::{day2_1, day2_2};
use crate::day03::{day3_1, day3_2};
use crate::day04::{day4_1, day4_2};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: u8 = (&args[1]).parse().unwrap();

    println!("BEGIN DAY {:02}", day);

    match day {
        1 => {
            println!("Day 1-1: {}", day1_1());
            println!("Day 1-2: {}", day1_2());
        }
        2 => {
            println!("Day 2-1: {}", day2_1());
            println!("Day 2-2: {}", day2_2());
        }
        3 => {
            println!("Day 3-1: {}", day3_1());
            println!("Day 3-2: {}", day3_2());
        }
        4 => {
            println!("Day 4-1: {}", day4_1());
            println!("Day 4-2: {}", day4_2());
        }
        _ => {
            println!("Invalid day.");
        }
    }

    println!("END DAY {:02}", day);
}
