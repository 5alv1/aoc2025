use std::str::FromStr;
use crate::Output;

pub fn modulo(a: i16, m: i16) -> i16 {
	((a % m) + m) % m
}

pub struct GetResult {
	result: String
}

impl GetResult {
	pub fn new() -> Self {
		GetResult {
			result: String::new()
		}
	}
}

pub trait Solve {
	fn parse(self: &mut Self, input: String);
	fn part1<T: Output>(self: &mut Self, out: &mut T);
	fn part2<T: Output>(self: &mut Self, out: &mut T);
}

impl Output for GetResult {
	fn flush(self: &mut Self, output: String) {
		self.result = output
	}
	
	fn parse<T: FromStr>(self: &Self) -> Option<T> {
		match self.result.parse::<T>() {
			Err(_) => None,
			Ok(s) => Some(s)
		}
	}
}
