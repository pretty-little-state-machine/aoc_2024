extern crate core;

use crate::DayResult;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let data = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&data).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&data).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}
type Equation = (usize, Vec<usize>);

#[derive(Debug, Default, Clone)]
struct Puzzle {
    equations: Vec<Equation>,
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

#[derive(Debug)]
pub struct Node {
    value: usize,
    depth: usize,
}

#[inline(always)]
fn run_dfs(equation: &Equation, concat: bool) -> usize {
    let (target, values) = equation;
    let mut queue = VecDeque::new();
    queue.push_back(Node {
        value: *values.first().unwrap(),
        depth: 1,
    });
    while let Some(node) = queue.pop_front() {
        if (node.value == *target) & (node.depth == values.len()) {
            return node.value;
        }
        // Too big - Abandon this branch of the tree
        if node.value > *target {
            continue;
        }
        if let Some(next) = values.get(node.depth) {
            queue.push_front(Node {
                value: node.value + next,
                depth: node.depth + 1,
            });
            queue.push_front(Node {
                value: node.value * next,
                depth: node.depth + 1,
            });
            if concat {
                queue.push_front(Node {
                    value: concat_ints(&node.value, next),
                    depth: node.depth + 1,
                });
            }
        }
    }
    0
}

fn part_1(p: &Puzzle) -> usize {
    p.equations.par_iter().map(|e| run_dfs(e, false)).sum()
}

fn part_2(p: &Puzzle) -> usize {
    p.equations.par_iter().map(|e| run_dfs(e, true)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("./src/example/day_07.txt").expect("File not found.");
        let x = parse(&input);
        assert_eq!(part_1(&x), 3749);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./src/example/day_07.txt").expect("File not found.");
        let x = parse(&input);
        assert_eq!(part_2(&x), 11387);
    }
}
