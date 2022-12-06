/// Executes day 6 part 1 with the exercise input
/// See https://adventofcode.com/2022/day/5
pub fn main() {
	println!("{}", run(include_bytes!("input.txt")));
}

fn run(file_input: &'static [u8]) -> usize {
	let distinct_chars = 14;
	return file_input
		.windows(distinct_chars) // Get elements in groups
		.position(|c|
			// Analyze if elements from this window repeat in the next marker positions
			!(0..distinct_chars-1).any(|i|
				(i+1..distinct_chars).any(|j| c[j] == c[i])
			)) // Return position of the beginning of the window
		.unwrap()
		+ distinct_chars; // Return the end of the window as a solution
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example1() {
		assert_eq!(19, run(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
	}

	#[test]
	fn example2() {
		assert_eq!(23, run(b"bvwbjplbgvbhsrlpgdmjqwftvncz"));
	}

	#[test]
	fn example3() {
		assert_eq!(23, run(b"nppdvjthqldpwncqszvftbrmjlhg"));
	}

	#[test]
	fn example4() {
		assert_eq!(29, run(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
	}

	#[test]
	fn example5() {
		assert_eq!(26, run(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
	}
}
