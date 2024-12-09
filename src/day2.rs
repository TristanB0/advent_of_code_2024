pub fn day2_1() {
    let reports = read_file();

    let mut res = 0;
    for report in reports {
        let safe1 = report.is_sorted_by(|a, b| {
            (a < b) && (a.abs_diff(*b) <= 3) && (a.abs_diff(*b) > 0)
        });
        let safe2 = report.is_sorted_by(|a, b| {
            (a > b) && (a.abs_diff(*b) <= 3) && (a.abs_diff(*b) > 0)
        });

        if safe1 || safe2 {
            res += 1
        }
    }

    println!("Day 2-1: {}", res);
}

pub fn day2_2() {
    todo!("Need day2_1 first");
}

fn read_file() -> Vec<Vec<u32>> {
    let mut output_vec = vec![];

    let file =
        std::fs::File::open("/home/tristan/Documents/Projects/advent_of_code/inputs/day2.txt")
            .expect("File not found.");
    let reader = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(reader) {
        let line = line.unwrap();
        let line_vec_it = line.split_whitespace();
        let line_vec: Vec<u32> = line_vec_it.map(|w| w.parse().unwrap()).collect();
        output_vec.push(line_vec);
    }

    output_vec
}
