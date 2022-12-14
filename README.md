# Advent of Code 2022

My [Advent of Code 2022](https://adventofcode.com/2022/) solutions in Rust.
This repository holds a single Rust project that will run the exercises based
on cmd input.

Each exercise folder has the format `d[0-9][0-9](a|b)` and contains an `input.txt` file,
holding the puzzle input.

Simply run the project and select an exercise.

```sh
$ cargo run
Advent of Code 2022 - @darioblanco
Please input the exercise id (e.g d01a).
d01a
Your choice: d01a

Solution
```

Each exercise has at least a test, taking the example input and asserting
the expected result given by the website.

To run the tests:

```sh
cargo test
```

## License

This project is released under the GNU GPL-3.0 license.
Check out [LICENSE](./LICENSE) file for more information.
