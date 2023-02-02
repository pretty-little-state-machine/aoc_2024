#![feature(stmt_expr_attributes)]
use crate::Equipment::{ClimbingGear, Neither, Torch};
use crate::Terrain::{Narrow, Rocky, Wet};
use num_traits::cast::FromPrimitive;
use pathfinding::prelude::dijkstra;

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
        const BEYOND_TARGET: usize = 100;
        let mut geologic_index: Vec<Vec<usize>> = Vec::with_capacity(target.y);
        for _ in 0..=target.y + BEYOND_TARGET + 1 {
            geologic_index.push(vec![0; target.x + BEYOND_TARGET + 1]);
        }
        for y in 0..=target.y + BEYOND_TARGET {
            for x in 0..=target.x + BEYOND_TARGET {
                let value = match (x, y) {
                    (0, 0) => 0,
                    (x, y) if target.x == x && target.y == y => 0,
                    (x, 0) => (x * 16_807) + depth,
                    (0, y) => (y * 48_271) + depth,
                    (x, y) => (geologic_index[y][x - 1] * geologic_index[y - 1][x]) + depth,
                };
                geologic_index[y][x] = value % 20_183;
            }
        }
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

    #[allow(dead_code)]
    fn debug(&self, paths: &[State]) {
        for y in 0..self.geologic_index.len() - 1 {
            for x in 0..self.geologic_index[0].len() {
                if let Some(state) = paths
                    .iter()
                    .filter(|state| state.position == Point { x, y })
                    .copied()
                    .collect::<Vec<State>>()
                    .first()
                {
                    match state.equipped {
                        Neither => print!("\x1b[1;38;5;8;107mN\x1b[0m"),
                        Torch => print!("\x1b[1;38;5;202;107mT\x1b[0m"),
                        ClimbingGear => print!("\x1b[1;38;5;21;107mC\x1b[0m"),
                    }
                } else {
                    let terrain: Terrain = Terrain::from_geologic_index(self.geologic_index[y][x]);
                    match terrain {
                        Rocky => print!("\x1b[1;38;5;223;40m#\x1b[0m"),
                        Wet => print!("\x1b[1;38;5;117;40m~\x1b[0m"),
                        Narrow => print!("\x1b[1;38;5;244;40m=\x1b[0m"),
                    }
                }
            }
            println!()
        }
    }
}

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Equipment {
    #[default]
    Neither,
    Torch,
    ClimbingGear,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    position: Point,
    equipped: Equipment,
}

impl State {
    fn successors(&self, survey: &Survey) -> Vec<(State, usize)> {
        let mut successors: Vec<(State, usize)> = Vec::new();
        let current_terrain =
            Terrain::from_geologic_index(survey.geologic_index[self.position.y][self.position.x]);
        // Tool swaps
        #[rustfmt::skip]
        match (&current_terrain, self.equipped) {
            (Rocky, ClimbingGear) => {
                successors.push((State{ position: self.position, equipped: Torch}, 7));
            }
            (Rocky, Torch) => {
                successors.push((State{ position: self.position, equipped: ClimbingGear}, 7));
            }
            (Wet, ClimbingGear) => {
                successors.push((State{ position: self.position, equipped: Neither}, 7));
            }
            (Wet, Neither) => {
                successors.push((State{ position: self.position, equipped: ClimbingGear}, 7));
            }
            (Narrow, Torch) => {
                successors.push((State{ position: self.position, equipped: Neither}, 7));
            }
            (Narrow, Neither) => {
                successors.push((State{ position: self.position, equipped: Torch}, 7));
            }
            _ => unreachable!("Invalid combination: {:?}, {:?}", current_terrain, self.equipped),
        }
        // Moving to new cells
        for n in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let x = self.position.x as isize + n.0;
            let y = self.position.y as isize + n.1;
            if y >= 0 && x >= 0 {
                let x = x as usize;
                let y = y as usize;
                if let Some(row) = survey.geologic_index.get(y) {
                    if let Some(cell) = row.get(x) {
                        let new_terrain = Terrain::from_geologic_index(*cell);
                        #[rustfmt::skip]
                        // We can't move into the zone as we switch tools, so the tool must be valid
                        // in both the current terrain _AND_ the new terrain.
                        //
                        // TODO: There's a better way to do this with sets and intersections to
                        // scale to many more options, but we know this problem won't be adding more
                        // types so I've left them broken out for clarity's sake.
                        match (self.equipped, &new_terrain) {
                            // Climbing gear may be used in Rocky or Wet, but not narrow
                            (ClimbingGear, Rocky) => {
                                successors.push((State{ position: Point {x, y}, equipped: ClimbingGear}, 1));
                            },
                            (ClimbingGear, Wet) => {
                                successors.push((State{ position: Point {x, y}, equipped: ClimbingGear}, 1));
                            },
                            // Torches may be used in Rocky or Narrow, but not Wet
                            (Torch, Rocky) => {
                                successors.push((State{ position: Point {x, y}, equipped: Torch}, 1));
                            }
                            (Torch, Narrow) => {
                                successors.push((State{ position: Point {x, y}, equipped: Torch}, 1));
                            }
                            // Neither may be used in Wet or Narrow, but not Rocky
                            (Neither, Wet) => {
                                successors.push((State{ position: Point {x, y}, equipped: Neither}, 1));
                            }
                            (Neither, Narrow) => {
                                successors.push((State{ position: Point {x, y}, equipped: Neither}, 1));
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        successors
    }
}

/// Returns the Depth and Target from the puzzle input
fn read_input(input: &str) -> (usize, Point) {
    let lines = input.lines().collect::<Vec<&str>>();
    let depth = lines.first().unwrap().split(": ").collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();
    let coords = lines.get(1).unwrap().split(": ").collect::<Vec<&str>>()[1]
        .split(',')
        .collect::<Vec<&str>>();
    (
        depth,
        Point {
            x: coords[0].parse::<usize>().unwrap(),
            y: coords[1].parse::<usize>().unwrap(),
        },
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let (depth, target) = read_input(input);
    let survey = Survey::new(depth, &target);
    Some(survey.risk_level())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (depth, target) = read_input(input);
    let survey = Survey::new(depth, &target);
    let torch_state = State {
        position: Point { x: 0, y: 0 },
        equipped: Torch,
    };
    let goal = State {
        position: target,
        equipped: Torch,
    };
    if let Some((_path, cost)) = dijkstra(&torch_state, |s| s.successors(&survey), |s| *s == goal) {
        // survey.debug(&path);
        Some(cost)
    } else {
        None
    }
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
