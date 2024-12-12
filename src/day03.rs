use core::str;
use regex::Regex;
use std::io::Read;

pub fn day3_1() -> u32 {
    let text = read_file("inputs/day03.txt");
    let number_list = parse_file(&text, false);

    sum(number_list)
}

pub fn day3_2() -> u32 {
    let text = read_file("inputs/day03.txt");
    let number_list = parse_file(&text, true);

    sum(number_list)
}

fn parse_file(text: &str, alt: bool) -> Vec<(u16, u16)> {
    if !alt {
        let regex = Regex::new(r"(?m)mul\((?<n1>\d{1,3}),(?<n2>\d{1,3})\)").unwrap();
        let numbers: Vec<(u16, u16)> = regex
            .captures_iter(text)
            .map(|cap| {
                let n1 = cap["n1"].parse().unwrap();
                let n2 = cap["n2"].parse().unwrap();
                (n1, n2)
            })
            .collect();
        return numbers;
    }

    let regex =
        Regex::new(r"(?m)(?<do>don't\(\)|do\(\))|mul\((?<n1>\d{1,3}),(?<n2>\d{1,3})\)").unwrap();

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

fn read_file(src: &str) -> String {
    let mut file = std::fs::File::open(src).expect("File not found.");

    let mut buffer = Vec::new();
    _ = file.read_to_end(&mut buffer);
    let text = str::from_utf8(&buffer).unwrap();

    text.to_string()
}

fn sum(number_list: Vec<(u16, u16)>) -> u32 {
    let mut sum: u32 = 0;
    for (n1, n2) in number_list {
        sum += n1 as u32 * n2 as u32;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_1() {
        let text = read_file("tests/day03-1.txt");
        let numbers = parse_file(&text, false);
        assert_eq!(sum(numbers), 161);
    }

    #[test]
    fn test_day3_2() {
        let text = read_file("tests/day03-2.txt");
        let numbers = parse_file(&text, true);
        assert_eq!(sum(numbers), 48);
    }
}
