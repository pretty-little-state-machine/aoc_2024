use crate::num_traits::FromPrimitive;
use crate::Acre::{Lumberyard, Open, Wooded};
use rustc_hash::FxHashMap;
use std::cmp::Ordering;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

#[derive(Debug, Copy, Clone, Primitive, Eq, PartialEq, Hash)]
enum Acre {
    Lumberyard = 35, // #
    Open = 46,       // .
    Wooded = 124,    // |
}

#[derive(Debug, Clone)]
struct Land {
    grid: Vec<Vec<Acre>>,
}

impl Land {
    fn new(input: &str) -> Self {
        let mut grid: Vec<Vec<Acre>> = Vec::new();
        for y in input.lines() {
            let mut row = Vec::new();
            for x in y.bytes() {
                let acre = Acre::from_u8(x).unwrap();
                row.push(acre);
            }
            grid.push(row);
        }
        Self { grid }
    }

    /// Returns the number of cells filled with whichever values
    fn count_neighbors(grid: &[Vec<Acre>], x: isize, y: isize) -> FxHashMap<Acre, usize> {
        let mut neighbors = FxHashMap::default();
        neighbors.insert(Open, 0);
        neighbors.insert(Wooded, 0);
        neighbors.insert(Lumberyard, 0);
        for dy in -1..=1 {
            for dx in -1..=1 {
                if (dx == 0 && dy == 0) || (x == 0 && dx == -1) || (y == 0 && dy == -1) {
                    continue;
                }
                if let Some(row) = grid.get((y + dy) as usize) {
                    if let Some(acre) = row.get((x + dx) as usize) {
                        *neighbors.get_mut(acre).unwrap() += 1;
                    }
                }
            }
        }
        neighbors
    }

    fn tick(&mut self) {
        // The puzzle states that the grid must not change until all cells are computed.
        let grid_snapshot = self.grid.clone();
        for y in 0..grid_snapshot.len() {
            for x in 0..grid_snapshot[0].len() {
                let neighbors = Self::count_neighbors(&grid_snapshot, x as isize, y as isize);
                let new_acre = match grid_snapshot[y][x] {
                    Open => {
                        if neighbors.get(&Wooded).unwrap() >= &3 {
                            Wooded
                        } else {
                            Open
                        }
                    }
                    Wooded => {
                        if neighbors.get(&Lumberyard).unwrap() >= &3 {
                            Lumberyard
                        } else {
                            Wooded
                        }
                    }
                    Lumberyard => {
                        if neighbors.get(&Lumberyard).unwrap() >= &1
                            && neighbors.get(&Wooded).unwrap() >= &1
                        {
                            Lumberyard
                        } else {
                            Open
                        }
                    }
                };
                self.grid[y][x] = new_acre;
            }
        }
    }

    fn count_acres(&self) -> FxHashMap<Acre, usize> {
        let mut acres = FxHashMap::default();
        acres.insert(Open, 0);
        acres.insert(Wooded, 0);
        acres.insert(Lumberyard, 0);
        for row in &self.grid {
            for acre in row {
                *acres.get_mut(acre).unwrap() += 1
            }
        }
        acres
    }
    #[allow(dead_code)]
    fn debug(&self) {
        for row in &self.grid {
            for acre in row {
                match acre {
                    Lumberyard => print!("#"),
                    Open => print!("."),
                    Wooded => print!("|"),
                }
            }
            println!()
        }
        println!()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut land = Land::new(input);
    for _ in 1..=10 {
        land.tick();
    }
    let counts = land.count_acres();
    Some(counts.get(&Lumberyard).unwrap() * counts.get(&Wooded).unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    const REPEAT_BUFFER: usize = 500;
    let mut land = Land::new(input);
    let mut count_cursor = FxHashMap::default();
    // The sequence starts repeating, we must capture the repetition length.
    let mut sequence_length = 0;
    for x in 1..=700 {
        land.tick();
        match x.cmp(&REPEAT_BUFFER) {
            Ordering::Equal => count_cursor = land.count_acres(),
            Ordering::Greater => {
                if count_cursor == land.count_acres() {
                    sequence_length = x - REPEAT_BUFFER;
                    break;
                }
            }
            _ => (),
        }
    }
    // Jump forwards in time
    for _ in 0..((1_000_000_000 - (REPEAT_BUFFER - sequence_length)) % sequence_length) {
        land.tick();
    }
    let counts = land.count_acres();
    Some(counts.get(&Lumberyard).unwrap() * counts.get(&Wooded).unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 18);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(1147));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 18);
        assert_eq!(part_two(&input), None);
    }
}
