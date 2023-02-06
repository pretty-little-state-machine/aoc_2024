use regex::Regex;
use std::cmp::max;
use std::collections::BTreeMap;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Default)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    /// Calculates the Manhattan Distance between two points
    fn manhattan(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Nanobot {
    point: Point,
    radius: isize,
}

impl Nanobot {
    /// Returns true if the other nanobot is in range of this nanobot.
    #[inline(always)]
    fn in_range(&self, other: &Self) -> bool {
        let distance = self.point.manhattan(&other.point) as isize;
        self.radius >= distance || other.radius >= distance
    }
}

fn parse_input(input: &str) -> Vec<Nanobot> {
    let mut nanobots = Vec::new();
    let re = Regex::new(r"<([-\d]+),([-\d]+),([-\d]+)>,\sr=([-\d]+)").unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        nanobots.push(Nanobot {
            point: Point {
                x: caps[1].parse::<isize>().unwrap(),
                y: caps[2].parse::<isize>().unwrap(),
                z: caps[3].parse::<isize>().unwrap(),
            },
            radius: caps[4].parse::<isize>().unwrap(),
        });
    }
    nanobots
}

/// Collapse the 3D Space into a 1D space to the origin using Manhattan distance
fn solve_with_1d_collapse(nanobots: &[Nanobot]) -> usize {
    let mut tree: BTreeMap<isize, isize> = BTreeMap::new();
    for bot in nanobots {
        let d = bot.point.manhattan(&Point::default()) as isize;
        tree.insert(max(0, d - bot.radius), 1);
        tree.insert(d + bot.radius + 1, -1);
    }
    collapse_overlapping_ranges(&mut tree) as usize
}

/// Collapses a set of overlapping ranges into the length of overlaps where
fn collapse_overlapping_ranges(tree: &mut BTreeMap<isize, isize>) -> isize {
    let mut count = 0;
    let mut max_count = 0;
    let mut result = 0;
    while let Some((distance, value)) = tree.pop_first() {
        count += value;
        if count > max_count {
            result = distance;
            max_count = count;
        }
    }
    result
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut bots = parse_input(input);
    bots.sort_by_key(|b| b.radius);
    let last_bot = bots.last().unwrap();
    let num_in_range = bots.iter().map(|b| last_bot.in_range(b) as usize).sum();
    Some(num_in_range)
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve_with_1d_collapse(&parse_input(input)))
}

fn main() {
    let input = &aoc::read_file("inputs", 23);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(6));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(36));
    }
}
