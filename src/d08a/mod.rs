/// Executes day 8 part 1 with the exercise input
/// See https://adventofcode.com/2022/day/8
pub fn main() {
	println!("{}", run(include_bytes!("input.txt")));
}

fn run(file_input: &'static [u8]) -> usize {
	let mut n_visible_trees: usize = 0;
	let n_columns = file_input.iter().position(|c| c == &b'\n').unwrap();
	let tree_sizes: Vec<&[u8]> = file_input
		.split(|c| c == &b'\n')
		.filter(|l| !l.is_empty())
		.collect();
	let n_rows = tree_sizes.len();
	// Calculate trees visible in the interior
	traverse_matrix(&tree_sizes, 0, 0, &n_rows, &n_columns, &mut n_visible_trees);
	// Calculate trees visible
	return n_visible_trees + n_rows * 2 + (n_columns - 2) * 2;
}

fn traverse_matrix(
	tree_sizes: &Vec<&[u8]>,
	i: usize,
	j: usize,
	n_rows: &usize,
	n_columns: &usize,
	n_visible_trees: &mut usize,
) -> bool {
	// The entire column is traversed: force row switch but do not end the recursion returning false
	if j >= *n_columns {
		return false;
	}
	// All rows are traversed: end recursion
	if i >= *n_rows {
		return true;
	}
	// Calculate if tree is visible in the inner ring
	if i > 0 && i < *n_rows - 1 && j > 0 && j < *n_columns - 1 {
		let tree_size = tree_sizes[i][j];
		// Upper path
		if !(0..i).any(|x| tree_size <= tree_sizes[x][j]) {
			*n_visible_trees += 1;
			if traverse_matrix(tree_sizes, i, j + 1, n_rows, n_columns, n_visible_trees) {
				// There are still columns to traverse
				return true;
			}
		}
		// Lower path
		if !(i + 1..*n_rows).any(|x| tree_size <= tree_sizes[x][j]) {
			*n_visible_trees += 1;
			if traverse_matrix(tree_sizes, i, j + 1, n_rows, n_columns, n_visible_trees) {
				// There are still columns to traverse
				return true;
			}
		}
		// Left path
		if !(0..j).any(|x| tree_size <= tree_sizes[i][x]) {
			*n_visible_trees += 1;
			if traverse_matrix(tree_sizes, i, j + 1, n_rows, n_columns, n_visible_trees) {
				// There are still columns to traverse
				return true;
			}
		}
		// Right path
		if !(j + 1..*n_columns).any(|x| tree_size <= tree_sizes[i][x]) {
			*n_visible_trees += 1;
			if traverse_matrix(tree_sizes, i, j + 1, n_rows, n_columns, n_visible_trees) {
				// There are still columns to traverse
				return true;
			}
		}
	}
	// Change column (tree is not visible)
	if traverse_matrix(tree_sizes, i, j + 1, n_rows, n_columns, n_visible_trees) {
		// There are still columns to traverse
		return true;
	}
	// Change row (column out of bonds)
	return traverse_matrix(tree_sizes, i + 1, 0, n_rows, n_columns, n_visible_trees);
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn example() {
		assert_eq!(21, run(include_bytes!("example.txt")));
	}
}
