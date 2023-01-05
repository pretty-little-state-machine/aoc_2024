<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2022

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

> **Note**
> Puzzle inputs are not checked into git. [Reasoning](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3).

<!--- advent_readme_stars table --->
## 2022 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2022/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2022/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2022/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2022/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2022/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2022/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2022/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2022/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2022/day/9) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2022/day/10) | ⭐ | ⭐ |
| [Day 11](https://adventofcode.com/2022/day/11) | ⭐ | ⭐ |
| [Day 12](https://adventofcode.com/2022/day/12) | ⭐ | ⭐ |
| [Day 14](https://adventofcode.com/2022/day/14) | ⭐ | ⭐ |
| [Day 15](https://adventofcode.com/2022/day/15) | ⭐ | ⭐ |
| [Day 16](https://adventofcode.com/2022/day/16) | ⭐ | ⭐ |
| [Day 17](https://adventofcode.com/2022/day/17) | ⭐ | ⭐ |
| [Day 18](https://adventofcode.com/2022/day/18) | ⭐ | ⭐ |
| [Day 19](https://adventofcode.com/2022/day/19) | ⭐ | ⭐ |
| [Day 20](https://adventofcode.com/2022/day/20) | ⭐ | ⭐ |
| [Day 21](https://adventofcode.com/2022/day/21) | ⭐ | ⭐ |
| [Day 22](https://adventofcode.com/2022/day/22) | ⭐ | ⭐ |
| [Day 23](https://adventofcode.com/2022/day/23) | ⭐ | ⭐ |
| [Day 24](https://adventofcode.com/2022/day/24) | ⭐ | ⭐ |
| [Day 25](https://adventofcode.com/2022/day/25) | ⭐ |   |
<!--- advent_readme_stars table --->

---

# Goals & What I've Learned
 
This year I wanted to expand my Rust experiences with data-structures, complicated cases with string parsing, and how 
memory is managed in Rust. Although I've written a Gameboy emulator it was a relatively simple project when it came to 
appeasing the borrow-checker; this left me with a huge hole in my knowledge.

Here are "lessons learned" for each day's problems.

> **SPOILER WARNING**  
> There will be solution spoilers in these notes.

## Day 1

My original solution involved sorting the calorie listing then taking the max value. There's a faster way to obtain 
a max value using the `k_largest` algorithm which uses a binary heap to keep the maximum value at the root of the heap.
Here's a stack overflow discussion of this: 

[What is the advantage of heaps over sorted arrays?](https://cs.stackexchange.com/questions/63931/what-is-the-advantage-of-heaps-over-sorted-arrays])

Rust's Itertools crate will provide this soon, for now I am using a PR branch to obtain this functionality.

## Day 2

Strings are slow when we have guaranteed ASCII char input! The original solution used lots of `&str` references for 
matching the Rock / Paper / Scissor puzzle inputs. I converted these over to `&[u8; 1]` and got a nice boost. You can 
see the code differences in this commit:

(Optimization - &str to chars commits)[https://github.com/itwasscience/aoc_2022/commits/main/src/bin/02.rs]

**Comparison of runtimes**
```
----------
| Day 02 |
----------
🎄 Part 1 🎄
15523 (elapsed: 206.60µs) -> (elapsed: 45.60µs)
🎄 Part 2 🎄
15702 (elapsed: 204.40µs) -> (elapsed: 41.50µs)
```

## Day 3

I used a similar optimization to Day 2 here and treated the inputs as ASCII and used a simple byte -> priority mapper
to just do some integer math to find rucksack item priorities.

## Day 4

My first solution used a `for x in foo { if ... else }` style of code to sum values to a mutable variable within the
loop. I rewrote this later to use the `filter()` and `count()` methods, which resulted in cleaner code.

(Cleanup Commit)[https://github.com/itwasscience/aoc_2022/commit/0ad3344e85c777b2db0712266d1b78d8e9e0da2c]

## Day 5

This day had a real tricky issue in the helper function I wrote to build the `Vec<VecDeque<char>>` up from the puzzle 
input. Observe the following code snippet which is trying to iterate over some input line-by-line and update a Vector 
with some function using that line's contents:

```rust
fn parse_line(input: &str) -> char {
    input.chars().nth(0).unwrap()
}

fn main() {
    let input = "a\nb\nc\nd\n";

    let mut broken = Vec::new();
    input
        .lines()
        .map(|line| {
            broken.push(parse_line(line));
        });
    println!("Broken: {:?}", broken);

    let mut working = Vec::new();
    input
        .lines()
        .for_each(|line| {
            working.push(parse_line(line));
        });
    println!("Working: {:?}", working);
}
```

What's happening here? I'm using CLion with the Rust plugin and the recommendation from the plugin is `Unused Map<Lines,
fn(&str)> that must be used` and recommends I prefix the first block of input parsing with `let _ =`. Doing this will
NOT fix the issue with this code. Why?

In Rust iterators are evaluated in a lazy fashion. This means that `map` will only do work when the iterator is advanced
either by calling `next` explicitly or via something like `collect()`. Just an assignment will not cause the iterator to
be evaluated; instead the type of the _ would have been of the type `Map<Lines<fn(&str)>>`.

What does the compiler say? 
```
   = note: `#[warn(unused_must_use)]` on by default
   = note: iterators are lazy and do nothing unless consumed
```

Unfortunately this message goes away using the plugin's recommendations but won't solve the issue!

There's two obvious ways to fix this issue. The less-elegant is to leave the `let _ =...` and then force an evaluation
with `.collect::<Vec<()>>()` (just a vector of the _unit_ type). Thankfully we can also use the `for_each()` method to
force an evaluation of the Iterator and avoid requiring collection entirely (and the `let _ =` statement).

## Day 6

Day 6 gave me the opportunity to check out some neat windowing functions in the Itertools crate. There really wasn't 
else to this day.

## Day 7

Day 7 posed the first real datastructure work. Although the puzzle was conceptually fairly simple it lent itself to a
_tree_ datastructure. Rust's borrow-checker gave me quite a bit of trouble here as my typical Pythonic solutions quickly
broke due to one particular borrow-checker rule:

> **Borrow Checker Error**  
> error[E0499]: cannot borrow `s` as mutable more than once at a time

My initial tree structure was something like this:

```rust
#[derive(Default)]
struct Directory<'a> {
    file_sizes: usize,
    children: Vec<&'a mut Directory<'a>>
}

impl <'a> Directory<'a> {
    fn sum(self) -> usize {
        let mut sum = self.file_sizes;
        for child in self.children {
            sum += child.sum();
        }
        sum
    }
}

fn main() {
    let mut child_a = Directory{file_sizes: 1, children: Vec::new()};
    let mut child_ab = Directory{file_sizes: 2, children:  Vec::new()};
    let mut child_ac = Directory{file_sizes: 3, children: Vec::new()};
    let mut root = Directory{file_sizes: 10,  children: Vec::new()};

    root.children.push(&mut child_a);
    child_a.children.push(&mut child_ab);
    child_a.children.push(&mut child_ac);
    root.sum();
}
```

But does this work? During the problem it became clear I would need to traverse this structure in a recursive fashion to
take sums of nested directories. This `sum` function quickly explodes:

```rust
error[E0507]: cannot move out of `*child` which is behind a mutable reference
--> src/main.rs:11:20
|
11 |             sum += child.sum();
|                    ^^^^^^-----
|                    |     |
|                    |     `*child` moved due to this method call
|                    move occurs because `*child` has type `Directory<'_>`, which does not implement the `Copy` trait
|
note: this function takes ownership of the receiver `self`, which moves `*child`
--> src/main.rs:8:12
|
8  |     fn sum(self) -> usize {
    |            ^^^^
```

What can we do here? 

The answer is to use something called an _arena_ to map our paths. I choose to use a HashMap since 
I had a known key I could use (the directory path) although `Vec` or other structures may work.

### Theory Behind Arenas

An arena is a structure that holds a reference to a particular node in the tree. Nodes will use a reference to the 
location of other nodes in the arena structure, such as a key or an offset, instead of a direct reference to another 
node.

Here is a tiny excerpt from my day 7 solution demonstrating the arena model:

```rust
use std::collections::HashMap;

/// The filesystem is a tree structure using an arena allocator hashmap with the keys as directory
/// paths. Directories hold the key indexes of child directories.
type DirectoryTree = HashMap<String, Directory>;

#[derive(Default, Debug)]
pub struct Directory {
    file_sizes: usize,
    children: Vec<String>,
}

/// Sums the filesystem recursively for a path.
pub fn sum_path(path: &String, filesystem: &DirectoryTree) -> usize {
    let mut sum: usize = filesystem.get(path).unwrap().file_sizes;
    for child in &filesystem.get(path).unwrap().children {
        sum += sum_path(child, filesystem);
    }
    sum
}

pub fn main() {
    let mut arena = DirectoryTree::default();

    let mut root = Directory{file_sizes: 10,  children: Vec::new()};
    let mut child_a = Directory{file_sizes: 1, children: Vec::new()};
    let child_ab = Directory{file_sizes: 2, children:  Vec::new()};
    let child_ac = Directory{file_sizes: 3, children: Vec::new()};

    // Children are arena keys, NOT references to the children themselves.
    root.children.push("a".to_string());
    child_a.children.push("ab".to_string());
    child_a.children.push("ac".to_string());

    // Build up the Arena with some Strings as the key to the HashMap
    arena.insert("root".to_string(), root);
    arena.insert("a".to_string(), child_a);
    arena.insert("ab".to_string(), child_ab);
    arena.insert("ac".to_string(), child_ac);

    println!("Sum: {}", sum_path(&"root".to_string(), &arena));
}
```

This works great! Notice that the summing function doesn't require any mutable references at all!

> **Optimization?**    
> There might be some room for improvement here if `sum_path()` is being called on very large, very deep trees since the
> recursive calculation has to be done each time `sum_path()` is called. Another `Hashmap` could be used to provide a 
> cache based on the search string for long-running programs. I didn't observe an improvement in Day 07. Always 
> benchmark "optimizations" and remove them if they don't actually deliver benefit for the increased complexity!


---

✨ You can start solving puzzles now! Head to the [Usage section](#usage) to see how to use this template. If you like, you can configure [some optional features](#optional-template-features).

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

Puzzle inputs are not checked into git. [Reasoning](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3).

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
2. Create an `.adventofcode.session` file in your home directory and paste your session cookie[^1] into it. To get this, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in your Cookies under the Application or Storage tab, and copy out the `session` cookie value.

Once installed, you can use the [download command](#download-input-for-a-day).
