use std::iter::Peekable;

/// Executes day 7 part 1 with the exercise input
/// See https://adventofcode.com/2022/day/7
pub fn main() {
	println!("{}", run(include_str!("input.txt")));
}

fn run(file_input: &str) -> u64 {
	let mut size_sum: u64 = 0;
	sum_folder(&mut file_input.lines().peekable(), &mut size_sum);
	return size_sum;
}

fn sum_folder(lines: &mut Peekable<core::str::Lines>, size_sum: &mut u64) -> u64 {
	let mut folder_size: u64 = 0;
	while let Some(line) = lines.next() {
		match line {
			"$ cd .." => break, // Move to previous folder
			"$ ls" => {
				let mut file_line = lines.next_if(|line| !line.contains("$ "));
				while file_line != None {
					let first = file_line.unwrap().split_whitespace().next().unwrap();
					match first.parse::<u64>() {
						Ok(file_size) => folder_size += file_size,
						Err(_) => (), // directory
					}
					file_line = lines.next_if(|line| !line.contains("$ "));
				}
			} // List directory (and calculate size of the output)
			_ => folder_size += sum_folder(lines, size_sum), // Move to another inner directory
		}
	}
	if folder_size <= 100000 {
		*size_sum += folder_size;
	}
	return folder_size;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(95437, run(include_str!("example.txt")));
	}
}
