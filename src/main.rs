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
		_ => println!("Unable to find exercise {}", exercise_id),
	}
}
