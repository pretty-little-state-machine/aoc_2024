use crate::Direction::{East, North, South, West};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

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

fn next_direction(direction: Direction) -> Direction {
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
        let (n, ne, e, se, s, sw, w, nw) = (
            &(elf.0, elf.1 - 1),     // N
            &(elf.0 + 1, elf.1 - 1), // NE
            &(elf.0 + 1, elf.1),     // E
            &(elf.0 + 1, elf.1 + 1), // SE
            &(elf.0, elf.1 + 1),     // S
            &(elf.0 - 1, elf.1 + 1), // SW
            &(elf.0 - 1, elf.1),     // W
            &(elf.0 - 1, elf.1 - 1), // NW
        );
        // If all directions are free, then the elf won't bother moving.
        if !elves.contains(n)
            && !elves.contains(ne)
            && !elves.contains(e)
            && !elves.contains(se)
            && !elves.contains(s)
            && !elves.contains(sw)
            && !elves.contains(w)
            && !elves.contains(nw)
        {
            // wanted_directions.insert(*elf, (elf.0, elf.1));
        } else {
            let mut attempted_direction = start_direction;
            for _ in 0..4 {
                match attempted_direction {
                    North => {
                        if !elves.contains(n) && !elves.contains(ne) && !elves.contains(nw) {
                            wanted_directions.insert(*elf, *n);
                            break;
                        }
                    }
                    South => {
                        if !elves.contains(s) && !elves.contains(se) && !elves.contains(sw) {
                            wanted_directions.insert(*elf, *s);
                            break;
                        }
                    }
                    West => {
                        if !elves.contains(w) && !elves.contains(sw) && !elves.contains(nw) {
                            wanted_directions.insert(*elf, *w);
                            break;
                        }
                    }
                    East => {
                        if !elves.contains(e) && !elves.contains(se) && !elves.contains(ne) {
                            wanted_directions.insert(*elf, *e);
                            break;
                        }
                    }
                }
                attempted_direction = next_direction(attempted_direction);
            }
        }
    }
    wanted_directions
}

/// Remove all wanted moves that will enter the same Point
fn filter_duplicate_moves(wanted_moves: &mut FxHashMap<Point, Point>) {
    let mut frequencies: FxHashMap<Point, usize> = FxHashMap::default();
    wanted_moves.values().for_each(|point| {
        if let Some(count) = frequencies.get_mut(point) {
            *count += 1;
        } else {
            frequencies.insert(*point, 1);
        }
    });
    frequencies.retain(|_, v| *v > 1);
    wanted_moves.retain(|_, v| !frequencies.contains_key(v));
}

fn execute_moves(elves: &Elves, wanted_moves: &FxHashMap<Point, Point>) -> Elves {
    let mut new_elves = Elves::default();
    for elf in elves {
        if let Some(new_elf) = wanted_moves.get(elf) {
            new_elves.insert(*new_elf);
        } else {
            new_elves.insert(*elf);
        }
    }
    new_elves
}

fn bounds(elves: &Elves) -> (Point, Point) {
    let x_vals = elves
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .map(|p| p.0)
        .collect::<Vec<isize>>();
    let y_vals = elves
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .map(|p| p.1)
        .collect::<Vec<isize>>();
    (
        (*x_vals.first().unwrap(), *y_vals.first().unwrap()),
        (*x_vals.last().unwrap(), *y_vals.last().unwrap()),
    )
}

fn score_elves(elves: &Elves) -> isize {
    let (min, max) = bounds(elves);
    (max.0 - min.0 + 1) * (max.1 - min.1 + 1) - elves.len() as isize
}

fn debug(elves: &Elves) {
    let (min, max) = bounds(elves);
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if elves.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut elves = parse_input(input);
    let mut direction = North;
    for _ in 1..=10 {
        let mut wanted_moves = get_wanted_moves(&elves, direction);
        filter_duplicate_moves(&mut wanted_moves);
        elves = execute_moves(&elves, &wanted_moves);
        direction = next_direction(direction);
    }
    Some(score_elves(&elves))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut elves = parse_input(input);
    let mut direction = North;
    let mut rounds: usize = 1;
    while rounds < 100_000 {
        let mut wanted_moves = get_wanted_moves(&elves, direction);
        filter_duplicate_moves(&mut wanted_moves);
        elves = execute_moves(&elves, &wanted_moves);
        direction = next_direction(direction);
        if wanted_moves.is_empty() {
            break;
        }
        rounds += 1;
    }
    Some(rounds)
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
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
