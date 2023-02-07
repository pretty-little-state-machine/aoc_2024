use crate::Direction::{East, North, South, West};
use pathfinding::prelude::dijkstra_all;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Default, Debug, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, Hash)]
struct Room {
    north: Option<Point>,
    east: Option<Point>,
    south: Option<Point>,
    west: Option<Point>,
}

impl Room {
    fn successors(&self) -> Vec<(Point, usize)> {
        let mut successors = Vec::new();
        if let Some(point) = self.north {
            successors.push((point, 1));
        }
        if let Some(point) = self.east {
            successors.push((point, 1));
        }
        if let Some(point) = self.south {
            successors.push((point, 1));
        }
        if let Some(point) = self.west {
            successors.push((point, 1));
        }
        successors
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
struct Building {
    rooms: FxHashMap<Point, Room>,
}

impl Building {
    fn add_room(&mut self, position: &Point) {
        if !self.rooms.contains_key(position) {
            self.rooms.insert(*position, Room::default());
        }
    }

    /// Adds neighbor to a room and returns the new neighbors position
    fn add_neighbor(&mut self, position: &Point, direction: Direction) -> Point {
        let current_room = self.rooms.get_mut(position).unwrap();
        match direction {
            North => {
                let point = Point {
                    x: position.x,
                    y: position.y - 1,
                };
                current_room.north = Some(point);
                self.add_room(&point);
                self.rooms.get_mut(&point).unwrap().south = Some(*position);
                point
            }
            East => {
                let point = Point {
                    x: position.x + 1,
                    y: position.y,
                };
                current_room.east = Some(point);
                self.add_room(&point);
                self.rooms.get_mut(&point).unwrap().west = Some(*position);
                point
            }
            South => {
                let point = Point {
                    x: position.x,
                    y: position.y + 1,
                };
                current_room.south = Some(point);
                self.add_room(&point);
                self.rooms.get_mut(&point).unwrap().north = Some(*position);
                point
            }
            West => {
                let point = Point {
                    x: position.x - 1,
                    y: position.y,
                };
                current_room.west = Some(point);
                self.add_room(&point);
                self.rooms.get_mut(&point).unwrap().east = Some(*position);
                point
            }
        }
    }
}

/// Use a stack to construct a building from the regex
fn build_building(building_string: &str) -> Building {
    let mut building = Building::default();
    let mut position = Point { x: 0, y: 0 };
    let mut position_stack = VecDeque::new();
    building.add_room(&position);

    for c in building_string.chars() {
        position = match c {
            '^' => continue,
            '$' => break,
            'N' => building.add_neighbor(&position, North),
            'E' => building.add_neighbor(&position, East),
            'S' => building.add_neighbor(&position, South),
            'W' => building.add_neighbor(&position, West),
            '(' => {
                position_stack.push_back(position);
                position
            }
            ')' => position_stack.pop_back().unwrap(),
            '|' => *position_stack.back().unwrap(),
            _ => unreachable!("Unknown Regex character!"),
        };
    }
    building
}

/// Find the farthest room by door count
pub fn part_one(input: &str) -> Option<usize> {
    let building = build_building(input);
    let paths = dijkstra_all(&Point::default(), |p| {
        building.rooms.get(p).unwrap().successors()
    });
    Some(*paths.values().map(|(_path, cost)| cost).max().unwrap())
}

/// Find all rooms that have at least 1000 doors to get to them
pub fn part_two(input: &str) -> Option<usize> {
    let building = build_building(input);
    let paths = dijkstra_all(&Point::default(), |p| {
        building.rooms.get(p).unwrap().successors()
    });
    Some(
        paths
            .values()
            .map(|(_path, cost)| cost)
            .filter(|&cost| *cost >= 1000)
            .count(),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 20);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(
            part_one("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"),
            Some(23)
        );
        assert_eq!(
            part_one("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"),
            Some(31)
        );
        assert_eq!(
            part_one("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"),
            Some(18)
        )
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(0));
    }
}
