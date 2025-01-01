use core::str;
use std::collections::HashSet;

pub fn day10_1() -> u32 {
    let file = read_file("inputs/day10.txt");
    let count = count_trailheads(file);

    count
}

pub fn day10_2() -> u32 {
    todo!();
}

fn read_file(src: &str) -> Vec<Vec<char>> {
    let mut grid_data: Vec<Vec<char>> = Vec::new();

    let file = std::fs::File::open(src).expect("File not found.");
    let reader = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(reader) {
        let line = line.unwrap();
        let line: Vec<char> = line.chars().map(|c| c).collect();
        grid_data.push(line);
    }

    grid_data
}

fn count_trailheads(grid: Vec<Vec<char>>) -> u32 {
    let mut count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '0' {
                let mut discovered: HashSet<(usize, usize)> = HashSet::new();
                count += look_around_case(&grid, i, j, '1', &mut discovered); // Look for '1'
                discovered.clear();
            }
        }
    }

    count
}

fn look_around_case(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    value: char,
    discovered: &mut HashSet<(usize, usize)>,
) -> u32 {
    let mut count = 0;
    let value_u32 = value.to_digit(16).unwrap(); // Base 16 to reach 10

    if value_u32 == 10 {
        if discovered.contains(&(x, y)) {
            return 0;
        }

        discovered.insert((x, y));

        return 1;
    }

    for i in -1isize..=1 {
        for j in -1isize..=1 {
            if (i == 0 && j == 0) // Center
                // Diagonals
                || (i == -1 && j == -1)
                || (i == -1 && j == 1)
                || (i == 1 && j == -1)
                || (i == 1 && j == 1)
                // Borders
                || (i == -1 && x == 0)
                || (j == -1 && y == 0)
                || (i == 1 && x == grid.len() - 1)
                || (j == 1 && y == grid[0].len() - 1)
            {
                continue;
            }

            let x = x as isize;
            let y = y as isize;

            if grid[(x + i) as usize][(y + j) as usize] == value {
                let new_value = char::from_digit(value_u32 + 1, 16).unwrap();
                count += look_around_case(
                    grid,
                    (x + i) as usize,
                    (y + j) as usize,
                    new_value,
                    discovered,
                );
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_1() {
        let file = read_file("tests/day10.txt");
        let count = count_trailheads(file);
        assert_eq!(count, 36);
    }

    #[test]
    fn test_day10_2() {
        assert_eq!(1, 0);
    }
}
