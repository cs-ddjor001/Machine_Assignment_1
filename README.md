# Program Description #
This program converts a real number between 0 and 1 in Base 10 to its equivalent representation in Base 2.

# How to run the program #
The program is written in Rust and can be compiled and run using Cargo.

- Use cargo build to compile the program.
- Use cargo run to run the program.
- Use cargo test to run the unit tests.
- Use cargo run -- followed by a list of real numbers between 0 and 1, separated by a space (e.g., cargo run -- 0.5 0.25 0.75), to get an output similar to the one below.

Example output:

| Base 10 | Base 2 |
| :-------|:-------|
| 0.5     | 0.1    |
| 0.25    | 0.01   |
| 0.75    | 0.11   |

# Dependencies #
The program uses the Hamcrest library for unit testing. Ensure it is added under [dev-dependencies] in the Cargo.toml file.

