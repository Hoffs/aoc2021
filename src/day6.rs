use std::collections::HashMap;

use crate::aoc;

fn p1(input: &str) -> String {
    let mut fishes: Vec<i64> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();

    for _day in 0..80 {
        let mut new_fishes = 0;
        for fish in &mut fishes {
            if fish == &0 {
                new_fishes += 1;
                *fish = 6;
            } else {
                *fish -= 1;
            }
        }

        for _ in 0..new_fishes {
            fishes.push(8);
        }
    }

    fishes.len().to_string()
}

fn p2(input: &str) -> String {
    let initial_fishes: Vec<i32> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let mut fishes: HashMap<i32, i64> = HashMap::new();

    for fish in &initial_fishes {
        if let Some(value) = fishes.get_mut(&fish) {
            *value += 1;
        } else {
            fishes.insert(*fish, 1);
        }
    }

    for _day in 0..256 {
        // could probably just use single map, cant be bothered to think it through
        let mut new_map: HashMap<i32, i64> = HashMap::new();
        for (day, count) in &fishes {
            if day == &0 {
                *new_map.entry(6).or_insert(0) += *count;
                *new_map.entry(8).or_insert(0) += *count;
            } else {
                *new_map.entry(day-1).or_insert(0) += *count;
            }
        }

        fishes = new_map;
    }

    let mut sum = 0;
    for (_, count) in &fishes {
        sum += count;
    }

    sum.to_string()
}

pub static SOLUTION: aoc::AocSolution2 = aoc::AocSolution2 {
    year: 2021,
    day: 6,
    solve_p1: p1,
    solve_p2: p2,
};
