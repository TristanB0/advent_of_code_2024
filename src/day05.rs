use core::str;
use std::collections::{HashMap, HashSet};

pub fn day5_1() -> u32 {
    let (rules, updates) = read_file("inputs/day05.txt");
    let valid_idx = identify_valid_updates(rules, updates.clone());

    middle_calculation(updates, valid_idx)
}

pub fn day5_2() -> u32 {
    todo!();
    // IDEA: For all errors, swap the pages concerned
}

type Rules = HashMap<u32, HashSet<u32>>;
type Updates = Vec<Vec<u32>>;

fn read_file(src: &str) -> (Rules, Updates) {
    let mut rules: Rules = HashMap::new();
    let mut updates = Vec::new();

    let file = std::fs::File::open(src).expect("File not found.");
    let reader = std::io::BufReader::new(file);

    let mut is_beginning = true;
    for line in std::io::BufRead::lines(reader) {
        let line = line.unwrap();
        if line.is_empty() {
            is_beginning = false;
            continue;
        }
        if is_beginning {
            // Parse rules
            let mut nums = line.split('|');

            let before: u32 = nums.next().unwrap().parse().unwrap();
            let after: u32 = nums.next().unwrap().parse().unwrap();

            if rules.contains_key(&after) {
                rules.get_mut(&after).unwrap().insert(before);
            } else {
                rules.insert(after, HashSet::from([before]));
            }
        } else {
            // Parse updates
            let nums = line.split(',');
            let mut update = Vec::new();

            for num in nums {
                let value: u32 = num.parse().unwrap();
                update.push(value);
            }

            updates.push(update);
        }
    }

    (rules, updates)
}

fn identify_valid_updates(rules: Rules, updates: Updates) -> Vec<usize> {
    let mut valid_updates = Vec::new();

    for (idx, update) in updates.iter().enumerate() {
        let mut is_valid = true;

        'loop_update: for (i, &v) in update.iter().enumerate() {
            if let Some(dependencies) = rules.get(&v) {
                for &dependency in dependencies {
                    if update[i..].contains(&dependency) && !update[..i].contains(&dependency) {
                        is_valid = false;
                        break 'loop_update;
                    }
                }
            }
        }

        if is_valid {
            valid_updates.push(idx);
        }
    }

    valid_updates
}

fn middle_calculation(updates: Updates, valid_idx: Vec<usize>) -> u32 {
    let mut sum = 0;
    for i in valid_idx {
        let middle_idx = updates[i].len() / 2;
        sum += updates[i][middle_idx];
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_1() {
        let (rules, updates) = read_file("tests/day05.txt");
        let valid_idx = identify_valid_updates(rules, updates.clone());
        let sum = middle_calculation(updates, valid_idx);
        assert_eq!(sum, 143);
    }

    #[test]
    fn test_day5_2() {
        assert_eq!(1, 0);
    }
}
