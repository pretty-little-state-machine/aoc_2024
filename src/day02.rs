extern crate core;

use crate::day02::LevelDirection::{Decreasing, Increasing, Unchanged};
use crate::day02::ReactorHealth::{Safe, Unsafe};
use crate::DayResult;
use num::abs;
use std::cmp::{Ordering, PartialEq};
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let reactors = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&mut reactors.clone()).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&mut reactors.clone()).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

const ASCII_LINESEP: u8 = 10;
const ASCII_SPACE: u8 = 32;
const ASCII_0: u8 = 48;
const ASCII_9: u8 = 57;

#[derive(Clone, PartialEq, Debug)]
enum LevelDirection {
    Unchanged,
    Increasing,
    Decreasing,
}

#[derive(Clone, PartialEq, Debug)]
enum ReactorHealth {
    Safe,
    Unsafe,
}

#[derive(Clone, PartialEq, Debug)]
struct Reactor {
    // We use a fixed-length array to reduce Vec<isize> allocations, much faster! 600us -> 23us
    values: [isize; 10],
    health: ReactorHealth,
    level_direction: Option<LevelDirection>,
}

impl Default for Reactor {
    fn default() -> Reactor {
        Self {
            values: [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
            health: Safe,
            level_direction: None,
        }
    }
}

fn parse(input: &str) -> Vec<Reactor> {
    let mut reactors: Vec<Reactor> = Vec::with_capacity(1000);
    let mut reactor = Reactor::default();
    let mut x = 0;
    let mut line_break = false;
    let mut offset = 0;
    for b in input.bytes() {
        match b {
            ASCII_LINESEP => {
                reactor.values[offset] = x;
                reactors.push(reactor);
                reactor = Reactor::default();
                offset = 0;
                x = 0;
                line_break = true;
            }
            ASCII_SPACE => {
                reactor.values[offset] = x;
                offset += 1;
                x = 0;
                line_break = false;
            }
            ASCII_0..=ASCII_9 => {
                x = x * 10 + (b - ASCII_0) as isize;
                line_break = false;
            }
            _ => panic!("Unexpected ASCII byte: {}", b),
        }
    }
    // The puzzle input may not have a trailing linebreak at the end, ensure we count the last check
    if !line_break {
        reactor.values[offset] = x;
        reactors.push(reactor);
    }
    reactors
}

fn part_1(reactors: &mut Vec<Reactor>) -> usize {
    let mut safe_count = 0;
    for reactor in reactors {
        run_reactor_test(reactor);
        if reactor.health == Safe {
            safe_count += 1;
        }
    }
    safe_count
}

#[inline(always)]
fn run_reactor_test(reactor: &mut Reactor) {
    let mut prev_x: Option<isize> = None;
    for x in &reactor.values {
        if *x == -1 {
            break;
        }
        if *x == -2 {
            continue;
        }
        if reactor.health != Unsafe
            && let Some(y) = prev_x
        {
            let current_direction = match x.cmp(&y) {
                Ordering::Greater => Increasing,
                Ordering::Less => Decreasing,
                _ => Unchanged,
            };
            // Values out of bounds are reactor failures
            let delta = abs(x - y);
            if !(1..=3).contains(&delta) {
                reactor.health = Unsafe;
            }
            // Reactor values that change direction are failures
            if let Some(reactor_direction) = &reactor.level_direction {
                if current_direction != *reactor_direction {
                    reactor.health = Unsafe;
                }
            } else {
                reactor.level_direction = Some(current_direction);
            }
        }
        prev_x = Some(*x);
    }
}

fn part_2(reactors: &mut Vec<Reactor>) -> usize {
    let mut safe_count = 0;

    for reactor in reactors {
        let mut reactor_success = false;
        let original_values = reactor.values;
        for offset in 0..10 {
            // Reset the reactor for every permutation of values
            reactor.health = Safe;
            reactor.level_direction = None;
            reactor.values = original_values;
            reactor.values[offset] = -2;
            run_reactor_test(reactor);
            if reactor.health == Safe {
                reactor_success = true;
            }
        }
        if reactor_success {
            safe_count += 1;
        }
    }
    safe_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("./src/example/day_02.txt").expect("File not found.");
        let mut reactors = parse(&input);
        assert_eq!(part_1(&mut reactors), 2);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./src/example/day_02.txt").expect("File not found.");
        let mut reactors = parse(&input);
        assert_eq!(part_2(&mut reactors), 4);
    }
}
