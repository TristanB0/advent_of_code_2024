use core::str;

pub fn day6_1() -> u32 {
    let grid = read_file("inputs/day06.txt");
    let count = predict_positions(grid);

    count
}

pub fn day6_2() -> u32 {
    todo!();
    // IDEA: Save the guard path, and test all possible coordinated to check for a loop
}

type Grid = Vec<Vec<char>>;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_file(src: &str) -> Grid {
    let mut grid = vec![];

    let file = std::fs::File::open(src).expect("File not found.");
    let reader = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(reader) {
        let line = line.unwrap();
        let line_vec: Vec<char> = line.chars().collect();
        grid.push(line_vec);
    }

    grid
}

fn predict_positions(grid: Grid) -> u32 {
    let mut count = 0;

    let mut grid = grid.clone();
    let mut guard = locate_guard(&grid).unwrap();
    let borders = (grid.len(), grid[0].len());
    let mut direction = Direction::Up;

    while guard.0 as i32 >= 0 && guard.0 < borders.0 && guard.1 as i32 >= 0 && guard.1 < borders.1 {
        if grid[guard.0][guard.1] != 'X' {
            count += 1;
        }
        grid[guard.0][guard.1] = 'X';

        match direction {
            Direction::Up => {
                if guard.0 as i32 - 1 < 0 {
                    break;
                }
                if grid[guard.0 - 1][guard.1] == '#' {
                    direction = Direction::Right;
                    grid[guard.0 - 1][guard.1] = 'X';
                    guard.0 += 1;
                }
                guard.0 -= 1;
            }
            Direction::Down => {
                if guard.0 + 1 >= borders.0 {
                    break;
                }
                if grid[guard.0 + 1][guard.1] == '#' {
                    direction = Direction::Left;
                    guard.0 -= 1;
                }
                guard.0 += 1;
            }
            Direction::Left => {
                if guard.1 as i32 - 1 < 0 {
                    break;
                }
                if grid[guard.0][guard.1 - 1] == '#' {
                    direction = Direction::Up;
                    guard.1 += 1;
                }
                guard.1 -= 1;
            }
            Direction::Right => {
                if guard.1 + 1 >= borders.1 {
                    break;
                }
                if grid[guard.0][guard.1 + 1] == '#' {
                    direction = Direction::Down;
                    guard.1 -= 1;
                }
                guard.1 += 1;
            }
        }
    }

    count
}

fn locate_guard(grid: &Grid) -> Option<(usize, usize)> {
    for (i, v) in grid.iter().enumerate() {
        for (j, &h) in v.iter().enumerate() {
            if h == '^' {
                return Some((i, j));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_1() {
        let grid = read_file("tests/day06.txt");
        let count = predict_positions(grid);
        assert_eq!(count, 41);
    }

    #[test]
    fn test_day6_2() {
        assert_eq!(1, 0);
    }
}
