extern crate core;

use crate::DayResult;
use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};
use num::Complex;
use std::collections::HashSet;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let data = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&mut data.clone()).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&mut data.clone()).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}
type WalkResult = Option<HashSet<(Complex<isize>, Complex<isize>), FxBuildHasher>>;

#[derive(Debug, Default, Clone)]
struct Puzzle {
    map: FxHashMap<Complex<isize>, char>,
    cursor: Complex<isize>,
}

fn parse(input: &str) -> Puzzle {
    let mut puzzle = Puzzle::default();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char.eq(&'^') {
                puzzle.cursor = Complex::new(x as isize, y as isize);
                puzzle.map.insert(Complex::new(x as isize, y as isize), '.');
            } else {
                puzzle
                    .map
                    .insert(Complex::new(x as isize, y as isize), char);
            }
        }
    }
    puzzle
}

#[inline(always)]
fn walk_guard(p: &mut Puzzle, check_loop: bool) -> WalkResult {
    let mut seen_positions = FxHashSet::default();
    let mut direction = Complex::new(0, -1); // Facing NORTH initially
    seen_positions.insert((p.cursor, direction));
    while let Some(tile) = p.map.get(&p.cursor) {
        match tile {
            '.' => {
                if check_loop {
                    seen_positions.insert((p.cursor, direction));
                } else {
                    // Never adjust seen direction if we're not checking for loops in the visited
                    seen_positions.insert((p.cursor, Complex::new(0, -1)));
                }
            }
            '#' => {
                p.cursor -= direction; // Step back from the impact
                direction *= Complex::i();
            }
            _ => panic!("Unknown Tile"),
        }
        p.cursor += direction;
        if seen_positions.contains(&(p.cursor, direction)) & check_loop {
            return None;
        }
    }
    Some(seen_positions)
}

fn part_1(p: &mut Puzzle) -> usize {
    walk_guard(p, false).unwrap().len()
}

fn part_2(p: &mut Puzzle) -> usize {
    let starting_point = p.cursor;
    let visited = walk_guard(p, false).unwrap();
    // Walk with obstructions added
    visited
        .iter()
        .map(|(tile, _)| {
            p.cursor = starting_point;
            p.map.insert(*tile, '#'); // Block a tile
            let walk = walk_guard(p, true);
            p.map.insert(*tile, '.'); // Restore a tile
            walk.is_none() as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("./src/example/day_06.txt").expect("File not found.");
        let mut x = parse(&input);
        assert_eq!(part_1(&mut x), 41);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./src/example/day_06.txt").expect("File not found.");
        let mut x = parse(&input);
        assert_eq!(part_2(&mut x), 6);
    }
}
