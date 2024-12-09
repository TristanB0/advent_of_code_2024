pub fn day1_1() {
    let (mut a, mut b) = read_from_file();

    a.sort();
    b.sort();

    println!("Day 1-1: {}", distance(a, b));
}

pub fn day1_2() {
    let (a, b) = read_from_file();

    println!("Day 1-2: {}", similarity(a, b));
}

fn read_from_file() -> (Vec<u32>, Vec<u32>) {
    let mut a = Vec::new();
    let mut b = Vec::new();

    let file =
        std::fs::File::open("inputs/day1.txt")
            .unwrap();
    let reader = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(reader) {
        let line = line.unwrap();
        let mut nums = line.split_whitespace();

        a.push(nums.next().unwrap().parse().unwrap());
        b.push(nums.next().unwrap().parse().unwrap());
    }

    (a, b)
}

fn distance(a: Vec<u32>, b: Vec<u32>) -> u32 {
    if a.len() != b.len() {
        panic!("Arrays must be the same length.");
    }

    let mut sum = 0;
    for i in 0..a.len() {
        if a[i] < b[i] {
            sum += b[i] - a[i];
        } else {
            sum += a[i] - b[i];
        }
    }

    sum
}

fn similarity(a: Vec<u32>, b: Vec<u32>) -> u32 {
    let mut sum = 0;
    for ai in a {
        sum += ai * count_value(b.clone(), ai);
    }
    sum
}

fn count_value(v: Vec<u32>, n: u32) -> u32 {
    let mut sum = 0;
    for i in v {
        if i == n {
            sum += 1;
        }
    }
    sum
}
