use crate::JetDirection::{Left, Right};
use crate::RockShape::{Bar, Line, Plus, ReverseL, Square};
use std::fmt;

struct Chamber {
    grid: Vec<u8>,
}

impl Default for Chamber {
    fn default() -> Self {
        Self {
            grid: Vec::with_capacity(20_000),
        }
    }
}

impl Chamber {
    fn add_rock(&mut self, rock: &Rock, bottom_rock_line: usize, shift: isize) {
        for row in 0..4 {
            let rock_byte = isize_shift(rock.get_row_bitmask(row), shift);
            let grid_byte = self.grid.get_mut(bottom_rock_line + row).unwrap();
            println!(
                "Shift: {} Rock: {:08b},  Grid: {:08b}",
                shift, rock_byte, grid_byte
            );
            *grid_byte |= rock_byte;
        }
    }
}

impl fmt::Debug for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.grid.iter().rev() {
            write!(f, "|").unwrap();
            for bit in 0..7 {
                if 1 == (line >> bit) & 0x1 {
                    write!(f, "#").unwrap();
                } else {
                    write!(f, ".").unwrap();
                }
            }
            write!(f, "|\n").unwrap();
        }
        Ok(())
    }
}

/// A block shape. Blocks are encoded as a u16 with 4 bits representing a row bottom to top.
#[repr(u32)]
#[derive(Copy, Clone, Debug)]
enum RockShape {
    // ........
    // ........
    // ........
    // ..####..
    Bar = 0b00000000_00000000_00000000_00111100,
    // ........
    // ...#....
    // ..###...
    // ...#....
    Plus = 0b00000000_00010000_00111000_00010000,
    // ........
    // ....#...
    // ....#...
    // ..###...
    ReverseL = 0b00000000_00001000_00001000_00111000,
    // ..#.....
    // ..#.....
    // ..#.....
    // ..#.....
    Line = 0b00100000_00100000_00100000_00100000,
    // ........
    // ........
    // ..##....
    // ..##....
    Square = 0b00000000_00000000_00110000_00110000,
}

fn isize_shift(byte: u8, shift: isize) -> u8 {
    if shift > 0 {
        byte >> shift
    } else if shift < 0 {
        ((byte as u16) << -shift) as u8
    } else {
        byte
    }
}

#[derive(Clone, Debug)]
struct Rock {
    shape: RockShape,
}

impl Rock {
    fn new() -> Self {
        Self { shape: Bar }
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

    fn get_row_bitmask(&self, row: usize) -> u8 {
        (((self.shape as u32) >> (row * 8)) & 0xFF) as u8
    }

    /// Returns true if any row in the block would hit a wall
    fn would_hit_side(&self, shift: isize) -> bool {
        for row in 0..3 {
            let test_byte = isize_shift(self.get_row_bitmask(row), shift);
            //println!("Test byte with shift: {} {:08b}", shift, test_byte);
            // Hit the left wall
            if 1 == (test_byte >> 7) & 0x1 {
                return true;
            }
            // Hit the right wall, remember the walls are 7 bits wide, not 8 so we must shift
            if 1 == (test_byte >> 1) & 0x1 {
                return true;
            }
        }
        false
    }
}

impl Iterator for Rock {
    type Item = Rock;

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

pub fn part_one(input: &str) -> Option<u32> {
    let mut chamber = Chamber::default();
    let mut rock = Rock::new();
    let mut gas_jet = GasJet::new(input);

    let mut total_rocks: usize = 0;
    let mut current_floor_height: usize = 0;
    let mut rock_bottom_layer_height: usize = 3;
    let mut shift: isize = 0;
    // Spawn in a block
    for _ in 0..5 {
        chamber.grid.push(0);
    }
    let mut i = 0;
    while i <= 20 {
        i += 1;
        // Shift the collision mask via the jet with edge collision detection
        println!("Jet Pushes {:?}", gas_jet.get_direction());
        match gas_jet.get_direction() {
            Right => {
                if !rock.would_hit_side(shift - 1) {
                    shift -= 1;
                    println!("Shifted rock right, shift: {}", shift);
                } else {
                    println!("Rock can't go further right");
                }
            }
            Left => {
                if !rock.would_hit_side(shift + 1) {
                    shift += 1;
                    println!("Shifted rock left, shift: {}", shift);
                } else {
                    println!("Rock can't go further left");
                }
            }
        }
        gas_jet.next();

        // Rock collision on the grid
        if rock_bottom_layer_height == 0
            || isize_shift(rock.get_row_bitmask(0), shift)
                & chamber.grid.get(rock_bottom_layer_height - 1).unwrap()
                > 0
        {
            println!("Collision!");
            chamber.add_rock(&rock, rock_bottom_layer_height, shift);
            current_floor_height += rock.get_height() - 1;
            // Prepare for the next round
            println!("NEW STONE");
            total_rocks += 1;
            rock = rock.next().unwrap();
            rock_bottom_layer_height = current_floor_height + 4;
            shift = 0;
            for _ in 0..4 {
                chamber.grid.push(0);
            }
        }
        // Drop down a tile
        println!("Stone is falling: {}", rock_bottom_layer_height);
        rock_bottom_layer_height -= 1;
        println!("{:?}", chamber);
    }
    println!("{:?}", chamber);

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
