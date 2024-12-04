extern crate core;

use crate::DayResult;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let puzzle = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&puzzle).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&puzzle).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

// Just for sanity's sake and debugging ease we redefine the ASCII
const X: u32 = 1;
const M: u32 = 2;
const A: u32 = 3;
const S: u32 = 4;
const TOTAL_LETTERS: usize = 140 * 140; // From large puzzle input

#[derive(Debug)]
struct Puzzle {
    letters: [u32; TOTAL_LETTERS],
    width: usize,
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            letters: [0; TOTAL_LETTERS],
            width: 0,
        }
    }
}

fn parse(input: &str) -> Puzzle {
    let mut puzzle = Puzzle::default();
    let mut width_check: usize = 0;
    let mut offset: usize = 0;
    for c in input.chars() {
        match c {
            '\n' => {
                puzzle.width = width_check;
                width_check = 0;
                continue;
            }
            'X' => puzzle.letters[offset] = X,
            'M' => puzzle.letters[offset] = M,
            'A' => puzzle.letters[offset] = A,
            'S' => puzzle.letters[offset] = S,
            _ => panic!("Unknown character: {c}"),
        }
        width_check += 1;
        offset += 1;
    }
    puzzle
}

#[inline(always)]
fn is_match(a: u32, b: u32, c: u32, d: u32) -> bool {
    ((a == X) & (b == M) & (c == A) & (d == S)) | ((a == S) & (b == A) & (c == M) & (d == X))
}

fn part_1(p: &Puzzle) -> usize {
    let mut found = 0;
    for i in 0..TOTAL_LETTERS {
        // Ensure we don't read across lines using modulo and horizontal scan backwards
        if i % p.width > 2
            && is_match(
                p.letters[i],
                p.letters[i - 1],
                p.letters[i - 2],
                p.letters[i - 3],
            )
        {
            found += 1;
        }
        // Multi-line matches
        if i >= p.width * 3 {
            // Vertical match - Checking from bottom up
            if is_match(
                p.letters[i],
                p.letters[i - p.width],
                p.letters[i - p.width * 2],
                p.letters[i - p.width * 3],
            ) {
                found += 1;
            }
            // Diagonal match - Bottom left to top right - ensure we don't "wrap" around lines
            if i % p.width < p.width - 3
                && is_match(
                    p.letters[i],
                    p.letters[i - p.width + 1],
                    p.letters[i - p.width * 2 + 2],
                    p.letters[i - p.width * 3 + 3],
                )
            {
                found += 1;
            }
            // Diagonal match - Bottom right to top left - ensure we don't "wrap" around lines
            if i % p.width > 2
                && is_match(
                    p.letters[i],
                    p.letters[i - p.width - 1],
                    p.letters[i - p.width * 2 - 2],
                    p.letters[i - p.width * 3 - 3],
                )
            {
                found += 1;
            }
        }
    }
    found
}

/*
Expected input pattern:
D E
 C
B A
*/
#[inline(always)]
fn is_cross_match(a: u32, b: u32, c: u32, d: u32, e: u32) -> bool {
    ((a == M) & (b == M) & (c == A) & (d == S) & (e == S))
        | ((a == S) & (b == S) & (c == A) & (d == M) & (e == M))
        | ((a == M) & (b == S) & (c == A) & (d == S) & (e == M))
        | ((a == S) & (b == M) & (c == A) & (d == M) & (e == S))
}

fn part_2(p: &Puzzle) -> usize {
    let mut found = 0;
    // Don't bother scanning until we're on the 3rd line
    for i in p.width * 2..TOTAL_LETTERS {
        // Scanning backwards from bottom right to top left
        // Again we have to avoid wrapping around the puzzle edge due to 1D array
        if i % p.width > 1 {
            found += is_cross_match(
                p.letters[i],
                p.letters[i - 2],
                p.letters[i - p.width - 1],
                p.letters[i - p.width * 2 - 2],
                p.letters[i - p.width * 2],
            ) as usize;
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("./src/example/day_04.txt").expect("File not found.");
        let x = parse(&input);
        assert_eq!(part_1(&x), 18);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./src/example/day_04.txt").expect("File not found.");
        let x = parse(&input);
        assert_eq!(part_2(&x), 9);
    }
}
