use std::{cmp, collections::HashMap};

use crate::aoc::AocSolution2;

#[derive(Debug)]
struct Line {
    from_x: i32,
    from_y: i32,
    to_x: i32,
    to_y: i32,
}

impl Line {
    fn from_str(str: &str) -> Line {
        let points: Vec<&str> = str.split(" -> ").collect();
        let from: Vec<i32> = points[0].split(',').map(|x| x.parse().unwrap()).collect();
        let to: Vec<i32> = points[1].split(',').map(|x| x.parse().unwrap()).collect();

        Line{from_x: from[0], from_y: from[1], to_x: to[0], to_y: to[1]}
    }
}

fn p1(input: &str) -> String {
    let lines: Vec<Line> = input.lines().map(|x| Line::from_str(x)).collect();
    let mut map: HashMap<String, i32> = HashMap::new();

    fn add_to_map(map: &mut HashMap<String, i32>, line: &Line) {
        let lower_x = cmp::min(line.from_x, line.to_x);
        let upper_x = cmp::max(line.from_x, line.to_x);
        let lower_y = cmp::min(line.from_y, line.to_y);
        let upper_y = cmp::max(line.from_y, line.to_y);

        if lower_x != upper_x && lower_y != upper_y {
            return
        }

        for x in lower_x..upper_x+1 {
            for y in lower_y..upper_y+1 {
                let key = format!("{}_{}", x, y);
                if let Some(value) = map.get_mut(&key) {
                    *value += 1;
                } else {
                    map.insert(key, 1);
                }
            }
        }
    }

    for line in &lines {
        add_to_map(&mut map, line);
    }

    let mut count = 0;
    for entry in &map {
        if entry.1 > &1 {
            count += 1;
        }
    }

    count.to_string()
}

fn p2(input: &str) -> String {
    let lines: Vec<Line> = input.lines().map(|x| Line::from_str(x)).collect();
    let mut map: HashMap<String, i32> = HashMap::new();

    fn add_to_map(map: &mut HashMap<String, i32>, line: &Line) {
        let lower_x = cmp::min(line.from_x, line.to_x);
        let upper_x = cmp::max(line.from_x, line.to_x);
        let lower_y = cmp::min(line.from_y, line.to_y);
        let upper_y = cmp::max(line.from_y, line.to_y);

        if lower_x != upper_x && lower_y != upper_y {
            let mut x = line.from_x;
            let mut y = line.from_y;
            let x_change = if line.to_x > line.from_x { 1 } else { -1 };
            let y_change = if line.to_y > line.from_y { 1 } else { -1 };
            loop {
                let key = format!("{}_{}", x, y);
                if let Some(value) = map.get_mut(&key) {
                    *value += 1;
                } else {
                    map.insert(key, 1);
                }

                if x == line.to_x && y == line.to_y {
                    break
                }

                x += x_change;
                y += y_change;
            }

        } else {
            for x in lower_x..upper_x+1 {
                for y in lower_y..upper_y+1 {
                    let key = format!("{}_{}", x, y);
                    if let Some(value) = map.get_mut(&key) {
                        *value += 1;
                    } else {
                        map.insert(key, 1);
                    }
                }
            }
        }
    }

    for line in &lines {
        add_to_map(&mut map, line);
    }

    let mut count = 0;
    for entry in &map {
        if entry.1 > &1 {
            count += 1;
        }
    }

    count.to_string()
}

pub static SOLUTION: AocSolution2 = AocSolution2 {
    year: 2021,
    day: 5,
    solve_p1: p1,
    solve_p2: p2,
};

