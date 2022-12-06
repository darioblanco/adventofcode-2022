/// Executes day 2 part 2 with the exercise input
/// See https://adventofcode.com/2022/day/2
pub fn main() {
	println!("{}", run(include_str!("input.txt")));
}

fn run(file_input: &str) -> u32 {
	return file_input
		.lines()
		.map(|l| {
			let args: Vec<&str> = l.trim().split_whitespace().collect();
			return calculate_score(args[0], args[1]);
		})
		.sum();
}

fn calculate_score(opponent_play: &str, round_outcome: &str) -> u32 {
	match opponent_play {
		// Rock
		"A" => {
			match round_outcome {
				// Lose: choose scissors
				"X" => return 3 + 0,
				// Draw: choose rock
				"Y" => return 1 + 3,
				// Win: choose paper
				"Z" => return 2 + 6,
				// Invalid play
				_ => return 0,
			};
		}
		// Paper
		"B" => {
			match round_outcome {
				// Lose: choose rock
				"X" => return 1 + 0,
				// Draw: choose paper
				"Y" => return 2 + 3,
				// Win: choose scissors
				"Z" => return 3 + 6,
				// Invalid play
				_ => return 0,
			};
		}
		// Scissors
		"C" => {
			match round_outcome {
				// Lose: choose paper
				"X" => return 2 + 0,
				// Draw: choose scissors
				"Y" => return 3 + 3,
				// Win: choose rock
				"Z" => return 1 + 6,
				// Invalid play
				_ => return 0,
			};
		}
		// Invalid play
		_ => return 0,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(12, run(include_str!("example.txt")));
	}
}
