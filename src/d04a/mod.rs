/// Executes day 3 part 2 with the exercise input
/// See https://adventofcode.com/2022/day/3
pub fn main() {
	println!("{}", run(include_str!("input.txt")));
}

fn run(file_input: &str) -> usize {
	return file_input
		// Split each pair
		.lines()
		// Skip empty lines (e.g. last line in unix is always empty)
		.filter(|l| !l.is_empty())
		// Parse minimum and maximum ranges per group
		.map(|line| {
			let (range1, range2) = line.trim().split_once(",").unwrap();
			let ((min1,  max1), ( min2,  max2)) = (
				range1.split_once('-').unwrap(),
				range2.split_once('-').unwrap(),
			);
			return (
				(min1.parse::<u8>().unwrap(), max1.parse::<u8>().unwrap()),
				(min2.parse::<u8>().unwrap(), max2.parse::<u8>().unwrap())
			)
		})
		// Count only the groups that overlap
		.filter(|((min1, max1), (min2, max2))|
			(min1 <= min2 && max1 >= max2) || (min1 >= min2 && max1 <= max2)
		)
		.count();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(2, run(include_str!("example.txt")));
    }
}
