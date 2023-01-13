use crate::Action::{MoveForward, TurnLeft, TurnRight};
use crate::Facing::{Down, Left, Right, Up};
use regex::Regex;
use rustc_hash::FxHashMap;

type Cell = (isize, isize);
type Map = FxHashMap<Cell, char>;
type CubeMap = [FxHashMap<Cell, char>; 6];

fn read_input_p2_real(input: &str) -> (CubeMap, Vec<Action>) {
    const DIM: usize = 50;
    let mut cube_map = [
        Map::default(),
        Map::default(),
        Map::default(),
        Map::default(),
        Map::default(),
        Map::default(),
    ];
    for (row, line) in input.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        // Build up the six faces while normalizing the coordinate space
        for (col, char) in line.chars().enumerate() {
            match char {
                '.' | '#' => {
                    if row < 50 && col >= 100 {
                        cube_map[0].insert(((row % DIM) as isize, (col % DIM) as isize), char);
                    } else if row < 50 && col >= 50 {
                        cube_map[1].insert(((row % DIM) as isize, (col % DIM) as isize), char);
                    } else if row < 100 && col >= 50 {
                        cube_map[2].insert(((row % DIM) as isize, (col % DIM) as isize), char);
                    } else if row < 150 && col >= 50 {
                        cube_map[3].insert(((row % DIM) as isize, (col % DIM) as isize), char);
                    } else if row < 150 {
                        cube_map[4].insert(((row % DIM) as isize, (col % DIM) as isize), char);
                    } else if row < 200 {
                        cube_map[5].insert(((row % DIM) as isize, (col % DIM) as isize), char);
                    }
                }
                _ => (),
            }
        }
    }
    (cube_map, build_actions(input))
}

fn read_input_p1(input: &str) -> (Map, Vec<Action>) {
    let mut map = Map::default();
    for (row, line) in input.lines().enumerate() {
        if line.is_empty() {
            break;
        }
        for (col, char) in line.chars().enumerate() {
            match char {
                '.' | '#' => {
                    map.insert((row as isize, col as isize), char);
                }
                _ => (),
            }
        }
    }
    (map, build_actions(input))
}

fn build_actions(input: &str) -> Vec<Action> {
    let mut actions: Vec<Action> = Vec::new();
    let re = Regex::new(r"([0-9]+|[LR])").unwrap();
    re.captures_iter(input.lines().last().unwrap())
        .for_each(|c| {
            if let Ok(integer) = c.get(0).unwrap().as_str().parse::<isize>() {
                actions.push(MoveForward(integer))
            } else {
                match c.get(0).unwrap().as_str() {
                    "R" => actions.push(TurnRight),
                    "L" => actions.push(TurnLeft),
                    _ => unreachable!("Invalid movement command"),
                }
            }
        });
    actions
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Player {
    facing: Facing,
    position: Cell,
    current_face: usize,
}

impl Player {
    /// Creates a new player facing right in the leftmost open tile in the top row
    fn new(map: &Map, current_face: usize) -> Self {
        let mut starting_point: Cell = (0, 9999);
        for cell in map.keys() {
            if cell.0 == 0 && cell.1 < starting_point.1 {
                starting_point = *cell;
            }
        }
        Self {
            facing: Right,
            position: starting_point,
            current_face,
        }
    }

    // Executes a single navigation on the map
    fn navigate(&mut self, distance: isize, map: &Map) {
        for _ in 0..distance {
            match self.facing {
                Up => self.step_row(-1, map),
                Down => self.step_row(1, map),
                Left => self.step_column(-1, map),
                Right => self.step_column(1, map),
            }
        }
    }

    /// Returns the adjacent face, Cell and Facing based on current facing and position.
    ///
    /// Any non-edge position will maintain the current face and Facing, making this safe to call
    /// at any time.
    fn wrap_cube(&self) -> (usize, Cell, Facing) {
        const MAX_IDX: isize = 49;
        // Ensure that we only wrap if we're on an edge _AND_ facing the appropriate direction
        let row = self.position.0;
        let col = self.position.1;
        if (row == MAX_IDX && self.facing == Down)
            || (row == 0 && self.facing == Up)
            || (col == 0 && self.facing == Left)
            || (col == MAX_IDX && self.facing == Right)
        {
            // 1-indexed for sanity. This is specific to the puzzle input
            let (mut new_face, cell, facing) = match (self.current_face + 1, self.facing) {
                (1, Up) => (6, (MAX_IDX, col), Up),
                (1, Down) => (3, (col, MAX_IDX), Left),
                (1, Right) => (4, (MAX_IDX - row, MAX_IDX), Left), // Inversion case
                (1, Left) => (2, (row, MAX_IDX), Left),
                (2, Up) => (6, (col, 0), Right),
                (2, Down) => (3, (0, col), Down),
                (2, Right) => (1, (row, 0), Right),
                (2, Left) => (5, (MAX_IDX - row, 0), Right), // Inversion Case
                (3, Up) => (2, (MAX_IDX, col), Up),
                (3, Down) => (4, (0, col), Down),
                (3, Right) => (1, (MAX_IDX, row), Up),
                (3, Left) => (5, (0, row), Down),
                (4, Up) => (3, (MAX_IDX, col), Up),
                (4, Down) => (6, (col, MAX_IDX), Left),
                (4, Right) => (1, (MAX_IDX - row, MAX_IDX), Left), // Inversion Case
                (4, Left) => (5, (row, MAX_IDX), Left),
                (5, Up) => (3, (col, 0), Right),
                (5, Down) => (6, (0, col), Down),
                (5, Right) => (4, (row, 0), Right),
                (5, Left) => (2, (MAX_IDX - row, 0), Right), // Inversion Case
                (6, Up) => (5, (MAX_IDX, col), Up),
                (6, Down) => (1, (0, col), Down),
                (6, Right) => (4, (MAX_IDX, row), Up),
                (6, Left) => (2, (0, row), Down),
                _ => unreachable!("Invalid cube orientation"),
            };
            // Restore 0-indexing
            new_face -= 1;
            (new_face, cell, facing)
        } else {
            (self.current_face, self.position, self.facing)
        }
    }

    // Executes a single navigation on the map
    fn navigate_3d(&mut self, distance: isize, map: &CubeMap) {
        for _ in 0..distance {
            // This checks an implicit move "forwards" one square in whatever direction to wrap
            let (new_face, new_cell, new_facing) = self.wrap_cube();
            // We haven't changed coordinate systems, walk normally
            if self.current_face == new_face {
                // We don't need  to worry about underflow here cause edges will hit the wrap state
                let new_position = match self.facing {
                    Up => (self.position.0 - 1, self.position.1),
                    Down => (self.position.0 + 1, self.position.1),
                    Left => (self.position.0, self.position.1 - 1),
                    Right => (self.position.0, self.position.1 + 1),
                };
                // Don't try to continue walking if we hit a wall
                if is_wall(*map[self.current_face].get(&new_position).unwrap()) {
                    break;
                } else {
                    self.position = new_position;
                }
            } else {
                // If the new cell would hit a wall, we won't wrap faces, we're stuck
                if is_wall(*map[new_face].get(&new_cell).unwrap()) {
                    break;
                } else {
                    self.facing = new_facing;
                    self.current_face = new_face;
                    self.position = new_cell;
                }
            }
        }
    }

    // Steps a row forwards or backwards until a wall is hit
    fn step_row(&mut self, distance: isize, map: &Map) {
        if let Some(cell) = map.get(&(self.position.0 + distance, self.position.1)) {
            if !is_wall(*cell) {
                self.position.0 += distance;
            }
        } else if distance > 0 {
            for row in 0..200 {
                if let Some(wrap_cell) = map.get(&(row, self.position.1)) {
                    if !is_wall(*wrap_cell) {
                        self.position.0 = row;
                    }
                    break;
                }
            }
        } else {
            for row in (0..200).rev() {
                if let Some(wrap_cell) = map.get(&(row, self.position.1)) {
                    if !is_wall(*wrap_cell) {
                        self.position.0 = row;
                    }
                    break;
                }
            }
        };
    }

    fn step_column(&mut self, distance: isize, map: &Map) {
        if let Some(cell) = map.get(&(self.position.0, self.position.1 + distance)) {
            if !is_wall(*cell) {
                self.position.1 += distance;
            }
        } else if distance > 0 {
            for col in 0..200 {
                if let Some(wrap_cell) = map.get(&(self.position.0, col)) {
                    if !is_wall(*wrap_cell) {
                        self.position.1 = col;
                    }
                    break;
                }
            }
        } else {
            for col in (0..200).rev() {
                if let Some(wrap_cell) = map.get(&(self.position.0, col)) {
                    if !is_wall(*wrap_cell) {
                        self.position.1 = col;
                    }
                    break;
                }
            }
        };
    }

    fn turn_right(&mut self) {
        self.facing = match self.facing {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        };
    }

    fn turn_left(&mut self) {
        self.facing = match self.facing {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        };
    }

    fn password(&self) -> isize {
        let mut password = 1000 * (self.position.0 + 1) + (self.position.1 + 1) * 4;
        password += match self.facing {
            Up => 3,
            Down => 1,
            Left => 2,
            Right => 0,
        };
        password
    }

    fn password_part_2(&self) -> isize {
        let row = match self.current_face + 1 {
            1 => self.position.0,
            2 => self.position.0,
            3 => self.position.0 + 50,
            4 => self.position.0 + 100,
            5 => self.position.0 + 100,
            6 => self.position.0 + 150,
            _ => unreachable!("Invalid face"),
        };
        let col = match self.current_face + 1 {
            1 => self.position.1 + 100,
            2 => self.position.1 + 50,
            3 => self.position.1 + 50,
            4 => self.position.1 + 50,
            5 => self.position.1,
            6 => self.position.1,
            _ => unreachable!("Invalid face"),
        };

        let mut password = 1000 * (row + 1) + (col + 1) * 4;
        password += match self.facing {
            Up => 3,
            Down => 1,
            Left => 2,
            Right => 0,
        };
        password
    }
}

#[derive(Debug)]
enum Action {
    MoveForward(isize),
    TurnLeft,
    TurnRight,
}

/// Returns true if the character is a wall
#[inline(always)]
fn is_wall(char: char) -> bool {
    char == '#'
}

pub fn part_one(input: &str) -> Option<isize> {
    let (map, actions) = read_input_p1(input);
    let mut player = Player::new(&map, 0);
    for action in actions {
        match action {
            MoveForward(distance) => player.navigate(distance, &map),
            TurnLeft => player.turn_left(),
            TurnRight => player.turn_right(),
        }
    }
    Some(player.password())
}

pub fn part_two(input: &str) -> Option<isize> {
    let (cube_map, actions) = read_input_p2_real(input);
    // Hard-coded starting face for my input
    let mut player = Player::new(&cube_map[1], 1);
    for action in actions {
        match action {
            MoveForward(distance) => player.navigate_3d(distance, &cube_map),
            TurnLeft => player.turn_left(),
            TurnRight => player.turn_right(),
        }
    }

    Some(player.password_part_2())
}

fn main() {
    let input = &aoc::read_file("inputs", 22);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(5031));
    }
}
