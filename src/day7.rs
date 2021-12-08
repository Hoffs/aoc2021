use std::cmp;

use crate::aoc;

fn median(numbers: &mut Vec<i64>) -> i64 {
    numbers.sort();

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid-1] + numbers[mid]) / 2
    } else {
        numbers[mid]
    }
}

fn p1(input: &str) -> String {
    let mut nums: Vec<i64> = input.split(',').map(|x| x.trim().parse().unwrap()).collect();

    let median = median(&mut nums);

    let fuel = nums.iter().fold(0, |acc, n| acc + (n - median).abs());

    fuel.to_string()
}

fn p2(input: &str) -> String {
    let nums: Vec<i64> = input.split(',').map(|x| x.trim().parse().unwrap()).collect();

    fn fuel_for_diff(from: &i64, to: &i64) -> i64 {
        let mut sum = 0;
        for (bonus, _) in (cmp::min(*from, *to)..cmp::max(*from, *to)).enumerate() {
            sum += 1 + bonus;
        }

        sum as i64
    }
    let mean = nums.iter().sum::<i64>() as f64 / nums.len() as f64;

    // why floor? who knows, round() didnt work, try floor() :)
    let fuel = nums.iter().fold(0, |acc, n| acc + fuel_for_diff(&(mean.floor() as i64), n));

    fuel.to_string()
}

pub static SOLUTION: aoc::AocSolution2 = aoc::AocSolution2 {
    year: 2021,
    day: 7,
    solve_p1: p1,
    solve_p2: p2,
};
