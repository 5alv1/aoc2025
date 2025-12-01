use crate::{Output, Solve};
use crate::utils::*;

#[derive(Clone)]
enum Rotations {
	LEFT(i16),
	RIGHT(i16),
}

pub struct Day1 {
	rotations: Vec<Rotations>,
	curr: i16,
	result: u16
}

impl Day1 {
	pub fn new() -> Self {
		Day1 {
			rotations: Vec::new(),
			curr: 50,
			result: 0
		}
	}
}

impl Solve for Day1 {
	fn parse(self: &mut Self, input: String) {
		let lines = input.lines().collect::<Vec<_>>();
		let _: Vec<_> = lines.iter().map(|line| {
			let dir = line.chars().next().unwrap();
			let n = (&line[1..]).to_string().parse::<i16>().unwrap();
			
			match dir {
				'L' => {
					self.rotations.push(Rotations::LEFT(n))
				},
				'R' => {
					self.rotations.push(Rotations::RIGHT(n))
				},
				_ => panic!("Wrong format")
			}
		}).collect();
	}
	
	fn part1<T: Output>(self: &mut Self, out: &mut T) {
		for r in &self.rotations {
			match r {
				Rotations::LEFT(n) => {
					self.curr -= n;
					self.curr = modulo(self.curr, 100);
				}
				Rotations::RIGHT(n) => {
					self.curr += n;
					self.curr = modulo(self.curr, 100);
				}
			};
			if self.curr == 0 {
				self.result += 1;
			}
		};
		out.flush(format!("{}", self.result));
	}
	
	fn part2<T: Output>(self: &mut Self, out: &mut T) {
		for r in &self.rotations {
			// let r = r.clone();
			match r {
				Rotations::LEFT(n) | Rotations::RIGHT(n) if n % 100 == 0 => {
					self.result += (n/100) as u16;
				},
				Rotations::LEFT(n) if self.curr != 0 => {
					let prev = self.curr;
					self.curr -= n;
					if self.curr <= 0 {
						let n = n - prev;
						self.result += (n/100) as u16 + 1;
					}
					
					self.curr = modulo(self.curr, 100);
				},
				Rotations::LEFT(n) => {
					self.curr -= n;
					self.curr = modulo(self.curr, 100); // self.curr % 100;
					self.result += (n/100) as u16;
				},
				Rotations::RIGHT(n) if self.curr != 0 => {
					self.curr += n;
					if self.curr >= 100 {
						let n = self.curr - 100;
						self.result += (n/100) as u16 + 1;
					}
					
					self.curr = modulo(self.curr, 100);
				}
				Rotations::RIGHT(n) => {
					self.curr += n;
					self.curr = modulo(self.curr, 100);
					self.result += (n/100) as u16;
				}
			};
		}
		out.flush(format!("{}", self.result));
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn input_test() {
		let mut o = GetResult::new();
		let input: Vec<Rotations> = vec![
			Rotations::LEFT(68),
			Rotations::LEFT(30),
			Rotations::RIGHT(48),
			Rotations::LEFT(5),
			Rotations::RIGHT(60),
			Rotations::LEFT(55),
			Rotations::LEFT(1),
			Rotations::LEFT(99),
			Rotations::RIGHT(14),
			Rotations::LEFT(82)
		];
		
		let mut d = Day1::new();
		d.rotations = input;
		
		d.part1(&mut o);
		assert_eq!(o.parse::<u16>().unwrap(), 3);
	}
	
	#[test]
	fn input_test_part2() {
		let mut o = GetResult::new();
		let input: Vec<Rotations> = vec![
			Rotations::LEFT(68),
			Rotations::LEFT(30),
			Rotations::RIGHT(48),
			Rotations::LEFT(5),
			Rotations::RIGHT(60),
			Rotations::LEFT(55),
			Rotations::LEFT(1),
			Rotations::LEFT(99),
			Rotations::RIGHT(14),
			Rotations::LEFT(82),
			Rotations::LEFT(500)
		];
		
		let mut d = Day1::new();
		d.rotations = input;
		
		d.part2(&mut o);
		assert_eq!(o.parse::<u16>().unwrap(), 11)
	}
}
