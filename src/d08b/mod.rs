/// Executes day 8 part 2 with the exercise input
/// See https://adventofcode.com/2022/day/8
pub fn main() {
	println!("{}", run(include_bytes!("input.txt")));
}

fn run(file_input: &'static [u8]) -> usize {
	let mut max_scenic_score: usize = 0;
	let n_columns = file_input.iter().position(|c| c == &b'\n').unwrap();
	let tree_sizes: Vec<&[u8]> = file_input
		.split(|c| c == &b'\n')
		.filter(|l| !l.is_empty())
		.collect();
	let n_rows = tree_sizes.len();

	for i in 0..n_rows {
		for j in 0..n_columns {
			let tree_size = tree_sizes[i][j];

			let upper_pos = (0..i).rev().position(|x| tree_size <= tree_sizes[x][j]);
			let upper_score = match upper_pos {
				None => i,
				Some(x) => x + 1,
			};

			let lower_pos = (i + 1..n_rows).position(|x| tree_size <= tree_sizes[x][j]);
			let lower_score = match lower_pos {
				None => n_rows - 1 - i,
				Some(x) => x + 1,
			};

			let left_pos = (0..j).rev().position(|x| tree_size <= tree_sizes[i][x]);
			let left_score = match left_pos {
				None => j,
				Some(x) => x + 1,
			};

			let right_pos = (j + 1..n_columns).position(|x| tree_size <= tree_sizes[i][x]);
			let right_score = match right_pos {
				None => n_columns - 1 - j,
				Some(x) => x + 1,
			};

			let scenic_score = upper_score * lower_score * left_score * right_score;
			if scenic_score > max_scenic_score {
				max_scenic_score = scenic_score;
			}
		}
	}

	return max_scenic_score;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(8, run(include_bytes!("example.txt")));
	}
}
