extern crate core;

use crate::DayResult;
use std::time::Instant;

use fxhash::FxHashMap;
use num::abs;

const ASCII_LINESEP: u8 = 10;
const ASCII_SPACE: u8 = 32;
const ASCII_0: u8 = 48;
const ASCII_9: u8 = 57;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let (mut a, mut b) = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&mut a, &mut b).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&mut a, &mut b).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

fn parse(input: &str) -> (Vec<isize>, Vec<isize>) {
    let mut a = Vec::with_capacity(1500);
    let mut b = Vec::with_capacity(1500);
    let mut second_block = false;
    let mut x: isize = 0;
    for c in input.bytes() {
        match (c, second_block) {
            (ASCII_LINESEP, true) => {
                b.push(x);
                x = 0;
                second_block = false;
            }
            (ASCII_SPACE, false) => {
                a.push(x);
                x = 0;
                second_block = true;
            }
            (ASCII_SPACE, true) => (),
            (ASCII_0..=ASCII_9, _) => x = x * 10 + (c - ASCII_0) as isize,
            _ => (),
        }
    }
    // The puzzle input may not have a trailing linebreak at the end, ensure we save the last value
    if second_block {
        b.push(x);
    }
    (a, b)
}

pub fn part_1(a: &mut [isize], b: &mut [isize]) -> isize {
    a.sort();
    b.sort();
    let mut sum: isize = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        sum += abs(x - y);
    }
    sum
}

pub fn part_2(a: &mut [isize], b: &mut [isize]) -> isize {
    let mut frequencies: FxHashMap<isize, isize> = FxHashMap::default();
    for b in b.iter() {
        *frequencies.entry(*b).or_insert(0) += 1;
    }
    let sim_score: isize = a
        .iter()
        .map(|x| {
            if let Some(v) = frequencies.get(x) {
                x * v
            } else {
                0
            }
        })
        .sum();
    sim_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("./src/example/day_01.txt").expect("File not found.");
        let (mut a, mut b) = parse(&input);

        assert_eq!(part_1(&mut a, &mut b), 11);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./src/example/day_01.txt").expect("File not found.");
        let (mut a, mut b) = parse(&input);
        assert_eq!(part_2(&mut a, &mut b), 31);
    }
}
