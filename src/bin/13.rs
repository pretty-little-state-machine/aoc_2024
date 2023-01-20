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
                        facing: North,
                        next_action: TurnLeft,
                        position: Point { x, y },
                        crashed: false,
                    });
                }
                'v' => {
                    tracks.insert(Point { x, y }, Vertical);
                    trains.push(Train {
                        facing: South,
                        next_action: TurnLeft,
                        position: Point { x, y },
                        crashed: false,
                    });
                }
                '>' => {
                    tracks.insert(Point { x, y }, Horizontal);
                    trains.push(Train {
                        facing: East,
                        next_action: TurnLeft,
                        position: Point { x, y },
                        crashed: false,
                    });
                }
                '<' => {
                    tracks.insert(Point { x, y }, Horizontal);
                    trains.push(Train {
                        facing: West,
                        next_action: TurnLeft,
                        position: Point { x, y },
                        crashed: false,
                    });
                }
                ' ' => {}
                _ => unreachable!("Invalid character for map"),
            }
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
    facing: Direction,
    next_action: Action,
    position: Point,
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
    fn tick(&mut self) -> Point {
        match &self.facing {
            North => self.position.y -= 1,
            South => self.position.y += 1,
            East => self.position.x += 1,
            West => self.position.x -= 1,
        }
        self.position
    }

    fn update_facing(&mut self, tracks: &Tracks) {
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
        } else {
            match (current_track, &self.facing) {
                (Horizontal, East) | (Intersection, East) => (),
                (Horizontal, West) | (Intersection, West) => (),
                (Vertical, North) | (Intersection, North) => (),
                (Vertical, South) | (Intersection, South) => (),
                // `/` Corners: N->E, E->N, S->W, W->S
                (CornerNWSE, North) => self.facing = East,
                (CornerNWSE, East) => self.facing = North,
                (CornerNWSE, West) => self.facing = South,
                (CornerNWSE, South) => self.facing = West,
                // `\` Corners: E->S, S->E, N->W, W->N
                (CornerNESW, South) => self.facing = East,
                (CornerNESW, East) => self.facing = South,
                (CornerNESW, North) => self.facing = West,
                (CornerNESW, West) => self.facing = North,
                _ => unreachable!(
                    "Invalid train / track movement combination: {:?}, {:?}",
                    current_track, &self.facing
                ),
            };
        }
    }
}

/// Trains are simulated per row per the problem statement.
fn tick(trains: &mut Trains, tracks: &Tracks, remove_trains: bool) -> Option<Point> {
    trains.sort_unstable_by_key(|t| (t.position.x, t.position.y));
    // let mut seen_pos: Vec<Point> = Vec::new();
    for i in 0..trains.len() {
        trains[i].tick();

        // We must do collision detection after moving EVERY train. Otherwise we might skip one
        let mut overlaps = trains.iter().map(|t| t.position).counts();
        overlaps.retain(|_, c| *c >= 2);
        let mut removals = overlaps.keys().collect::<Vec<&Point>>();
        while let Some(&point) = removals.pop() {
            if !remove_trains {
                return Some(point);
            } else {
                for train in &mut *trains {
                    if train.position == point && !train.crashed {
                        train.crashed = true;
                    }
                }
            }
        }
    }
    trains.retain(|t| !t.crashed);
    if trains.len() == 1 {
        return Some(trains[0].position);
    }
    trains.iter_mut().for_each(|t| t.update_facing(tracks));
    /*
     for train in &mut *trains {
         println!("{}, {} :: {:?} ", train.position.x, train.position.y, tracks.get(&train.position).unwrap());
     }
    */
    // println!();
    // for train in &mut *trains {
    //  println!("{}, {}", train.position.x, train.position.y);
    // }
    None
}

pub fn part_one(input: &str) -> Option<String> {
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
