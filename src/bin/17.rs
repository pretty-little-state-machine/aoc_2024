use crate::Material::{Clay, Water};
use regex::Regex;
use rustc_hash::FxHashMap;
use std::ops::Range;

type ScanResults = Vec<Scan>;
type Reservoir = FxHashMap<Point, Material>;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point {
    y: usize,
    x: usize,
}

#[derive(Debug, Copy, Clone)]
enum Material {
    Clay,
    Water,
}

#[derive(Debug)]
struct Scan {
    x: Range<usize>,
    y: Range<usize>,
}

fn parse_input(input: &str) -> ScanResults {
    let re = Regex::new(r"([xy])=([\d]+).*=([\d]+)..([\d]+)").unwrap();
    let mut scan_results = ScanResults::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let a = caps[2].parse::<usize>().unwrap()..caps[2].parse::<usize>().unwrap() + 1;
        let b = caps[3].parse::<usize>().unwrap()..caps[4].parse::<usize>().unwrap() + 1;
        let scan = if "y" == &caps[1] {
            Scan { y: a, x: b }
        } else {
            Scan { y: b, x: a }
        };
        scan_results.push(scan);
    }
    scan_results
}

fn build_reservoir(scan_results: &ScanResults) -> Reservoir {
    let mut reservoir = Reservoir::default();
    for result in scan_results {
        for y in result.y.clone() {
            for x in result.x.clone() {
                reservoir.insert(Point { x, y }, Clay);
            }
        }
    }
    reservoir
}

fn bounds(reservoir: &Reservoir) -> (Point, Point) {
    let min_x = reservoir.keys().min_by_key(|v| v.x).unwrap().x;
    let max_x = reservoir.keys().max_by_key(|v| v.x).unwrap().x;
    let min_y = reservoir.keys().min_by_key(|v| v.y).unwrap().y;
    let max_y = reservoir.keys().max_by_key(|v| v.y).unwrap().y;
    (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y })
}

#[allow(dead_code)]
fn debug(reservoir: &Reservoir) {
    let bounds = bounds(reservoir);
    for y in bounds.0.y - 1..bounds.1.y + 1 {
        for x in bounds.0.x - 1..=bounds.1.x + 1 {
            if let Some(material) = reservoir.get(&Point { x, y }) {
                match material {
                    Clay => print!("#"),
                    Water => print!("~"),
                }
            } else {
                print!(".");
            }
        }
        println!()
    }
}

/// Drops water. Water tries to go down, then left as far as it can or right as far as it can. If
/// the water overflows the bounds of the simulation false is immediately returned.
fn add_water(reservoir: &mut Reservoir, y_limit: usize) -> bool {
    let mut drop = Point { x: 500, y: 0 };
    // These two bools constrain horizontal movement so drops don't wiggle back and forth
    let mut may_move_right: bool = true;
    let mut may_move_left: bool = true;
    loop {
        // Fall due to Gravity until the drop hits either water or clay
        while !reservoir.contains_key(&Point {
            x: drop.x,
            y: drop.y + 1,
        }) {
            may_move_right = true;
            may_move_left = true;
            drop.y += 1;
            if drop.y > y_limit {
                reservoir.insert(drop, Water);
                return false; // The water is overflowing!
            }
            continue;
        }
        if may_move_left
            && !reservoir.contains_key(&Point {
                x: drop.x - 1,
                y: drop.y,
            })
        {
            drop.x -= 1;
            may_move_right = false;
            continue;
        }
        if may_move_right
            && !reservoir.contains_key(&Point {
                x: drop.x + 1,
                y: drop.y,
            })
        {
            drop.x += 1;
            may_move_left = false;
            continue;
        }
        break;
    }
    // Commit the drop to stay inside the reservoir
    reservoir.insert(drop, Water);
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let scan_results = parse_input(input);
    let mut reservoir = build_reservoir(&scan_results);
    let bounds = bounds(&reservoir);
    let mut x = 0;
    while x < 50 {
        println!();
        add_water(&mut reservoir, bounds.1.y);
        x += 1;
        debug(&reservoir);
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 17);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
