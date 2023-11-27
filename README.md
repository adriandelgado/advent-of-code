# Advent of Code in Rust :crab:

My [Advent of Code (AoC)](https://adventofcode.com/) solutions using the Rust language.

## Usage

To check a specific solution run the following command

```bash
cargo run --release -- 2015 1 2 # get solution for year 2015 day 1 part 2
```

The application expects an `AOC_SESSION` env variable which has an Advent of Code session cookie
and a `USER_AGENT` env variable with your github handle. (see: <https://www.reddit.com/r/adventofcode/comments/z9dhtd/please_include_your_contact_info_in_the_useragent/>)

The env variables can be also provided via an `.env` file.
