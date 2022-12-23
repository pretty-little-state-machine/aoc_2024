use rustc_hash::{FxHashMap, FxHashSet};
use crate::Direction::{North, South, East, West};

// A point is in (x, y) coordinates
type Point = (isize, isize);
type Elves = FxHashSet<Point>;

fn parse_input(input: &str) -> Elves {
    let mut elves = Elves::default();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.insert((x as isize, y as isize));
            }
        }
    }
    elves
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn next_direction( direction: Direction) -> Direction {
    match direction {
        North => South,
        South => West,
        West => East,
        East => North,
    }
}

/// Returns a HashMap of Elves at a point and the point they want to go to
fn get_wanted_moves(elves: &Elves, start_direction: Direction) -> FxHashMap<Point, Point> {
    let mut wanted_directions = FxHashMap::default();
    for elf in elves {
        let mut attempted_direction = start_direction;
        for _ in 0..4 {
            match attempted_direction {
                North => {
                    if !elves.contains(&(elf.0, elf.1 - 1))
                        && !elves.contains(&(elf.0 - 1, elf.1 - 1))
                        && !elves.contains(&(elf.0 + 1, elf.1 - 1)) {
                        wanted_directions.insert(*elf, (elf.0, elf.1 - 1));
                        break;
                    }
                }
                South => {
                    if !elves.contains(&(elf.0, elf.1 + 1))
                        && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                        && !elves.contains(&(elf.0 + 1, elf.1 + 1)) {
                        wanted_directions.insert(*elf, (elf.0, elf.1 + 1));
                        break;
                    }
                }
                West => {
                    if !elves.contains(&(elf.0 - 1, elf.1 ))
                        && !elves.contains(&(elf.0 - 1, elf.1 + 1))
                        && !elves.contains(&(elf.0 - 1, elf.1 - 1)) {
                        wanted_directions.insert(*elf, (elf.0 - 1, elf.1));
                        break;
                    }
                }
                East => {
                    if !elves.contains(&(elf.0 + 1, elf.1 ))
                        && !elves.contains(&(elf.0 + 1, elf.1 + 1))
                        && !elves.contains(&(elf.0 + 1, elf.1 - 1)) {
                        wanted_directions.insert(*elf, (elf.0 + 1, elf.1));
                        break;
                    }
                }
            }
            attempted_direction = next_direction(attempted_direction);
        }
    }
    wanted_directions
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut elves = parse_input(input);
    let mut start_direction = North;
    let wanted_moves = get_wanted_moves(&elves, start_direction);
    println!("{:?}", elves);
    println!("{:?}", wanted_moves);


    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 23);
        assert_eq!(part_two(&input), None);
    }
}
