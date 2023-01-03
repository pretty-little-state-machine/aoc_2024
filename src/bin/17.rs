use crate::JetDirection::{Left, Right};
use crate::Shape::{Bar, Line, Plus, ReverseL, Square};
use itertools::{all, any};
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::BuildHasherDefault;

#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

struct Chamber {
    grid: FxHashSet<Point>,
    height: usize,
}

impl Default for Chamber {
    fn default() -> Self {
        let mut slf = Self {
            grid: FxHashSet::default(),
            height: 0,
        };
        // Add a floor
        for x in 0..=7 {
            slf.grid.insert(Point { x, y: 0 });
        }
        slf
    }
}

impl Chamber {
    /// Returns true if the collision occurs and adds the rock to the field if insert is true.
    /// Note that this function will move the piece up on the y-axis offset to avoid the overlap.
    fn collision(&mut self, rock: &Rock, insert: bool) -> bool {
        let rock_set = FxHashSet::from_iter(rock.points.iter().map(|p| *p));
        let intersection: Vec<_> = rock_set.intersection(&self.grid).collect();
        if intersection.is_empty() {
            false
        } else {
            if insert {
                for p in rock_set {
                    self.grid.insert(Point { x: p.x, y: p.y + 1 });
                }
            }
            true
        }
    }

    fn jet_collision(&mut self, rock: &Rock, offset: isize) -> bool {
        let rock_set = FxHashSet::from_iter(rock.points.iter().map(|p| Point {
            x: p.x + offset,
            y: p.y + 1,
        }));
        let intersection: Vec<_> = rock_set.intersection(&self.grid).collect();
        !intersection.is_empty()
    }
}

impl fmt::Debug for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (1..=self.height).rev() {
            write!(f, "{y}|").unwrap();
            for x in 0..7 {
                if self.grid.contains(&Point { x, y: y as isize }) {
                    write!(f, "#").unwrap();
                } else {
                    write!(f, ".").unwrap();
                }
            }
            write!(f, "|\n").unwrap();
        }
        write!(f, "0+-------+\n").unwrap();
        Ok(())
    }
}

#[derive(Debug)]
enum JetDirection {
    Right,
    Left,
}

#[derive(Debug)]
struct GasJet {
    pattern: String,
    pattern_position: usize,
}

impl GasJet {
    fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            pattern_position: 0,
        }
    }

    fn get_direction(&self) -> JetDirection {
        match self.pattern.chars().nth(self.pattern_position).unwrap() {
            '>' => Right,
            '<' => Left,
            _ => unreachable!("Unknown stream pattern"),
        }
    }

    fn next(&mut self) {
        self.pattern_position += 1;
        if self.pattern_position == self.pattern.len() {
            self.pattern_position = 0;
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Shape {
    Bar,
    Plus,
    ReverseL,
    Line,
    Square,
}

#[derive(Clone, Debug)]
struct Rock {
    shape: Shape,
    points: Vec<Point>,
}

fn build_shape(shape: Shape) -> Vec<Point> {
    match shape {
        // ####
        Bar => {
            vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ]
        }
        // .#.
        // ###
        // .#.
        Plus => {
            vec![
                Point { x: 0, y: 1 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 2, y: 1 },
            ]
        }
        // ..#
        // ..#
        // ###
        ReverseL => {
            vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 1 },
                Point { x: 2, y: 2 },
            ]
        }
        // #
        // #
        // #
        // #
        Line => {
            vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 3 },
            ]
        }
        // ##
        // ##
        Square => {
            vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
            ]
        }
    }
}

impl Rock {
    fn new() -> Self {
        Self {
            shape: Bar,
            points: build_shape(Bar),
        }
    }

    fn get_height(&self) -> isize {
        match self.shape {
            Bar => 1,
            Plus => 2,
            ReverseL => 3,
            Line => 4,
            Square => 2,
        }
    }

    fn update_shape(&mut self) {
        match self.shape {
            Bar => self.shape = Plus,
            Plus => self.shape = ReverseL,
            ReverseL => self.shape = Line,
            Line => self.shape = Square,
            Square => self.shape = Bar,
        }
        self.points = build_shape(self.shape);
    }

    /// Moves up or down. Collision detection is NOT performed on this operation.
    fn move_vertical(&mut self, amount: isize) {
        self.points.iter_mut().for_each(|p| p.y += amount);
    }

    /// Attempts to shift the object within the playing field based on wall boundaries or other
    /// pieces.
    fn move_horizontal(&mut self, amount: isize, chamber: &mut Chamber) {
        if all(
            self.points
                .iter()
                .map(|p| p.x + amount >= 0 && p.x + amount < 7)
                .collect::<Vec<bool>>(),
            |b| b == true,
        ) {
            if !chamber.jet_collision(&self, amount) {
                // println!("Shifting by {amount}");
                self.points.iter_mut().for_each(|p| p.x += amount);
            } else {
                // println!("Hit another piece");
            }
        } else {
            // println!("out of bounds");
        }
        // println!("{self:?}");
    }
}

impl Iterator for Rock {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = self.clone();
        next.update_shape();
        Some(next)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut chamber = Chamber::default();
    let mut rock = Rock::new();
    let mut gas_jet = GasJet::new(input);

    chamber.height = 3;
    // Set the initial position of our first rock.
    rock.move_vertical(chamber.height as isize);
    rock.move_horizontal(2, &mut chamber);

    let mut total_rocks = 1;
    loop {
        // Bump with a jet
        match gas_jet.get_direction() {
            Right => rock.move_horizontal(1, &mut chamber),
            Left => rock.move_horizontal(-1, &mut chamber),
        }
        gas_jet.next();
        // Now Drop
        if !chamber.collision(&rock, true) {
            rock.move_vertical(-1);
        } else {
            chamber.height = chamber.grid.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y as usize;
            total_rocks += 1;
            if total_rocks > 2_022 {
                break;
            }
            chamber.height += 3;
            rock = rock.next().unwrap();
            // Critical that the piece moves up before horizontal moves to avoid invalid collisions
            rock.move_vertical(chamber.height as isize);
            rock.move_horizontal(2, &mut chamber);
        }
    }

    Some(chamber.height)
}

pub fn part_two(input: &str) -> Option<usize> {
    const GOAL: usize = 1_000_000_000_000;
    let mut chamber = Chamber::default();
    let mut rock = Rock::new();
    let mut gas_jet = GasJet::new(input);

    // Key: Hash of top line contents, jet position and piece type. Value of top line height & rocks
    let mut repeat_hash: HashMap<
        (usize, Shape, usize),
        (usize, usize),
        BuildHasherDefault<FxHasher>,
    > = FxHashMap::default();
    let mut seen_repeats = 0;
    chamber.height = 3;
    // Set the initial position of our first rock.
    rock.move_vertical(chamber.height as isize);
    rock.move_horizontal(2, &mut chamber);

    let mut top_from_repeats = 0;
    let mut total_rocks: usize = 1;
    loop {
        // Bump with a jet
        match gas_jet.get_direction() {
            Right => rock.move_horizontal(1, &mut chamber),
            Left => rock.move_horizontal(-1, &mut chamber),
        }
        gas_jet.next();
        // Now Drop
        if !chamber.collision(&rock, true) {
            rock.move_vertical(-1);
        } else {
            chamber.height = chamber.grid.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y as usize;

            // Bound this to only run a bit after the simulation starts and if we haven't yet found
            // the repeat block.
            if chamber.height > 2000 && seen_repeats < 1 {
                // The top-most line is just a binary representation of the filled squares
                let mut top_line: usize = 0;
                for bit in 0..7 {
                    if chamber.grid.contains(&Point {
                        x: bit,
                        y: chamber.height as isize,
                    }) {
                        top_line |= 1 << bit;
                    }
                }
                // Create a hash of the current piece shape and the jet acting on it and the line
                let hash = (gas_jet.pattern_position, rock.shape, top_line);
                if repeat_hash.contains_key(&hash) && seen_repeats < 1 {
                    let delta_height = chamber.height - repeat_hash.get(&hash).unwrap().0;
                    let delta_rocks = total_rocks - repeat_hash.get(&hash).unwrap().1;
                    let num_to_repeat = (GOAL - total_rocks) / delta_rocks;
                    total_rocks += num_to_repeat * delta_rocks;
                    top_from_repeats += num_to_repeat * delta_height;
                    seen_repeats += 1;
                } else {
                    repeat_hash.insert(hash, (chamber.height, total_rocks));
                }
            }
            total_rocks += 1;
            if total_rocks > 1_000_000_000_000 {
                break;
            }
            chamber.height += 3;
            rock = rock.next().unwrap();
            // Critical that the piece moves up before horizontal moves to avoid invalid collisions
            rock.move_vertical(chamber.height as isize);
            rock.move_horizontal(2, &mut chamber);
        }
    }
    // Now that we have the repeat hash we can just restart the simulation near 1 trillion

    Some(chamber.height + top_from_repeats)
}

fn main() {
    let input = &aoc::read_file("inputs", 17);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::Rev;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
