pub fn day2_1() {
    let reports = read_file();

    let mut res = 0;
    for report in reports {
        let safe1 =
            report.is_sorted_by(|a, b| (a < b) && (a.abs_diff(*b) <= 3) && (a.abs_diff(*b) > 0));
        let safe2 =
            report.is_sorted_by(|a, b| (a > b) && (a.abs_diff(*b) <= 3) && (a.abs_diff(*b) > 0));

        if safe1 || safe2 {
            res += 1
        }
    }

    println!("Day 2-1: {}", res);
}

pub fn day2_2() {
    let reports = read_file();

    let mut res = 0;
    for report in reports {
        let safe1 =
            report.is_sorted_by(|a, b| (a < b) && (a.abs_diff(*b) <= 3) && (a.abs_diff(*b) > 0));
        if safe1 {
            res += 1;
            continue;
        }

        let safe2 =
            report.is_sorted_by(|a, b| (a > b) && (a.abs_diff(*b) <= 3) && (a.abs_diff(*b) > 0));
        if safe2 {
            res += 1;
            continue;
        }

        let mut safe3 = false;
        for i in 0..report.len() {
            let mut report_bis = report.clone();
            _ = report_bis.remove(i);
            let safe1_bis = report_bis
                .is_sorted_by(|a, b| (a < b) && (a.abs_diff(*b) <= 3) && (a.abs_diff(*b) > 0));
            if safe1_bis {
                safe3 = !safe3;
                break;
            }

            let safe2_bis = report_bis
                .is_sorted_by(|a, b| (a > b) && (a.abs_diff(*b) <= 3) && (a.abs_diff(*b) > 0));
            if safe2_bis {
                safe3 = !safe3;
                break;
            }
        }
        if safe3 {
            res += 1;
        }
    }

    println!("Day 2-2: {}", res);
}

fn read_file() -> Vec<Vec<u32>> {
    let mut output_vec = vec![];

    let file =
        std::fs::File::open("inputs/day2.txt")
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
