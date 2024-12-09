mod day01;
mod day02;

use crate::day01::{day1_1, day1_2};
use crate::day02::{day2_1, day2_2};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let day: u32 = (&args[1]).parse().unwrap();

    println!("BEGIN DAY {:02}", day);

    match day {
        1 => {
            day1_1();
            day1_2();
        },
        2 => {
            day2_1();
            day2_2();
        },
        _ => {
            println!("Invalid day.");
        }
    }

    println!("END DAY {:02}", day);
}
