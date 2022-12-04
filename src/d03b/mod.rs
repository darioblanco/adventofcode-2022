/// Executes day 3 part 2 with the exercise input
/// See https://adventofcode.com/2022/day/3
pub fn main() {
	// Load in bytes so priorities can be transformed easily
	println!("{}", run(include_bytes!("input.txt")));
}

fn run(file_input: &'static [u8]) -> u16 {
	return file_input
		// Split by rucksack (every newline)
		.split(|b| *b == b'\n')
		// Skip empty lines (e.g. last line in unix is always empty)
		.filter(|l| !l.is_empty())
		// Create groups of 3 rucksacks
		.collect::<Vec<&[u8]>>()
		.chunks(3)
		// Find the repeated element per rucksack group
		.map(|rucksack_group| rucksack_group[0]
			.iter()
			.find(|x|
				rucksack_group[1].contains(x) &&
				rucksack_group[2].contains(x)
			)
			.unwrap()
		)
		// Calculate priority for the repeated element
		// This is based in ASCII characters A=65, Z=90, a=97, z=122
		.map(|x|
			// Values need to be converted to u16 to avoid overflow in the sum
			// With u8 the maximum is 255, with u16 is 65535
			if *x >= b'a' {
				// Lowercase element (priority from 1 to 26)
				(x - b'a') as u16 + 1
			} else {
				// Uppercase element (priority from 27 to 52)
				(x - b'A') as u16 + 27
			}
		)
		// Sum all priorities from the repeated element in all rucksack groups
		.sum::<u16>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(70, run(include_bytes!("example.txt")));
    }
}
