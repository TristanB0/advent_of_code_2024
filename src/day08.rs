use core::str;
use std::collections::{HashMap, HashSet};

pub fn day8_1() -> u32 {
    let (size, data) = read_file("inputs/day08.txt");
    let sum = count_antipodes(size, data);

    sum
}

pub fn day8_2() -> u32 {
    todo!();
}

fn read_file(src: &str) -> ((u32, u32), HashMap<u8, Vec<(u32, u32)>>) {
    let mut grid_data: HashMap<u8, Vec<(u32, u32)>> = HashMap::new();

    let file = std::fs::File::open(src).expect("File not found.");
    let reader = std::io::BufReader::new(file);

    let mut line_count = 0;
    let mut line_size = 0;
    for line in std::io::BufRead::lines(reader) {
        let line = line.unwrap();
        line_size = line.len() as u32;
        for (idx, val) in line.chars().enumerate() {
            match val {
                '.' => {
                    continue;
                }
                value => {
                    grid_data
                        .entry(value as u8)
                        .and_modify(|v| v.push((line_count, idx as u32)))
                        .or_insert(vec![(line_count, idx as u32)]);
                }
            }
        }

        line_count += 1;
    }

    ((line_count as u32, line_size), grid_data)
}

fn count_antipodes(grid_size: (u32, u32), antennas: HashMap<u8, Vec<(u32, u32)>>) -> u32 {
    let mut installed_loc = HashSet::new();
    let mut count = 0;

    for (_, locs) in antennas.iter() {
        for loc in locs {
            for l in locs {
                // Skip if same location
                if l == loc {
                    continue;
                }

                let diff = (l.0 as i32 - loc.0 as i32, l.1 as i32 - loc.1 as i32);
                let new_loc_1 = (l.0 as i32 + diff.0, l.1 as i32 + diff.1);
                let new_loc_2 = (loc.0 as i32 - diff.0, loc.1 as i32 - diff.1);
                if !(installed_loc.contains(&new_loc_1)
                    || new_loc_1.0 < 0
                    || new_loc_1.0 >= grid_size.0 as i32
                    || new_loc_1.1 < 0
                    || new_loc_1.1 >= grid_size.1 as i32)
                {
                    installed_loc.insert(new_loc_1);
                    count += 1;
                }

                if !(installed_loc.contains(&new_loc_2)
                    || new_loc_2.0 < 0
                    || new_loc_2.0 >= grid_size.0 as i32
                    || new_loc_2.1 < 0
                    || new_loc_2.1 >= grid_size.1 as i32)
                {
                    installed_loc.insert(new_loc_2);
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_1() {
        let (size, data) = read_file("tests/day08.txt");
        let sum = count_antipodes(size, data);
        assert_eq!(sum, 14);
    }

    #[test]
    fn test_day8_2() {
        assert_eq!(1, 0);
    }
}
