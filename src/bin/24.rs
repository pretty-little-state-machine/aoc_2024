use pathfinding::prelude::astar;

//  Modify for tests vs real input
/*
const HEIGHT: usize = 6;
const WIDTH: usize = 8;
const LCM: usize = 12;
*/

const HEIGHT: usize = 37;
const WIDTH: usize = 102;
const TIME_CUBE_TICKS: usize = 720;

// Do not touch these
const BLIZZ_HEIGHT: usize = HEIGHT - 2;
const BLIZZ_WIDTH: usize = WIDTH - 2;
const BLIZZ_SIZE: usize = BLIZZ_HEIGHT * BLIZZ_WIDTH;
const UP: usize = 0;
const DOWN: usize = 1;
const LEFT: usize = 2;
const RIGHT: usize = 3;

type TimeCube = [[[bool; BLIZZ_WIDTH]; HEIGHT]; TIME_CUBE_TICKS];

#[derive(Debug)]
struct Basin {
    blizzards: [[bool; BLIZZ_SIZE]; 4],
}

impl Basin {
    fn new(input: &str) -> Self {
        let mut slf = Basin {
            blizzards: [[false; BLIZZ_SIZE]; 4],
        };
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if y == 0 || x == 0 || y > BLIZZ_HEIGHT || x > BLIZZ_WIDTH {
                    continue;
                }
                let idx = (y - 1) * BLIZZ_WIDTH + x - 1;
                match char {
                    '.' => {
                        for b in 0..4 {
                            slf.blizzards[b][idx] = false;
                        }
                    }
                    '^' => slf.blizzards[UP][idx] = true,
                    'v' => slf.blizzards[DOWN][idx] = true,
                    '<' => slf.blizzards[LEFT][idx] = true,
                    '>' => slf.blizzards[RIGHT][idx] = true,
                    _ => (),
                }
            }
        }
        slf
    }

    // Maybe this is faster with bitmasks?
    fn tick_blizzards(&mut self) {
        self.blizzards[UP].rotate_left(BLIZZ_WIDTH);
        self.blizzards[DOWN].rotate_right(BLIZZ_WIDTH);
        // Slice rights and lefts into chunks, shift those chunks
        for x in self.blizzards[RIGHT].chunks_mut(BLIZZ_WIDTH) {
            x.rotate_right(1);
        }
        for x in self.blizzards[LEFT].chunks_mut(BLIZZ_WIDTH) {
            x.rotate_left(1);
        }
    }

    // Returns the basin as a 2-dimensional grid slice. This also adds in a fixed slice for the
    // start and end end rows in the basin since these are fixed. We ignore walls on the sides, so
    // start position is 0,0
    fn get_slice(&self) -> [[bool; BLIZZ_WIDTH]; HEIGHT] {
        let mut slice = [[true; BLIZZ_WIDTH]; HEIGHT];
        slice[0][0] = false;
        #[allow(clippy::needless_range_loop)] // Clippy suggests an impossible borrow here.
        for y in 1..=BLIZZ_HEIGHT {
            for x in 0..BLIZZ_WIDTH {
                let idx = (y - 1) * BLIZZ_WIDTH + x;
                slice[y][x] = self.blizzards[UP][idx]
                    || self.blizzards[DOWN][idx]
                    || self.blizzards[LEFT][idx]
                    || self.blizzards[RIGHT][idx];
            }
        }
        slice[HEIGHT - 1][BLIZZ_WIDTH - 1] = false;
        slice
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
struct Pos(usize, usize, usize);

impl Pos {
    // Distance here is only measured in two dimensions, we do not care about the time coordinate
    fn distance(&self, other: &Pos) -> u32 {
        ((self.0.abs_diff(other.0)) + (self.1.abs_diff(other.1))) as u32
    }
    /// Chooses the next successor in time, so the next slice is always checked. Tiles that are
    /// blizzards are excluded completely from the successors list
    fn successors(&self, time_cube: &TimeCube) -> Vec<(Pos, u32)> {
        // The time coordinate is the modulo of the TimeCube's length since the pattern repeats
        let &Pos(x, y, t) = self;
        // Limit the a* to some reasonable number of time steps
        if t > 1_000 {
            return Vec::new();
        }
        vec![
            Pos(x, y, t + 1),                   // Waiting in place
            Pos(x, y.saturating_sub(1), t + 1), // Moving North
            Pos(x, y + 1, t + 1),               // Moving South
            Pos(x + 1, y, t + 1),               // Moving East
            Pos(x.saturating_sub(1), y, t + 1), // Moving West
        ]
        .into_iter()
        .map(|p| {
            if p.0 == BLIZZ_WIDTH || p.1 == HEIGHT || time_cube[p.2 % time_cube.len()][p.1][p.0] {
                (p, 999_999)
            } else {
                (p, 1)
            }
        })
        .filter(|(_, c)| *c == 1)
        .collect()
    }
}

fn build_time_cube(input: &str) -> Box<TimeCube> {
    let mut basin = Basin::new(input);
    // Just run the blizzard simulation for a while and we'll DFS through the "layers" of snow in
    // a 3d coordinate space where Z is ~~~time itself~~~. The pattern repeats itself every least-
    // common multiple of width and height so we only need to really build this once then we can
    // stack the graphs if we need to _go deeper_.

    // When the Sun shines upon Earth, 2 – major Time points are created on opposite sides of Earth
    let mut time_cube = Box::new([[[false; BLIZZ_WIDTH]; HEIGHT]; TIME_CUBE_TICKS]);
    for z in 0..TIME_CUBE_TICKS {
        time_cube[z] = basin.get_slice();
        basin.tick_blizzards();
    }
    time_cube
}

fn get_path(start: &Pos, goal: &Pos, time_cube: &TimeCube) -> (Vec<Pos>, u32) {
    astar(
        start,
        |p| p.successors(time_cube),
        |p| p.distance(goal) / 3,
        |p| p.0 == goal.0 && p.1 == goal.1,
    )
    .unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let time_cube = build_time_cube(input);
    let (_, minutes) = get_path(
        &Pos(0, 0, 0),
        &Pos(BLIZZ_WIDTH - 1, HEIGHT - 1, 0),
        &time_cube,
    );
    Some(minutes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let time_cube = build_time_cube(input);
    let (_, minutes_a) = get_path(
        &Pos(0, 0, 0),
        &Pos(BLIZZ_WIDTH - 1, HEIGHT - 1, 0),
        &time_cube,
    );
    let (_, minutes_b) = get_path(
        &Pos(BLIZZ_WIDTH - 1, HEIGHT - 1, minutes_a as usize),
        &Pos(0, 0, 0),
        &time_cube,
    );
    let (_, minutes_c) = get_path(
        &Pos(0, 0, (minutes_a + minutes_b) as usize),
        &Pos(BLIZZ_WIDTH - 1, HEIGHT - 1, 0),
        &time_cube,
    );
    Some(minutes_a + minutes_b + minutes_c)
}

fn main() {
    let input = &aoc::read_file("inputs", 24);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
