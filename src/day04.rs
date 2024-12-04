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

// A prime number encoding scheme guarantees that only "XMAS" matches this constant
const XMAS_MATCH: usize = 2_usize.pow(1) * 3_usize.pow(2) * 5_usize.pow(3) * 7_usize.pow(4);
const SAMX_MATCH: usize = 2_usize.pow(4) * 3_usize.pow(3) * 5_usize.pow(2) * 7_usize.pow(1);
const TOTAL_LETTERS: usize = 10 * 10; // From large puzzle input

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
            'X' => puzzle.letters[offset] = 1,
            'M' => puzzle.letters[offset] = 2,
            'A' => puzzle.letters[offset] = 3,
            'S' => puzzle.letters[offset] = 4,
            _ => panic!("Unknown character: {c}"),
        }
        width_check += 1;
        offset += 1;
    }
    puzzle
}

#[inline(always)]
fn is_match(a: u32, b: u32, c: u32, d: u32) -> bool {
    let x = 2_usize.pow(a) * 3_usize.pow(b) * 5_usize.pow(c) * 7_usize.pow(d);
    (x == SAMX_MATCH) | (x == XMAS_MATCH)
}

fn part_1(p: &Puzzle) -> usize {
    let mut found = 0;
    for i in 0..TOTAL_LETTERS {
        // Ensure we don't read across lines using modulo horizontal scan backwards
        if (i % p.width > 2) {
            if is_match(
                p.letters[i],
                p.letters[i - 1],
                p.letters[i - 2],
                p.letters[i - 3],
            ) {
                found += 1;
            }
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
            // Diagonal match - Bottom left to top right - again, ensure we don't "wrap" around
            if (i % p.width < p.width - 2) {
                if is_match(
                    p.letters[i],
                    p.letters[i - p.width + 1],
                    p.letters[i - p.width * 2 + 2],
                    p.letters[i - p.width * 3 + 3],
                ) {
                    found += 1;
                }
            }
            // Diagonal match - Bottom right to top left - ensure we don't "wrap" around
            if (i % p.width > 2) {
                if is_match(
                    p.letters[i],
                    p.letters[i - p.width - 1],
                    p.letters[i - p.width * 2 - 2],
                    p.letters[i - p.width * 3 - 3],
                ) {
                    found += 1;
                }
            }
        }
    }
    found
}

fn part_2(puzzle: &Puzzle) -> usize {
    0
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
        assert_eq!(part_2(&x), 48);
    }
}
