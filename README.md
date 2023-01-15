<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2018

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

> **Note**
> Puzzle inputs are not checked into
> git. [Reasoning](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3)
> .

<!--- advent_readme_stars table --->

## 2018 Results

I had a secondary goal of clearing all puzzles in under 1 second. The profile doesn't include loading the file from 
storage but does include parsing for fairness.

**Total Runtime: TBD**

|  Day   |  Part 1  |  Part 2  |
|:------:|:--------:|:--------:|
| Day 01 | 100.10µs | 29.66ms  |
| Day 02 | 587.50µs | 174.90µs |
| Day 02 | 325.10ms | 266.11ms |
| Day 04 | 385.30µs | 321.70µs |
| Day 05 | 16.52ms  | 90.58ms  |
 


---

✨ You can start solving puzzles now! Head to the [Usage section](#usage) to see how to use this template. If you like,
you can configure [some optional features](#optional-template-features).

## Usage

You must have the [Rust toolchain](https://www.rust-lang.org/tools/install) installed.

### Download input for a day

> **Note**  
> This command requires [installing the aoc-cli crate](#download-puzzle-inputs-via-aoc-cli).

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# Downloading input with aoc-cli...
# Loaded session cookie from "/home/felix/.adventofcode.session".
# Downloading input for day 1, 2021...
# Saving puzzle input to "/tmp/tmp.MBdcAdL9Iw/input"...
# Done!
# ---
# 🎄 Successfully wrote input to "src/inputs/01.txt"!
```

To download inputs for previous years, append the `--year/-y` flag. _(example: `cargo download 1 --year 2020`)_

Puzzle inputs are not checked into
git. [Reasoning](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3)
.

### Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Running `target/debug/01`
# 🎄 Part 1 🎄
#
# 6 (elapsed: 37.03µs)
#
# 🎄 Part 2 🎄
#
# 9 (elapsed: 33.18µs)
```

`solve` is an alias for `cargo run --bin`. To run an optimized version for benchmarking, append the `--release` flag.

Displayed _timings_ show the raw execution time of your solution without overhead (e.g. file reads).

### Run all solutions

```sh
cargo all

# output:
#     Running `target/release/aoc`
# ----------
# | Day 01 |
# ----------
# 🎄 Part 1 🎄
#
# 0 (elapsed: 170.00µs)
#
# 🎄 Part 2 🎄
#
# 0 (elapsed: 30.00µs)
# <...other days...>
# Total: 0.20ms
```

`all` is an alias for `cargo run`. To run an optimized version for benchmarking, use the `--release` flag.

_Total timing_ is computed from individual solution _timings_ and excludes as much overhead as possible.

### Run all solutions against example input

```sh
cargo test
```

### Format code

```sh
cargo fmt
```

### Lint code

```sh
cargo clippy
```

## Optional template features

### Download puzzle inputs via aoc-cli

1. Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) via cargo: `cargo install aoc-cli`.
2. Create an `.adventofcode.session` file in your home directory and paste your session cookie[^1] into it. To get this,
   press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the
   Application or Storage tab, and copy out the `session` cookie value.

Once installed, you can use the [download command](#download-input-for-a-day).
