extern crate core;

use crate::DayResult;
use fxhash::FxHashMap;
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

#[derive(Debug)]
struct Puzzle {
    rules: FxHashMap<isize, Vec<isize>>,
    updates: [[isize; 20]; 1000],
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            rules: FxHashMap::default(),
            updates: [[-1; 20]; 1000],
        }
    }
}

fn parse(input: &str) -> Puzzle {
    let mut puzzle = Puzzle::default();
    let mut second_block = false;
    let mut update_offset = 0;
    for l in input.lines() {
        println!("{:?}", l);
        if l.is_empty() {
            second_block = true;
            continue;
        }
        if !second_block {
            let mut fields = l.split('|');
            let first_page: isize = fields.next().unwrap().parse().unwrap();
            let second_page: isize = fields.next().unwrap().parse().unwrap();
            puzzle
                .rules
                .entry(first_page)
                .and_modify(|v| v.push(second_page))
                .or_insert(vec![second_page]);
        } else {
            for (i, page) in l.split(',').map(|x| x.parse().unwrap()).enumerate() {
                puzzle.updates[update_offset][i] = page;
            }
            update_offset += 1;
        }
    }
    puzzle
}

fn part_1(p: &Puzzle) -> usize {
    for update in p.updates.iter() {
        // Check all rules for pages
    }
    0
}

fn part_2(p: &Puzzle) -> usize {
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
        assert_eq!(part_1(&x), 143);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./src/example/day_05.txt").expect("File not found.");
        let x = parse(&input);
        assert_eq!(part_2(&x), 9);
    }
}
