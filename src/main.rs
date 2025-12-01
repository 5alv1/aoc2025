use std::{env, fs};
use std::str::FromStr;
use crate::utils::*;
use crate::day1::Day1;

mod day1;
mod utils;

pub trait Output {
    fn flush(self: &mut Self, output: String);
    fn parse<T: FromStr>(self: &Self) -> Option<T>;
}

pub struct StdoutOutput {}

impl Output for StdoutOutput {
    fn flush(self: &mut Self, output: String) {
        println!("{}", output)
    }
    
    fn parse<T: FromStr>(self: &Self) -> Option<T> {
        None
    }
}

fn main() {
    let input_path = env::var("INPUT_PATH").unwrap();
    let content = fs::read_to_string(input_path).unwrap();
    
    let mut day1 = Day1::new();
    day1.parse(content);
    
    day1.part2(&mut StdoutOutput {});
}
