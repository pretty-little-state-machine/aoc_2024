use std::cmp;
use std::cmp::Ordering;
use crate::Action::{Straight, TurnLeft, TurnRight};
use crate::Direction::{East, North, South, West};
use crate::Track::{CornerNESW, CornerNWSE, Horizontal, Intersection, Vertical};
use itertools::Itertools;
use rustc_hash::FxHashMap;

type Tracks = FxHashMap<Point, Track>;
type Trains = Vec<Train>;

fn parse_input(input: &str) -> (Tracks, Trains) {
    let mut tracks = FxHashMap::default();
    let mut trains = Vec::default();
    let mut id: isize = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '|' => {
                    tracks.insert(Point { x, y }, Vertical);
                }
                '-' => {
                    tracks.insert(Point { x, y }, Horizontal);
                }
                '/' => {
                    tracks.insert(Point { x, y }, CornerNWSE);
                }
                '\\' => {
                    tracks.insert(Point { x, y }, CornerNESW);
                }
                '+' => {
                    tracks.insert(Point { x, y }, Intersection);
                }
                '^' => {
                    tracks.insert(Point { x, y }, Vertical);
                    trains.push(Train {
                        id,
                        facing: North,
                        next_action: TurnLeft,
                        position: Point { x, y },
                        may_move: true,
                        crashed: false,
                    });
                }
                'v' => {
                    tracks.insert(Point { x, y }, Vertical);
                    trains.push(Train {
                        id,
                        facing: South,
                        next_action: TurnLeft,
                        position: Point { x, y },
                        may_move: true,
                        crashed: false,
                    });
                }
                '>' => {
                    tracks.insert(Point { x, y }, Horizontal);
                    trains.push(Train {
                        id,
                        facing: East,
                        next_action: TurnLeft,
                        position: Point { x, y },
                        may_move: true,
                        crashed: false,
                    });
                }
                '<' => {
                    tracks.insert(Point { x, y }, Horizontal);
                    trains.push(Train {
                        id,
                        facing: West,
                        next_action: TurnLeft,
                        position: Point { x, y },
                        may_move: true,
                        crashed: false,
                    });
                }
                ' ' => {}
                _ => unreachable!("Invalid character for map"),
            }
            id += 1;
        }
    }
    (tracks, trains)
}

/// A Point.
/// WARNING: The Y Coordinate is first so we can use the built in CMP to order by y then x
#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    y: usize,
    x: usize,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Debug, Copy, Clone)]
enum Action {
    TurnLeft,
    Straight,
    TurnRight,
}

#[derive(Debug, Copy, Clone)]
struct Train {
    id: isize,
    facing: Direction,
    next_action: Action,
    position: Point,
    may_move: bool,
    crashed: bool,
}

#[derive(Debug, Eq, PartialEq)]
enum Track {
    Horizontal = '-' as isize,
    Vertical = '|' as isize,
    CornerNWSE = '/' as isize,
    CornerNESW = '\\' as isize,
    Intersection = '+' as isize,
}

impl Train {
    fn tick(&mut self, tracks: &Tracks) -> Point {
        self.may_move = false;
        let current_track = tracks.get(&self.position).expect("Track isn't contiguous!");
        // Trains entering an intersection adjust their facing direction
        if Intersection == *tracks.get(&self.position).unwrap() {
            self.facing = match (&self.next_action, &self.facing) {
                (TurnLeft, North) => {
                    self.next_action = Straight;
                    West
                }
                (TurnLeft, West) => {
                    self.next_action = Straight;
                    South
                }
                (TurnLeft, South) => {
                    self.next_action = Straight;
                    East
                }
                (TurnLeft, East) => {
                    self.next_action = Straight;
                    North
                }
                (TurnRight, North) => {
                    self.next_action = TurnLeft;
                    East
                }
                (TurnRight, East) => {
                    self.next_action = TurnLeft;
                    South
                }
                (TurnRight, South) => {
                    self.next_action = TurnLeft;
                    West
                }
                (TurnRight, West) => {
                    self.next_action = TurnLeft;
                    North
                }
                (Straight, _) => {
                    self.next_action = TurnRight;
                    self.facing
                }
            }
        }
        match (current_track, &self.facing) {
            (Horizontal, East) | (Intersection, East) => self.position.x += 1,
            (Horizontal, West) | (Intersection, West) => self.position.x -= 1,
            (Vertical, North) | (Intersection, North) => self.position.y -= 1,
            (Vertical, South) | (Intersection, South) => self.position.y += 1,
            // `/` Corners: N->E, E->N, S->W, W->S
            (CornerNWSE, North) => {
                self.facing = East;
                self.position.x += 1;
            }
            (CornerNWSE, East) => {
                self.facing = North;
                self.position.y -= 1;
            }
            (CornerNWSE, West) => {
                self.facing = South;
                self.position.y += 1;
            }
            (CornerNWSE, South) => {
                self.facing = West;
                self.position.x -= 1;
            }
            // `\` Corners: E->S, S->E, N->W, W->N
            (CornerNESW, South) => {
                self.facing = East;
                self.position.x += 1;
            }
            (CornerNESW, East) => {
                self.facing = South;
                self.position.y += 1;
            }
            (CornerNESW, North) => {
                self.facing = West;
                self.position.x -= 1;
            }
            (CornerNESW, West) => {
                self.facing = North;
                self.position.y -= 1;
            }
            _ => unreachable!(
                "Invalid train / track movement combination: {:?}, {:?}",
                current_track, &self.facing
            ),
        };

        }
        self.position
    }
}

/// Trains are simulated per row per the problem statement.
fn tick(trains: &mut Trains, tracks: &Tracks, remove_trains: bool) -> Option<Point> {
    trains.sort_unstable_by_key(|t| (t.position.x, t.position.y));

    let mut seen_pos: Vec<Point> = Vec::new();
    for train in &mut *trains {
        seen_pos.push(train.tick(tracks))
    }

    let mut overlaps = seen_pos.iter().counts();
    overlaps.retain(|_, c| *c >= 2);
    for overlap in overlaps.keys() {
        if !remove_trains {
            return Some(**overlap);
        } else {
            // Only allow up to two trains per position to be crashed
            let mut crashed_trains: usize = 0;
            while crashed_trains < 2 {
                for train in &mut *trains {
                    if train.position == **overlap {
                        train.crashed = true;
                        println!("Crashed Train: {} @ {:?}", train.id, train.position);
                        crashed_trains += 1;
                    }
                }
            }

            for train in &mut *trains {
                println!("{},{}", train.position.x, train.position.y);
            }
            println!();
        }
    }
    trains.retain(|t| !t.crashed);
    if trains.len() == 1 {
        return Some(trains[0].position);
    }
    trains.iter_mut().for_each(|t| t.may_move = true);
    // println!();
   // for train in &mut *trains {
     //  println!("{}, {}", train.position.x, train.position.y);
    // }
    None
}

pub fn part_one(input: &str) -> Option<String> {
    return None;
    let (tracks, mut trains) = parse_input(input);
    loop {
        match tick(&mut trains, &tracks, false) {
            None => {}
            Some(point) => return Some(format!("{},{}", point.x, point.y)),
        }
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let (tracks, mut trains) = parse_input(input);
    loop {
        match tick(&mut trains, &tracks, true) {
            None => {}
            Some(point) => return Some(format!("{},{}", point.x, point.y)),
        }
    }
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_one(&input), Some("7,3".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_two(&input), Some("6,4".to_string()));
    }
}
