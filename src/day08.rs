extern crate core;

use crate::DayResult;
use fxhash::FxHashMap;
use itertools::Itertools;
use num::Complex;
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
#[derive(Debug, Default, Clone)]
struct Puzzle {
    positions: FxHashMap<Complex<isize>, char>,
}

fn parse(input: &str) -> Puzzle {
    let mut puzzle = Puzzle::default();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            puzzle
                .positions
                .insert(Complex::new(x as isize, y as isize), char);
        }
    }
    puzzle
}

// Builds anti-nodes with the same characters as the antenna that made them
fn build_antinodes(antennas: &Puzzle, account_for_harmonics: bool) -> Puzzle {
    let depth = if account_for_harmonics { 0..=40 } else { 1..=1 };
    let mut antinodes = Puzzle::default();
    let antenna_chars = antennas
        .positions
        .iter()
        .map(|(_, &v)| v)
        .unique()
        .filter(|c| *c != '.')
        .collect::<Vec<_>>();
    for a in antenna_chars {
        antennas
            .positions
            .iter()
            .filter(|(_, &v)| v == a)
            .map(|(k, _)| k)
            .permutations(2)
            .for_each(|pos| {
                for d in depth.clone() {
                    let delta =
                        Complex::new((pos[0].re - pos[1].re) * d, (pos[0].im - pos[1].im) * d);
                    let antinode = pos[0] + delta;
                    if antennas.positions.contains_key(&antinode) {
                        antinodes.positions.insert(antinode, a);
                    }
                }
            });
    }
    antinodes
}

fn part_1(p: &Puzzle) -> usize {
    build_antinodes(p, false).positions.len()
}

fn part_2(p: &Puzzle) -> usize {
    build_antinodes(p, true).positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("./src/example/day_08.txt").expect("File not found.");
        let x = parse(&input);
        assert_eq!(part_1(&x), 14);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./src/example/day_08.txt").expect("File not found.");
        let x = parse(&input);
        assert_eq!(part_2(&x), 34);
    }
}
