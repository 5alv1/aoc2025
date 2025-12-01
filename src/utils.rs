use crate::Output;

pub fn modulo(a: i16, m: i16) -> i16 {
	((a % m) + m) % m
}

pub struct GetResult {
	pub result: String
}

impl GetResult {
	pub fn new() -> Self {
		GetResult {
			result: String::new()
		}
	}
}

impl Output for GetResult {
	fn flush(self: &mut Self, output: String) {
		self.result = output
	}
}
