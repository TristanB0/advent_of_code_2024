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

use crate::day01::{day1_1, day1_2};
use crate::day02::{day2_1, day2_2};
use crate::day03::{day3_1, day3_2};
use crate::day04::{day4_1, day4_2};
use crate::day05::{day5_1, day5_2};
use crate::day06::{day6_1, day6_2};
use crate::day07::{day7_1, day7_2};
use crate::day08::{day8_1, day8_2};
use crate::day09::{day9_1, day9_2};
use crate::day10::{day10_1, day10_2};
use crate::day11::{day11_1, day11_2};

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
        5 => {
            println!("Day 5-1: {}", day5_1());
            println!("Day 5-2: {}", day5_2());
        }
        6 => {
            println!("Day 6-1: {}", day6_1());
            println!("Day 6-2: {}", day6_2());
        }
        7 => {
            println!("Day 7-1: {}", day7_1());
            println!("Day 7-2: {}", day7_2());
        }
        8 => {
            println!("Day 8-1: {}", day8_1());
            println!("Day 8-2: {}", day8_2());
        }
        9 => {
            println!("Day 9-1: {}", day9_1());
            println!("Day 9-2: {}", day9_2());
        }
        10 => {
            println!("Day 10-1: {}", day10_1());
            println!("Day 10-2: {}", day10_2());
        }
        11 => {
            println!("Day 11-1: {}", day11_1());
            println!("Day 11-2: {}", day11_2());
        }
        _ => {
            println!("Invalid day.");
        }
    }

    println!("END DAY {:02}", day);
}
