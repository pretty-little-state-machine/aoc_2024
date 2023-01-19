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

#[derive(Debug, Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
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
    fn tick(&mut self, position: &Point, tracks: &Tracks) -> Point {
        self.may_move = false;
        let current_track = tracks.get(position).expect("Track isn't contiguous!");
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
        self.position
    }
}

fn get_bounds(tracks: &Tracks) -> (usize, usize) {
    (
        tracks.keys().max_by_key(|k| k.x).unwrap().x,
        tracks.keys().max_by_key(|k| k.y).unwrap().y,
    )
}

/// Trains are simulated per row per the problem statement.
fn tick(
    trains: &mut Trains,
    tracks: &Tracks,
    bounds: (usize, usize),
    remove_trains: bool,
) -> Option<Point> {
    let mut new_positions: FxHashMap<Point, Vec<isize>> = FxHashMap::default();
    for y in 0..=bounds.1 {
        for x in 0..=bounds.0 {
            for train in &mut *trains {
                if (Point { x, y }) == train.position && train.may_move && !train.crashed {
                    let new_pos = train.tick(&Point { x, y }, tracks);
                    if new_positions.contains_key(&new_pos) {
                        if new_positions.get(&new_pos).unwrap().len() < 2 {
                            new_positions.get_mut(&new_pos).unwrap().push(train.id)
                        }
                    } else {
                        new_positions.insert(new_pos, vec![train.id]);
                    }
                }
            }
        }
    }
    // Allow next tick for all trains
    trains.iter_mut().for_each(|t| t.may_move = true);

    // Collision detection will only remove up to two trains
    for (point, possible_trains) in new_positions {
        if possible_trains.len() > 1 {
            if remove_trains {
                for train in trains.iter_mut() {
                    if possible_trains.contains(&train.id) {
                        println!("crashed train: {:?} @ {:?}", train.id, point);
                        train.crashed = true;
                    }
                }
                println!();
                let mut retained = trains.clone();
                retained.retain(|t| !t.crashed);

                println!("{} trains remain", retained.len());
                if retained.len() == 1 {
                    return Some(retained[0].position);
                }
            } else {
                for train in trains.iter_mut() {
                    if train.id == possible_trains[0] {
                        return Some(train.position);
                    }
                }
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<String> {
    let (tracks, mut trains) = parse_input(input);
    let bounds = get_bounds(&tracks);
    loop {
        match tick(&mut trains, &tracks, bounds, false) {
            None => {}
            Some(point) => return Some(format!("{},{}", point.x, point.y)),
        }
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let (tracks, mut trains) = parse_input(input);
    let bounds = get_bounds(&tracks);
    loop {
        match tick(&mut trains, &tracks, bounds, true) {
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
