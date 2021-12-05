use crate::aoc::AocSolution2;

fn p1(input: &str) -> String {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let length = lines.len();
    let width: usize = lines.first().unwrap_or(&Vec::new()).len();
    let mut gamma = String::with_capacity(width);
    let mut epsilon = String::with_capacity(width);
    for i in 0..width {
        let mut one_count = 0;
        for line in &lines {
            match line[i] {
                '0' => one_count += 1,
                _ => {}
            }
        }

        if length - one_count > one_count {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }

    (isize::from_str_radix(&gamma, 2).unwrap_or_default()
        * isize::from_str_radix(&epsilon, 2).unwrap_or_default())
    .to_string()
}

fn p2(input: &str) -> String {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let length = lines.len();
    let width: usize = lines.first().unwrap_or(&Vec::new()).len();

    fn filter_line(lines: &Vec<Vec<char>>, width: usize, length: usize, keep_char: char) -> usize {
        let keep_majority = match keep_char {
            '1' => true,
            _ => false,
        };

        let mut rows_to_visit: Vec<usize> = (0..length).collect();
        let mut next_rows_to_visit: Vec<usize> = Vec::new();

        for i in 0..width {
            let mut one_count = 0;
            for row_id in &rows_to_visit {
                match lines[*row_id][i] {
                    '1' => one_count += 1,
                    _ => {}
                }
            }

            let majority_one = one_count > rows_to_visit.len() - one_count;
            let equal = one_count == rows_to_visit.len() - one_count;

            for row_id in &rows_to_visit {
                match lines[*row_id][i] {
                    '1' => {
                        if equal {
                            if keep_char == '1' {
                                next_rows_to_visit.push(*row_id);
                            }
                        } else if majority_one && keep_majority {
                            next_rows_to_visit.push(*row_id);
                        } else if !majority_one && !keep_majority {
                            next_rows_to_visit.push(*row_id);
                        }
                    }
                    _ => {
                        if equal {
                            if keep_char == '0' {
                                next_rows_to_visit.push(*row_id);
                            }
                        } else if majority_one && !keep_majority {
                            next_rows_to_visit.push(*row_id);
                        } else if !majority_one && keep_majority {
                            next_rows_to_visit.push(*row_id);
                        }
                    }
                }
            }

            if next_rows_to_visit.len() == 1 {
                return *next_rows_to_visit.first().unwrap_or(&0);
            }

            rows_to_visit = next_rows_to_visit;
            next_rows_to_visit = Vec::new();
        }

        *rows_to_visit.first().unwrap_or(&0)
    }

    let ox: String = lines[filter_line(&lines, width, length, '1')]
        .iter()
        .collect();
    let o2: String = lines[filter_line(&lines, width, length, '0')]
        .iter()
        .collect();

    (isize::from_str_radix(&ox, 2).unwrap_or_default()
        * isize::from_str_radix(&o2, 2).unwrap_or_default())
    .to_string()
}

pub static SOLUTION: AocSolution2 = AocSolution2 {
    year: 2021,
    day: 3,
    solve_p1: p1,
    solve_p2: p2,
};
