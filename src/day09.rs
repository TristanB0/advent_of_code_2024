use std::fs;

pub fn day9_1() -> u64 {
    let data = read_file("inputs/day09.txt");
    let disk = represent_data(data);
    let new_disk = move_blocks(disk);
    let sum = checksum(new_disk);

    sum
}

pub fn day9_2() -> u32 {
    todo!();
}

fn read_file(src: &str) -> String {
    let diskmap = fs::read_to_string(src).unwrap();

    diskmap.trim().to_string()
}

fn represent_data(content: String) -> Vec<char> {
    let mut disk_repr = String::new();
    for (i, c) in content.chars().enumerate() {
        for _ in 0..c.to_digit(10).unwrap() {
            if i % 2 == 0 {
                disk_repr.push((i / 2).to_string().chars().next().unwrap());
            } else {
                disk_repr.push('.');
            }
        }
    }

    disk_repr.chars().collect()
}

fn move_blocks(mut data: Vec<char>) -> Vec<char> {
    let mut left: usize = 0;
    let mut right = data.len() - 1;

    while left < right {
        while data[left] != '.' && left < right {
            left += 1;
        }

        while data[right] != '.' && data[left] == '.' {
            data[left] = data[right];
            data[right] = '.';
            right -= 1;
            left += 1;
        }

        while data[right] == '.' && right > left {
            right -= 1;
        }
    }

    data
}

fn checksum(data: Vec<char>) -> u64 {
    let mut sum = 0;

    for (i, c) in data.iter().enumerate() {
        if *c == '.' {
            break;
        }

        sum += i as u64 * c.to_digit(10).unwrap() as u64;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_1() {
        let data = read_file("tests/day09.txt");
        let disk = represent_data(data);
        let new_disk = move_blocks(disk);
        let sum = checksum(new_disk);
        assert_eq!(sum, 1928);
    }

    #[test]
    fn test_day9_2() {
        assert_eq!(1, 0);
    }
}
