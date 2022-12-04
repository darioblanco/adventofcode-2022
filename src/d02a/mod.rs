/// Executes day 2 part 1 with the exercise input
/// See https://adventofcode.com/2022/day/2
pub fn main() {
	println!("{}", run(include_str!("input.txt")));
}

fn run(file_input: &str) -> u32 {
	return file_input.lines()
		.map(|l| {
			let args: Vec<&str> = l.trim().split_whitespace().collect();
			return calculate_score(args[0], args[1]);
		})
		.sum();
}

fn calculate_score(opponent_play: &str, own_play: &str) -> u32 {
	let shape_score: u32 = match own_play {
		 // Rock
		"X" => 1,
		// Paper
		"Y" => 2,
		// Scissors
		"Z" => 3,
		// Invalid play
		_ => 0,
	};
	let mut outcome_score = 3; // Draw by default
	match opponent_play {
		// Rock
		"A" => {
			if own_play == "Y" {
				// Rock vs Paper
				outcome_score = 6
			} else if own_play == "Z" {
				// Rock vs Scissors
				outcome_score = 0
			}
		},
		// Paper
		"B" => {
			if own_play == "Z" {
				// Paper vs Scissors
				outcome_score = 6
			} else if own_play == "X" {
				// Paper vs Rock
				outcome_score = 0
			}
		},
		// Scissors
		"C" => {
			if own_play == "X" {
				// Scissors vs Rock
				outcome_score = 6
			} else if own_play == "Y" {
				// Scissors vs Paper
				outcome_score = 0
			}
		},
		// Invalid play
		_ => (),
	}
	return shape_score + outcome_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(15, run(include_str!("example.txt")));
    }
}
