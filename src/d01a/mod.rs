/// Executes day 1 part 1 with the exercise input
/// See https://adventofcode.com/2022/day/1
pub fn main() {
	println!("{}", run(include_str!("input.txt")));
}

fn run(file_input: &str) -> u32 {
	let mut current_cal: u32 = 0;
	let mut max_cal: u32 = 0;
	for line in file_input.lines() {
		let _: u32 = match line.trim().parse() {
			Ok(num) => {
				current_cal = current_cal + num;
				num
			}
			Err(_) => {
				if max_cal < current_cal {
					max_cal = current_cal;
				}
				current_cal = 0;
				continue;
			}
		};
	}
	return max_cal;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(24000, run(include_str!("example.txt")));
	}
}
