# Advent of Code in Rust :crab:

My [Advent of Code (AoC)](https://adventofcode.com/) solutions using the Rust language.

## Build

The build process expects a top level directory named `files` with all the input files in the format `dayNN.txt` where `NN` is a number from 01 to 25 (left padded).

## Usage

To get all solutions you can run the default binary

```bash
cargo run --release
```

To check a specific solution run the following command

```bash
cargo run --release -- --day 1 --step a # get solution for day 1 part A
```

## License

This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.
