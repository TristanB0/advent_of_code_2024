use core::str;
use std::io::Read;

use regex::Regex;

pub fn day3_1() {
    let number_list = read_file(false);

    let sum = sum(number_list);

    println!("Day 3-1: {}", sum);
}

pub fn day3_2() {
    let number_list = read_file(true);

    let sum = sum(number_list);

    println!("Day 3-2: {}", sum);
}

fn read_file(alt: bool) -> Vec<(u16, u16)> {
    let mut file = std::fs::File::open("inputs/day03.txt").expect("File not found.");

    let mut buffer = Vec::new();
    _ = file.read_to_end(&mut buffer);
    let text = str::from_utf8(&buffer).unwrap();

    if !alt {
        let regex = Regex::new(r"(?m)mul\((?<n1>\d{1,3}),(?<n2>\d{1,3})\)").unwrap();
        let numbers: Vec<(u16, u16)> = regex.captures_iter(text)
            .map(|cap| {
                let n1 = cap["n1"].parse().unwrap();
                let n2 = cap["n2"].parse().unwrap();
                (n1, n2)
            })
            .collect();
        return numbers;
    }

    let regex = Regex::new(r"(?m)(?<do>don't\(\)|do\(\))|mul\((?<n1>\d{1,3}),(?<n2>\d{1,3})\)").unwrap();

    let mut numbers: Vec<(u16, u16)> = Vec::new();
    let mut doable = true;
    for cap in regex.captures_iter(text) {
        let is_do = cap.name("do").is_some();
        if is_do {
            if &cap["do"] == "don't()" {
                doable = false;
            }
            if &cap["do"] == "do()" {
                doable = true;
            }
            continue;
        }

        if doable {
            let n1 = cap["n1"].parse().unwrap();
            let n2 = cap["n2"].parse().unwrap();
            numbers.push((n1, n2));
        }
    }

    numbers
}

fn sum(number_list: Vec<(u16, u16)>) -> u32{
    let mut sum: u32 = 0;
    for (n1, n2) in number_list {
        sum += n1 as u32 * n2 as u32;
    }
    sum
}
