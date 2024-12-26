use core::str;

pub fn day7_1() -> u64 {
    let equations = read_file("inputs/day07.txt");
    let sum = test_configurations(equations);

    sum
}

pub fn day7_2() -> u64 {
    todo!();
}

#[derive(Debug)]
struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

fn read_file(src: &str) -> Vec<Equation> {
    let mut equations = Vec::new();

    let file = std::fs::File::open(src).expect("File not found.");
    let reader = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(reader) {
        let line = line.unwrap();
        let mut equation = Equation {
            test_value: 0,
            numbers: Vec::new(),
        };

        let eq = line.split_once(':').unwrap();
        equation.test_value = eq.0.parse().unwrap();

        let mut nums = eq.1.split_whitespace();

        while let Some(num) = nums.next() {
            let num = num.parse().unwrap();
            equation.numbers.push(num);
        }

        equations.push(equation);
    }

    equations
}

fn test_configurations(equations: Vec<Equation>) -> u64 {
    let mut sum = 0;

    for eq in equations {
        let num_size = eq.numbers.len() as u32;
        let n_permutations = 2u64.pow(num_size);

        for i in 0..n_permutations {
            let mut tmp_res = eq.numbers[0];

            let perm = generate_perm(i, num_size as usize);

            for j in 1..num_size {
                let j = j as usize;
                match perm.chars().nth(j - 1).unwrap() {
                    '0' => tmp_res += eq.numbers[j],
                    '1' => tmp_res *= eq.numbers[j],
                    _ => panic!("Invalid permutation."),
                }
            }

            if tmp_res == eq.test_value {
                sum += eq.test_value;
                break;
            }
        }
    }

    sum
}

fn generate_perm(n: u64, size: usize) -> String {
    let n_txt = format!("{:0size$b}", n, size = size);
    n_txt.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_1() {
        let equations = read_file("tests/day07.txt");
        let sum = test_configurations(equations);
        assert_eq!(sum, 3749);
    }

    #[test]
    fn test_day7_2() {
        assert_eq!(1, 0);
    }
}
