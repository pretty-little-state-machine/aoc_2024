#![allow(warnings)]
use crate::DayResult;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&input).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&input).to_string();
    let p2_duration = start.elapsed();
    // (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
    (None, (p1, p1_duration), (p2, p2_duration))
}

fn part_1(input: &str) -> usize {
    0
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {}

    #[test]
    fn test_part_2() {}
}
