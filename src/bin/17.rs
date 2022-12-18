use std::fmt;
use crate::BlockShape::{Bar, Line, Plus, ReverseL, Square};
use crate::JetDirection::{Left, Right};
use rustc_hash::FxHashSet;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Chamber {
    grid: Vec<u8>,
}

impl Default for Chamber {
    fn default() -> Self {
        Self { grid: Vec::with_capacity(20_000) }
    }
}

impl fmt::Debug for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.grid.iter().rev() {
            for bit in 0..7 {
                if (line >> bit) && 0x1 {
                    write!(f, "#")
                }
            }
        }
    }
}

/// A block shape. Blocks are encoded as a u16 with 4 bits representing a row bottom to top.
#[repr(u16)]
#[derive(Copy, Clone, Debug)]
enum BlockShape {
    // ....
    // ....
    // ....
    // ####
    Bar = 0b0000_0000_0000_1111,
    // ....
    // .#..
    // ###.
    // .#..
    Plus = 0b0000_0100_1110_0100,
    // ....
    // ..#.
    // ..#.
    // ###.
    ReverseL = 0b0000_0010_0010_1110,
    // #...
    // #...
    // #...
    // #...
    Line = 0b1000_1000_1000_1000,
    // ....
    // ....
    // ##..
    // ##..
    Square = 0b0000_0000_1100_1100,
}

#[derive(Clone, Debug)]
struct Block {
    shape: BlockShape,
}

impl Block {
    fn new() -> Self {
        Self {
            shape: Line,
        }
    }

    fn get_height(&self) -> usize {
        match self.shape {
            Bar => 1,
            Plus => 3,
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
    }
}

impl Iterator for Block {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        self.update_shape();
        Some(self.clone())
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

fn is_collision(block: &Block, chamber: &Chamber, offset: &Point) -> bool {
    for block_point in block.offsets {
        let low_check = offset.y.saturating_add(block_point.y);
        if low_check == 0 {
            return true;
        }
        let side_shift = offset.x.saturating_add(block_point.x).clamp(0, 7 - block_point.x);
        // TODO: Block to already filled blocks positions
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let chamber = Chamber::default();
    let mut block = Block::new();
    let mut gas_jet = GasJet::new(input);

    let mut total_rocks: usize = 0;
    let current_floor_height: usize = 0;

    let mut block_offset = Point { x: 2, y: (current_floor_height + 2) as isize };
    while total_rocks <= 20 {
        while !is_collision(&block, &chamber, &block_offset) {
            match gas_jet.get_direction() {
                Right => block_offset.x += 1,
                Left => block_offset.x -= 1,
            }
            block_offset -= 1;
        }

        gas_jet.next();
        block.next();
    }

    Some(0)
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
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
