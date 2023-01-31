use crate::Equipment::{ClimbingGear, Neither, Torch};
use crate::Terrain::{Narrow, Rocky, Wet};
use num_traits::cast::FromPrimitive;
use std::cmp::min;
use std::collections::VecDeque;
use rustc_hash::FxHashSet;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

#[derive(Debug, Primitive, Eq, PartialEq)]
enum Terrain {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

impl Terrain {
    fn from_geologic_index(index: usize) -> Self {
        Terrain::from_usize(index % 3).unwrap()
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Survey {
    target: Point,
    geologic_index: Vec<Vec<usize>>,
}

impl Survey {
    fn new(depth: usize, target: &Point) -> Self {
        // The puzzle states our path may exceed the bounding box of the target.
        const BEYOND_TARGET:usize = 20;
        let mut geologic_index: Vec<Vec<usize>> = Vec::with_capacity(target.y);
        for _ in 0..=target.y + BEYOND_TARGET + 1{
            geologic_index.push(vec![0; target.x + BEYOND_TARGET + 1]);
        }
        for y in 0..=target.y + BEYOND_TARGET {
            for x in 0..=target.x + BEYOND_TARGET {
                let value = match (x, y) {
                    (0, 0) => 0,
                    (x, 0) => ((x * 16_807) + depth) % 20_183,
                    (0, y) => ((y * 48_271) + depth) % 20_183,
                    (x, y) => {
                        ((geologic_index[y][x - 1] * geologic_index[y - 1][x]) + depth) % 20_183
                    }
                };
                geologic_index[y][x] = value;
            }
        }
        geologic_index[target.y][target.x] = 0; // Special case due to puzzle instructions
        Self {
            target: *target,
            geologic_index,
        }
    }

    /// Risk Levels are the sums of all cells bounded by (0,0) and the point at the Target
    fn risk_level(&self) -> usize {
        let mut risk_level = 0;
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                risk_level += self.geologic_index[y][x] % 3
            }
        }
        risk_level
    }

    fn debug(&self) {
        for y in 0..=self.target.y {
            for x in 0..=self.target.x {
                let terrain: Terrain =
                    Terrain::from_geologic_index(self.geologic_index[y][x]);
                match terrain {
                    Rocky => print!("."),
                    Wet => print!("="),
                    Narrow => print!("|"),
                }
            }
            println!()
        }
    }

    fn get_successors(&self, point: &Point) -> Vec<(Point, Equipment)> {
        let mut successors: Vec<(Point, Equipment)> = Vec::new();
        for n in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let x = point.x as isize + n.0;
            let y = point.y as isize + n.1;
            if y >= 0 && x >= 0 {
                let x = x as usize;
                let y = y as usize;
                if let Some(row) = self.geologic_index.get(y) {
                    if let Some(cell) = row.get(x) {
                        let terrain = Terrain::from_geologic_index(*cell);
                        match terrain {
                            Rocky => {
                                successors.push((Point { x, y }, ClimbingGear));
                                successors.push((Point { x, y }, Torch));
                            }
                            Wet => {
                                successors.push((Point { x, y }, ClimbingGear));
                                successors.push((Point { x, y }, Neither));
                            }
                            Narrow => {
                                successors.push((Point { x, y }, Torch));
                                successors.push((Point { x, y }, Neither));
                            }
                        }
                    }
                }
            }
        }
        successors
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
enum Equipment {
    #[default]
    Neither,
    Torch,
    ClimbingGear,
}

#[derive(Default, Debug)]
struct State {
    position: Point,
    equipped: Equipment,
    elapsed: usize,
    visited: FxHashSet<Point>,
}

/// Finds the quickest traversal and returns the number of elapsed minutes.
fn find_quickest_path(state: State, target: &Point, survey: &Survey) -> usize {
    let mut fastest_time = usize::MAX;
    let mut queue = VecDeque::new();
    queue.push_back(state);
    while let Some(current) = queue.pop_front() {
        if current.position == *target {
            continue;
        }
        let successors = survey.get_successors(&current.position);
        for (position, equipped) in successors {
            if current.visited.contains(&position) {
                continue;
            }
            let elapsed = if current.equipped == equipped {
                1 + current.elapsed
            } else {
                8 + current.elapsed
            };
            let mut visited = current.visited.clone();
            visited.insert(position);
            let new_state = State {
                position,
                equipped,
                elapsed,
                visited,
            };
            if position.x > 9 && position.y > 9 {
                println!("{new_state:?}");
            }
            queue.push_back(new_state);
            if queue.len() > 10_000_000 {
                return 0;
            }
        }
        fastest_time = min(fastest_time, usize::MAX - current.elapsed);
    }
    usize::MAX - fastest_time
}

pub fn part_one(input: &str) -> Option<usize> {
    let survey = Survey::new(9171, &Point { x: 7, y: 721 });
    Some(survey.risk_level())
}

pub fn part_two(input: &str) -> Option<usize> {
    // let target = Point{x: 7, y: 721};
    // let survey = Survey::new(9171, &target);
    let target = Point{x: 10, y: 10};
    let survey = Survey::new(510, &target);
    // There are two possible starting states, one with a torch and one with climbing gear
    let torch_state = State{
        position: Point{x:0 , y: 0},
        equipped: Torch,
        elapsed: 0,
        visited: FxHashSet::default(),
    };
    Some(find_quickest_path(torch_state, &target, &survey))
    // let successors = survey.get_successors(&Point{x:10, y:10});
    // println!("{successors:?}");
    // None
}

fn main() {
    let input = &aoc::read_file("inputs", 22);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(114));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(45));
    }
}
