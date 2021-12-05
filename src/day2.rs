use crate::aoc::AocSolution2;

fn p1(input: &str) -> String {
    let mut horizontal = 0;
    let mut depth = 0;
    let lines = input.lines();
    for line in lines {
        let mut split = line.split_whitespace();
        let direction: &str = split.next().unwrap_or_default();
        let amount: i32 = split.next().unwrap_or_default().parse().unwrap_or_default();
        match direction {
            "forward" => horizontal += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            unknown_direction => panic!("unexpected {}", unknown_direction),
        }
    }
    (horizontal * depth).to_string()
}

fn p2(input: &str) -> String {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    let lines = input.lines();
    for line in lines {
        let mut split = line.split_whitespace();
        let direction: &str = split.next().unwrap_or_default();
        let amount: i32 = split.next().unwrap_or_default().parse().unwrap_or_default();
        match direction {
            "forward" => {
                horizontal += amount;
                depth += aim * amount;
            }
            "down" => aim += amount,
            "up" => aim -= amount,
            unknown_direction => panic!("unexpected {}", unknown_direction),
        }
    }
    (horizontal * depth).to_string()
}

pub static SOLUTION: AocSolution2 = AocSolution2 { year: 2021, day: 2, solve_p1: p1, solve_p2: p2 };
