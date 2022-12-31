use std::collections::HashSet;
use crate::JetDirection::{Left, Right};
use crate::Shape::{Bar, Line, Plus, ReverseL, Square};
use itertools::{all, any};
use rustc_hash::FxHashSet;
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Ord, PartialOrd, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

struct Chamber {
    grid: FxHashSet<Point>,
    height: isize,
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
        let rock_set = FxHashSet::from_iter(rock.points.iter().map(|p| Point { x: p.x + offset, y: p.y + 1 }));
        let intersection: Vec<_> = rock_set.intersection(&self.grid).collect();
        !intersection.is_empty()
    }
}

impl fmt::Debug for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (1..10).rev() {
            write!(f, "{y}|").unwrap();
            for x in 0..7 {
                if self.grid.contains(&Point { x, y }) {
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

#[derive(Copy, Clone, Debug)]
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

pub fn part_one(input: &str) -> Option<isize> {
    let mut chamber = Chamber::default();
    let mut rock = Rock::new();
    let mut gas_jet = GasJet::new(input);

    chamber.height = 3;
    // Set the initial position of our first rock.
    rock.move_vertical(chamber.height);
    rock.move_horizontal(2, &mut chamber);

    let mut total_rocks = 1;
    loop {
        // Bump with a jet
        match gas_jet.get_direction() {
            Right => rock.move_horizontal(1, &mut chamber),
            Left => rock.move_horizontal(-1, &mut chamber),
        }
        gas_jet.next();
        // Drop Down. If this is a collision we stop.

        if chamber.collision(&rock, true) {
            // println!("Collision!");
            total_rocks += 1;
            if total_rocks == 2_022 {
                break;
            }
            // Update the chamber height, make a new piece and offset it appropriately
            chamber.height = chamber.grid.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y + 3;
            // println!("{chamber:?}");
            // println!();
            // println!("made a new rock @ y = {}!", chamber.height);
            rock = rock.next().unwrap();
            // Critical that the piece moves up before horizontal moves to avoid invalid collisions
            rock.move_vertical(chamber.height);
            rock.move_horizontal(2, &mut chamber);
        } else {
            rock.move_vertical(-1);
        }
    }

    Some(chamber.height + 1)
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
    use std::iter::Rev;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
