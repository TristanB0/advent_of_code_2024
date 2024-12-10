use core::str;
use std::io::Read;

use regex::Regex;

pub fn day3_1() {
    let number_list = read_file();

    let mut sum: u32 = 0;
    for (n1, n2) in number_list {
        sum += n1 as u32 * n2 as u32;
    }

    println!("Day 3-1: {}", sum);
}

pub fn day3_2() {
    println!("Day 3-2: {}", "TODO");
}

fn read_file() -> Vec<(u16, u16)> {
    let mut file = std::fs::File::open("inputs/day03.txt").expect("File not found.");
    let regex = Regex::new(r"(?m)mul\((?<n1>\d{1,3}),(?<n2>\d{1,3})\)").unwrap();

    let mut buffer = Vec::new();
    _ = file.read_to_end(&mut buffer);
    let text = str::from_utf8(&buffer).unwrap();

    let mut numbers: Vec<(u16, u16)> = Vec::new();
    for cap in regex.captures_iter(text) {
        let n1 = cap["n1"].parse().unwrap();
        let n2 = cap["n2"].parse().unwrap();
        numbers.push((n1, n2));
    }

    numbers
}
