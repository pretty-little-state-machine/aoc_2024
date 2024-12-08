extern crate core;

use crate::DayResult;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let mut data = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&mut data.clone()).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&mut data.clone()).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

#[derive(Debug, Default, Clone)]
struct Puzzle {
    equations: Vec<(usize, Vec<usize>)>,
}

fn parse(input: &str) -> Puzzle {
    let mut puzzle = Puzzle::default();
    for line in input.lines() {
        let mut fields = line.split(": ");
        let target: usize = fields.next().unwrap().parse().unwrap();
        let values = fields
            .next()
            .unwrap()
            .split(" ")
            .map(|s| s.parse().unwrap())
            .collect::<Vec<usize>>();
        puzzle.equations.push((target, values));
    }
    puzzle
}

#[inline(always)]
fn concat_ints(a: &usize, b: &usize) -> usize {
    match b {
        0..=10 => a * 10 + b,
        11..=100 => a * 100 + b,
        101..=1_000 => a * 1_000 + b,
        1001..=10_000 => a * 10_000 + b,
        10_001..=100_000 => a * 100_000 + b,
        100_001..=1_000_000 => a * 1_000_000 + b,
        1_000_001..=10_000_000 => a * 10_000_000 + b,
        10_000_001..=100_000_000 => a * 100_000_000 + b,
        100_000_001..=1_000_000_000 => a * 1_000_000_000 + b,
        1_000_000_001..=10_000_000_000 => a * 10_000_000_000 + b,
        10_000_000_001..=100_000_000_000 => a * 100_000_000_000 + b,
        _ => panic!("BIG NUMBER"),
    }
}

fn part_1(p: &mut Puzzle) -> usize {
    p.equations
        .par_iter()
        .map(|(target, values)| {
            let mut queue = VecDeque::new();
            queue.push_back(DfsNode {
                value: *values.first().unwrap(),
                depth: 1,
            });
            while let Some(node) = queue.pop_front() {
                if node.value == *target {
                    return node.value;
                }
                // Too big - Abandon this branch of the tree
                if node.value > *target {
                    continue;
                }
                if let Some(next) = values.get(node.depth) {
                    queue.push_front(DfsNode {
                        value: node.value + next,
                        depth: node.depth + 1,
                    });
                    queue.push_front(DfsNode {
                        value: node.value * next,
                        depth: node.depth + 1,
                    });
                }
            }
            0
        })
        .sum()
}

pub struct DfsNode {
    value: usize,
    depth: usize,
}

fn part_2(p: &mut Puzzle) -> usize {
    p.equations
        .par_iter()
        .map(|(target, values)| {
            let mut queue = VecDeque::new();
            queue.push_back(DfsNode {
                value: *values.first().unwrap(),
                depth: 1,
            });
            while let Some(node) = queue.pop_front() {
                if node.value == *target {
                    return node.value;
                }
                // Too big - Abandon this branch of the tree
                if node.value > *target {
                    continue;
                }
                if let Some(next) = values.get(node.depth) {
                    queue.push_front(DfsNode {
                        value: node.value + next,
                        depth: node.depth + 1,
                    });
                    queue.push_front(DfsNode {
                        value: node.value * next,
                        depth: node.depth + 1,
                    });
                    queue.push_front(DfsNode {
                        value: concat_ints(&node.value, &next),
                        depth: node.depth + 1,
                    });
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("./src/example/day_07.txt").expect("File not found.");
        let mut x = parse(&input);
        assert_eq!(part_1(&mut x), 3749);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./src/example/day_07.txt").expect("File not found.");
        let mut x = parse(&input);
        assert_eq!(part_2(&mut x), 11387);
    }
}
