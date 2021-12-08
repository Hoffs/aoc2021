use std::collections::HashMap;

use aoc::AocSolution2;
use std::env;

mod aoc;
mod day0;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn add_solution<'x>(map: &mut HashMap<String, &'x AocSolution2>, solution: &'x AocSolution2) {
    let key = format!("{}-{}", solution.year, solution.day);
    map.insert(key, solution);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let day = env::args().nth(1).unwrap_or("1".to_string());
    let key = format!("{}-{}", "2021", &day);
    let mut solutions: HashMap<String, &AocSolution2> = HashMap::new();
    add_solution(&mut solutions, &day0::SOLUTION);
    add_solution(&mut solutions, &day2::SOLUTION);
    add_solution(&mut solutions, &day3::SOLUTION);
    add_solution(&mut solutions, &day4::SOLUTION);
    add_solution(&mut solutions, &day5::SOLUTION);
    add_solution(&mut solutions, &day6::SOLUTION);
    add_solution(&mut solutions, &day7::SOLUTION);

    let input = aoc::get_input("2021", &day)?;
    if let Some(solution) = solutions.get(&key) {
        let s1 = (solution.solve_p1)(&input);
        println!("part1 = {}", s1);
        let s2 = (solution.solve_p2)(&input);
        println!("part2 = {}", s2);
    }
    Ok(())
}
