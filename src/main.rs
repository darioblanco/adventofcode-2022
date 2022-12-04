use std::io;

mod d01a;
mod d01b;

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
		_ => println!("Unable to find exercise {}", exercise_id),
	}
}