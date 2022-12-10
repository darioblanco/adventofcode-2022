use std::io;

mod d01a;
mod d01b;
mod d02a;
mod d02b;
mod d03a;
mod d03b;
mod d04a;
mod d04b;
mod d05a;
mod d05b;
mod d06a;
mod d06b;
mod d07a;
// mod d07b;
// mod d08a;
// mod d08b;
// mod d09a;
// mod d09b;
// mod d10a;
// mod d10b;
// mod d11a;
// mod d11b;
// mod d12a;
// mod d12b;
// mod d13a;
// mod d13b;
// mod d14a;
// mod d14b;
// mod d15a;
// mod d15b;
// mod d16a;
// mod d16b;
// mod d17a;
// mod d17b;
// mod d18a;
// mod d18b;
// mod d19a;
// mod d19b;
// mod d20a;
// mod d20b;
// mod d21a;
// mod d21b;
// mod d22a;
// mod d22b;
// mod d23a;
// mod d23b;
// mod d24a;
// mod d24b;
// mod d25a;
// mod d25b;

fn main() {
	println!("Advent of Code 2022 - @darioblanco");

	println!("Please input the exercise id (e.g d01a).");

	let mut exercise_id = String::new();

	io::stdin()
		.read_line(&mut exercise_id)
		.expect("Failed to read exercise id");

	println!("Your choice: {exercise_id}");

	match exercise_id.trim() {
		"d01a" => d01a::main(),
		"d01b" => d01b::main(),
		"d02a" => d02a::main(),
		"d02b" => d02b::main(),
		"d03a" => d03a::main(),
		"d03b" => d03b::main(),
		"d04a" => d04a::main(),
		"d04b" => d04b::main(),
		"d05a" => d05a::main(),
		"d05b" => d05b::main(),
		"d06a" => d06a::main(),
		"d06b" => d06b::main(),
		"d07a" => d07a::main(),
		// "d07b" => d07b::main(),
		// "d08a" => d08a::main(),
		// "d08b" => d08b::main(),
		// "d09a" => d09a::main(),
		// "d09b" => d09b::main(),
		// "d10a" => d10a::main(),
		// "d10b" => d10b::main(),
		// "d11a" => d11a::main(),
		// "d11b" => d11b::main(),
		// "d12a" => d12a::main(),
		// "d12b" => d12b::main(),
		// "d13a" => d13a::main(),
		// "d13b" => d13b::main(),
		// "d14a" => d14a::main(),
		// "d14b" => d14b::main(),
		// "d15a" => d15a::main(),
		// "d15b" => d15b::main(),
		// "d16a" => d16a::main(),
		// "d16b" => d16b::main(),
		// "d17a" => d17a::main(),
		// "d17b" => d17b::main(),
		// "d18a" => d18a::main(),
		// "d18b" => d18b::main(),
		// "d19a" => d19a::main(),
		// "d19b" => d19b::main(),
		// "d20a" => d20a::main(),
		// "d20b" => d20b::main(),
		// "d21a" => d21a::main(),
		// "d21b" => d21b::main(),
		// "d22a" => d22a::main(),
		// "d22b" => d22b::main(),
		// "d23a" => d23a::main(),
		// "d23b" => d23b::main(),
		// "d24a" => d24a::main(),
		// "d24b" => d24b::main(),
		// "d25a" => d25a::main(),
		// "d25b" => d25b::main(),
		_ => println!("Unable to find exercise {}", exercise_id),
	}
}
