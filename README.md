# Advent of Code 2023 in Rust

My [Advent of Code 2023][aoc-2023] solutions in the Rust programming language.
This repository holds a separate Rust project for each day and part.

I attempt to develop a standalone, elegant, compact and fast solution for each
problem (two each day).

## Run solutions

Each Rust project contains a `input.txt` file, holding the puzzle input. Simply
run the project to see the solution appear.

```bash
# Switch to day 1a, and run it
cd day01a
cargo run --release

# or run everything in parallel
cd ../runner
cargo run --release --bin runner-par

# or benchmark every day
cd ../runner
cargo run --release --bin bench
```

## License

This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.

[aoc-2023]: https://adventofcode.com/2023
