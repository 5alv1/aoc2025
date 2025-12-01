use std::{env, fs};
use crate::day1::Day1;

mod day1;
mod utils;

pub trait Output {
    fn flush(self: &mut Self, output: String);
}

pub struct StdoutOutput {}

impl Output for StdoutOutput {
    fn flush(self: &mut Self, output: String) {
        println!("{}", output)
    }
}

pub trait Solve {
    fn parse(self: &mut Self, input: String);
    fn part1<T: Output>(self: &mut Self, out: &mut T);
    fn part2<T: Output>(self: &mut Self, out: &mut T);
}

fn main() {
    let input_path = env::var("INPUT_PATH").unwrap();
    let content = fs::read_to_string(input_path).unwrap();
    
    let mut day1 = Day1::new();
    day1.parse(content);
    
    day1.part2(&mut StdoutOutput {});
}
