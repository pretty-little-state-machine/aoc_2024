extern crate core;

use crate::DayResult;
use fxhash::FxHashMap;
use itertools::Itertools;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let data = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let (p1, invalid_pages) = process_pages(&data);
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&data, invalid_pages).to_string();
    let p2_duration = start.elapsed();
    (
        Some(parse_duration),
        (p1.to_string(), p1_duration),
        (p2, p2_duration),
    )
}

#[derive(Debug, Default)]
struct Puzzle {
    rules: FxHashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

fn parse(input: &str) -> Puzzle {
    let mut puzzle = Puzzle::default();
    let mut second_block = false;
    let mut update_offset = 0;
    for l in input.lines() {
        if l.is_empty() {
            second_block = true;
            continue;
        }
        if !second_block {
            let mut fields = l.split('|');
            let first_page = fields.next().unwrap().parse().unwrap();
            let second_page = fields.next().unwrap().parse().unwrap();
            puzzle
                .rules
                .entry(first_page)
                .and_modify(|v| v.push(second_page))
                .or_insert(vec![second_page]);
        } else {
            puzzle.updates.push(Vec::default());
            for page in l.split(',').map(|x| x.parse().unwrap()) {
                puzzle.updates[update_offset].push(page);
            }
            update_offset += 1;
        }
    }
    puzzle
}

fn process_pages(p: &Puzzle) -> (usize, Vec<Vec<usize>>) {
    let mut invalid_updates = Vec::new();
    let total = p
        .updates
        .iter()
        .map(|u| {
            let mut valid_update = true;
            for (page_index, page) in u.iter().enumerate() {
                let pages_after = &u[page_index..u.len()];
                for pa in pages_after {
                    if valid_update {
                        if let Some(rule) = p.rules.get(page) {
                            valid_update = rule.contains(pa);
                        }
                        if let Some(rule) = p.rules.get(pa) {
                            valid_update = !rule.contains(page);
                        }
                    }
                }
            }
            if valid_update {
                u[(u.len() - 1) / 2]
            } else {
                invalid_updates.push(u.clone());
                0
            }
        })
        .sum();
    (total, invalid_updates)
}

fn part_2(p: &Puzzle, invalid_pages: Vec<Vec<usize>>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("./src/example/day_05.txt").expect("File not found.");
        let x = parse(&input);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./src/example/day_05.txt").expect("File not found.");
        let x = parse(&input);
        let (_, invalid_pages) = process_pages(&x);
        assert_eq!(part_2(&x, invalid_pages), 9);
    }
}
