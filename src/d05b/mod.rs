use std::collections::VecDeque;

/// Executes day 5 part 1 with the exercise input
/// See https://adventofcode.com/2022/day/5
pub fn main() {
	println!("{}", run(include_str!("input.txt")));
}

fn run(file_input: &str) -> String {
	// Split input in initial crate stacks and movements
	let (
		crate_stacks, rearrangement_procedure
	) = file_input.split_once("\n\n").unwrap();

	// Initialize stack data structures based on last line of the crate plan
	let mut stacks = Vec::<VecDeque<char>>::new();
	let mut aux_stack = VecDeque::<char>::new();
	crate_stacks
		.lines()
		.rev()
		.next()
		.unwrap()
		.split_whitespace()
		.for_each(|_| stacks.push(VecDeque::new()));

	// Fill stack data structures with the input elements
	crate_stacks
		.lines()
		.rev()
		.skip(1)
		.for_each(|line| {
			line.chars()
				.skip(1)
				.step_by(4)
				.enumerate()
				.filter(|(_, c)| c != &' ')
				.for_each(|(i, c)| stacks[i].push_front(c))
		});

	// Perform rearrangement procedures
	rearrangement_procedure
		.lines()
		.for_each(|line| {
			let commands: Vec<&str> = line
				.split_whitespace()
				.collect();
			let (quantity, origin, target) = (
				commands[1].parse::<usize>().unwrap(),
				commands[3].parse::<usize>().unwrap() - 1,
				commands[5].parse::<usize>().unwrap() -1
			);
			(0..quantity).for_each(|_| {
				let crate_box = stacks[origin].pop_front().unwrap();
				aux_stack.push_front(crate_box);
			});
			aux_stack
				.iter()
				.for_each(|crate_box| stacks[target].push_front(*crate_box));
			aux_stack.clear();
		});

	return stacks
		.iter()
        .map(|stack| format!("{}", stack.get(0).unwrap()))
		.collect::<String>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!("MCD", run(include_str!("example.txt")));
    }
}
