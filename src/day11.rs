use core::str;

pub fn day11_1() -> usize {
    let stones = read_file("inputs/day11.txt");
    let res = blink(stones, 25);

    res
}

pub fn day11_2() -> u32 {
    todo!();
}

struct Stone {
    value: u64,
    n_digits: usize,
}

impl Stone {
    fn new(value: u64) -> Self {
        let n_digits = value.to_string().len();

        Self { value, n_digits }
    }

    fn is_len_even(&self) -> bool {
        self.n_digits % 2 == 0
    }

    fn split_half(&self) -> (Stone, Stone) {
        let value_str = self.value.to_string();
        let mid = value_str.len() / 2;
        let part1 = value_str[..mid].parse().unwrap();
        let part2 = value_str[mid..].parse().unwrap();

        (Stone::new(part1), Stone::new(part2))
    }
}

fn read_file(src: &str) -> Vec<Stone> {
    let file = std::fs::File::open(src).expect("File not found.");
    let reader = std::io::BufReader::new(file);
    let line = std::io::BufRead::lines(reader).next().unwrap().unwrap();

    line.split(' ')
        .map(|c| Stone::new(c.parse().unwrap()))
        .collect()
}

fn rules(stones: &Vec<Stone>) -> Vec<Stone> {
    let mut output = Vec::new();

    for stone in stones {
        if stone.value == 0 {
            output.push(Stone::new(1));
        } else if stone.is_len_even() {
            let (part1, part2) = stone.split_half();
            output.push(part1);
            output.push(part2);
        } else {
            output.push(Stone::new(stone.value * 2024));
        }
    }

    output
}

fn blink(stones: Vec<Stone>, n: u32) -> usize {
    let mut output = stones;

    for _ in 0..n {
        output = rules(&output);
    }

    output.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_1() {
        let stones = read_file("tests/day11.txt");
        let res = blink(stones, 25);
        assert_eq!(res, 55312);
    }

    #[test]
    fn test_day11_2() {
        assert_eq!(1, 0);
    }
}
