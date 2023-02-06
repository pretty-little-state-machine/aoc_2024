use itertools::any;
use regex::Regex;
use rustc_hash::FxHashMap;
use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, BTreeMap};

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
    num_in_range: usize,
}

impl Nanobot {
    #[inline(always)]
    fn new(point: Point, radius: isize, nanobots: &[Nanobot]) -> Self {
        let mut slf = Self {
            point,
            radius,
            num_in_range: 0,
        };
        slf.num_in_range = slf.bots_in_range(nanobots);
        slf
    }

    /// Returns true if the other nanobot is in range of this nanobot.
    #[inline(always)]
    fn in_range(&self, other: &Self) -> bool {
        let distance = self.point.manhattan(&other.point) as isize;
        self.radius >= distance || other.radius >= distance
    }

    #[inline(always)]
    fn bots_in_range(&self, nanobots: &[Nanobot]) -> usize {
        nanobots
            .iter()
            .map(|other| self.in_range(other) as usize)
            .sum()
    }
}

impl Ord for Nanobot {
    /// The priority for nanobots is and their tie-breakers:
    ///  1 - Most Bots
    ///  2 - Closest to Origin - Note the flip of other/self here!
    ///  3 - Smallest Search Radius - Note the flip of other/self here!
    #[rustfmt::skip]
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = Point { x: 0, y: 0, z: 0 };
        self.num_in_range.cmp(&other.num_in_range)
            .then_with(|| {
                other.point.manhattan(&origin).cmp(&self.point.manhattan(&origin))
            })
    }
}

impl PartialOrd for Nanobot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
            num_in_range: 0,
        });
    }
    nanobots
}


#[inline(always)]
#[rustfmt::skip]
fn get_starting_nanobot(nanobots: &[Nanobot]) -> Nanobot {
    let min_x = nanobots.iter().min_by_key(|n| n.point.x).unwrap().point.x;
    let max_x = nanobots.iter().max_by_key(|n| n.point.x).unwrap().point.x;
    let min_y = nanobots.iter().min_by_key(|n| n.point.y).unwrap().point.y;
    let max_y = nanobots.iter().max_by_key(|n| n.point.y).unwrap().point.y;
    let min_z = nanobots.iter().min_by_key(|n| n.point.z).unwrap().point.z;
    let max_z = nanobots.iter().max_by_key(|n| n.point.z).unwrap().point.z;
    let mut nanobot = Nanobot{ point: Point {
            x: min_x + (max_x - min_x)/2,
            y: min_y + (max_y - min_y)/2,
            z: min_z + (max_z - min_z)/2,
        },
        radius: 1,
        num_in_range: 0,
    };
    // Expand it until it encompasses all bots
    let target_bots = nanobots.len();
    while nanobot.num_in_range != target_bots {
        nanobot.radius *= 2;
        nanobot.num_in_range = nanobot.bots_in_range(nanobots);
    }
    nanobot
}

fn subdivide_search_bots(bot: &Nanobot, heap: &mut BinaryHeap<Nanobot>, nanobots: &[Nanobot]) {
    // Special cases where the radius can't subdivide any longer
    if bot.radius == 1 {
        heap.push(Nanobot::new(bot.point, 0, nanobots));
    } else if bot.radius == 2 {
        heap.push(Nanobot::new(bot.point, 1, nanobots));
    } else {
        let new_radius = (bot.radius as f64 * 0.666).ceil() as isize;
        let offset = bot.radius - new_radius;
        #[rustfmt::skip]
            let points = [
            Point { x: bot.point.x + offset, y: bot.point.y, z: bot.point.z },
            Point { x: bot.point.x - offset, y: bot.point.y, z: bot.point.z },
            Point { x: bot.point.x, y: bot.point.y + offset, z: bot.point.z },
            Point { x: bot.point.x, y: bot.point.y - offset, z: bot.point.z },
            Point { x: bot.point.x, y: bot.point.y, z: bot.point.z + offset },
            Point { x: bot.point.x, y: bot.point.y, z: bot.point.z - offset },
        ];
        for point in points {
            let n = Nanobot::new(point, new_radius, nanobots);
            // println!("    {n:?}");
            heap.push(n);
        }
    }
}

/// Uses an Octree to narrow down the search space using a big nanobot that subdivides
/// https://en.wikipedia.org/wiki/Octree
fn find_best_point_octahedron(nanobots: &[Nanobot]) -> Point {
    let mut search_heap = BinaryHeap::new();
    search_heap.push(get_starting_nanobot(nanobots));

    while let Some(search_bot) = search_heap.pop() {
        println!("{:?}", search_bot);
        if search_bot.radius == 0 {
            return search_bot.point;
        }
        subdivide_search_bots(&search_bot, &mut search_heap, nanobots);
    }
    Point::default()
}

/// Collapse the 3D Space into a 1D space to the origin using Manhattan distance
fn solve_with_1d_collapse(nanobots: &[Nanobot]) -> usize {
    let mut x_tree: BTreeMap<isize, isize> = BTreeMap::new();
    let mut y_tree: BTreeMap<isize, isize> = BTreeMap::new();
    let mut z_tree: BTreeMap<isize, isize> = BTreeMap::new();
    for bot in nanobots {
        let dx =  Point{x: bot.point.x, y: 0, z: 0}.manhattan(&Point::default()) as isize;
        x_tree.insert(max(0, dx - bot.radius), 1);
        x_tree.insert(dx + bot.radius + 1, -1);

        let dy =  Point{x: 0, y: bot.point.y, z: 0}.manhattan(&Point::default()) as isize;
        y_tree.insert(max(0, dy - bot.radius), 1);
        y_tree.insert(dy + bot.radius + 1, -1);

        let dz =  Point{x: 0, y: 0, z: bot.point.z}.manhattan(&Point::default()) as isize;
        z_tree.insert(max(0, dz - bot.radius), 1);
        z_tree.insert(dz + bot.radius + 1, -1);
    }
    let x = collapse_overlapping_ranges(&mut x_tree);
    let y = collapse_overlapping_ranges(&mut y_tree);
    let z = collapse_overlapping_ranges(&mut z_tree);
    println!("{x}, {y}, {z}");
    Point{x, y, z}.manhattan(&Point::default())
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
    let bots = parse_input(input);
    // let point = find_best_point_octahedron(&bots);
    // Some(point.manhattan(&Point::default()))
    Some(solve_with_1d_collapse(&bots))
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
