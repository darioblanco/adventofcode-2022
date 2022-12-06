use std::cmp::Reverse;

/// Executes day 1 part 2 with the exercise input
/// See https://adventofcode.com/2022/day/1
pub fn main() {
	println!("{}", run(include_str!("input.txt")));
}

fn run(file_input: &str) -> u32 {
	let mut calories = file_input
		.split("\n\n")
		.map(|c| c.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
		.collect::<Vec<u32>>();
	calories.sort_unstable_by_key(|k| Reverse(*k));
	return calories[0] + calories[1] + calories[2];
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(45000, run(include_str!("example.txt")));
	}
}
