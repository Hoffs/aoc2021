extern crate dotenv;

use reqwest::Error;
use std::env;

type AocSolutionFn = fn(&str) -> String;

pub struct AocSolution2 {
    pub year: u32,
    pub day: u8,

    pub solve_p1: AocSolutionFn,
    pub solve_p2: AocSolutionFn,
}

pub fn get_input(year: &str, day: &str) -> Result<String, Error> {
    dotenv::dotenv().ok();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::blocking::Client::new();
    let request = client.get(&url).header(
        "cookie",
        format!(
            "session={}",
            env::var("session").unwrap_or(String::from(""))
        ),
    );
    request.send()?.text()
}
