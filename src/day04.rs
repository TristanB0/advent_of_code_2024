use core::str;

pub fn day4_1() -> u32 {
    let text_array = read_file_to_array("inputs/day04.txt");

    analyze_array(text_array)
}

pub fn day4_2() -> u32 {
    let text_array = read_file_to_array("inputs/day04.txt");

    analylze_array_2(text_array)
}

fn read_file_to_array(src: &str) -> Vec<Vec<char>> {
    let mut output_array = Vec::new();

    let file = std::fs::File::open(src).expect("File not found.");
    let reader = std::io::BufReader::new(file);

    for line in std::io::BufRead::lines(reader) {
        let line = line.unwrap();
        let line_vec: Vec<char> = line.chars().collect();
        output_array.push(line_vec);
    }

    output_array
}

/// Find how many times "XMAS" appear in the text.
fn analyze_array(text_array: Vec<Vec<char>>) -> u32 {
    let horizontal_count = analyze_horizontal(text_array.clone());
    let vertical_count = analyze_vertical(text_array.clone());
    let diagonal_count = analyze_diagonal(text_array);

    horizontal_count + vertical_count + diagonal_count
}

fn analyze_line(line: &str) -> u32 {
    let mut count = 0;
    let pattern = "XMAS";
    let pattern_len = pattern.len();

    let end_limit = line.len() - pattern_len + 1;
    for i in 0..end_limit {
        if &line[i..(i + pattern_len)] == pattern
            || &line[i..(i + pattern_len)] == pattern.chars().rev().collect::<String>()
        {
            count += 1;
        }
    }

    count
}

fn analyze_horizontal(text: Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    for line in text.iter() {
        let line_text: String = line.iter().collect();

        count += analyze_line(&line_text);
    }

    count
}

fn analyze_vertical(text: Vec<Vec<char>>) -> u32 {
    let mut alt_text: Vec<String> = vec![String::default(); text[0].len()];
    for i in 0..text.len() {
        for j in 0..text[i].len() {
            alt_text[j].push(text[i][j]);
        }
    }

    let mut count = 0;
    for line in alt_text.iter() {
        count += analyze_line(&line);
    }

    count
}

/// Note: Terrible code, efficiency-wise or cleanliness-wise.
fn analyze_diagonal(text: Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    let rows = text.len() as i32;
    let cols = text[0].len() as i32;

    for i in 0..rows {
        for j in 0..cols {
            let mut diagonals: Vec<String> = vec![];
            let mut diagonal = String::new();
            let mut diag_size = 0;
            let mut row = i as i32;
            let mut col = j as i32;

            // Down-right
            while diag_size != 4 && row < rows && col < cols {
                diagonal.push(text[row as usize][col as usize]);
                diag_size += 1;
                row += 1;
                col += 1;
            }
            diagonals.push(diagonal);
            (diag_size, row, col, diagonal) = (0, i, j, String::default());

            // Down-left
            while diag_size != 4 && row < rows && col >= 0 {
                diagonal.push(text[row as usize][col as usize]);
                diag_size += 1;
                row += 1;
                col -= 1;
            }
            diagonals.push(diagonal);

            // Useless because of reverse in analyze_line
            // (diag_size, row, col, diagonal) = (0, i, j, String::default());
            // // Up-right
            // while diag_size != 4 && row >= 0 && col < cols {
            //     diagonal.push(text[row as usize][col as usize]);
            //     diag_size += 1;
            //     row -= 1;
            //     col += 1;
            // }
            // diagonals.push(diagonal);
            // (diag_size, row, col, diagonal) = (0, i, j, String::default());

            // // Up-left
            // while diag_size != 4 && row >= 0 && col >= 0 {
            //     diagonal.push(text[row as usize][col as usize]);
            //     diag_size += 1;
            //     row -= 1;
            //     col -= 1;
            // }
            // diagonals.push(diagonal);

            for diagonal in diagonals.iter() {
                if diagonal.len() == 4 {
                    count += analyze_line(&diagonal);
                }
            }
        }
    }

    count
}

/// Find how many times a X of "MAS" appears in the text.
fn analylze_array_2(text_array: Vec<Vec<char>>) -> u32 {
    let mut count = 0;
    let rows = text_array.len();
    let cols = text_array[0].len();

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if text_array[i][j] == 'A' {
                match text_array[i - 1][j - 1] {
                    'M' => {
                        if text_array[i + 1][j + 1] == 'S' {
                            match text_array[i - 1][j + 1] {
                                'S' => {
                                    if text_array[i + 1][j - 1] == 'M' {
                                        count += 1;
                                    }
                                }
                                'M' => {
                                    if text_array[i + 1][j - 1] == 'S' {
                                        count += 1;
                                    }
                                }
                                _ => continue,
                            }
                        }
                    }
                    'S' => {
                        if text_array[i + 1][j + 1] == 'M' {
                            match text_array[i - 1][j + 1] {
                                'M' => {
                                    if text_array[i + 1][j - 1] == 'S' {
                                        count += 1;
                                    }
                                }
                                'S' => {
                                    if text_array[i + 1][j - 1] == 'M' {
                                        count += 1;
                                    }
                                }
                                _ => continue,
                            }
                        }
                    }
                    _ => continue,
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
    fn test_day4_1() {
        let text_array = read_file_to_array("tests/day04.txt");
        let count = analyze_array(text_array);
        assert_eq!(count, 18);
    }

    #[test]
    fn test_day4_2() {
        let text_array = read_file_to_array("tests/day04.txt");
        let count = analylze_array_2(text_array);
        assert_eq!(count, 9);
    }
}
