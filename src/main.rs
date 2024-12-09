mod day1;
mod day2;

use crate::day1::{day1_1, day1_2};
use crate::day2::{day2_1, day2_2};

fn main() {
    println!("BEGIN Day 1");
    day1_1();
    day1_2();
    println!("END Day 1");

    println!("BEGIN Day 2");
    day2_1();
    day2_2();
    println!("END Day 2");
}
