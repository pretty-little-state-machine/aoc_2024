use rustc_hash::{FxHashSet};
type Point = (usize, usize); // (x, y)
type Direction = char;

#[derive(Debug)]
struct Bridge {
    visited: FxHashSet<Point>,
    knots: [Point; 10],
}

impl Default for Bridge {
    /// Initialize the simulation with the cursor of the knot in the middle of the bridge
    fn default() -> Self {
        Self {
            visited: FxHashSet::default(),
            knots: [(300, 300); 10],
        }
    }
}

impl Bridge {
    pub fn tick(&mut self, direction: Direction) {
        match direction {
            'R' => self.knots[0].0 += 1,
            'L' => self.knots[0].0 -= 1,
            'U' => self.knots[0].1 += 1,
            'D' => self.knots[0].1 -= 1,
            _ => unreachable!("Unsupported movement!"),
        }
    }

    /// Updates a segment based on the segment further up the rope
    #[inline(always)]
    fn update_knot(&mut self, knot_idx: usize) {
        let delta_x = self.knots[knot_idx - 1].0 as isize - self.knots[knot_idx].0 as isize;
        let delta_y = self.knots[knot_idx - 1].1 as isize - self.knots[knot_idx].1 as isize;

        if delta_x > 1 && delta_y > 1 {
            self.knots[knot_idx].0 += 1;
            self.knots[knot_idx].1 += 1;
        } else if delta_x < -1 && delta_y < -1 {
            self.knots[knot_idx].0 -= 1;
            self.knots[knot_idx].1 -= 1;
        } else if delta_x < -1 && delta_y > 1 {
            self.knots[knot_idx].0 -= 1;
            self.knots[knot_idx].1 += 1;
        } else if delta_x > 1 && delta_y < -1 {
            self.knots[knot_idx].0 += 1;
            self.knots[knot_idx].1 -= 1;
        } else if delta_x > 1 {
            self.knots[knot_idx].0 += 1;
            self.knots[knot_idx].1 = self.knots[knot_idx - 1].1;
        } else if delta_x < -1 {
            self.knots[knot_idx].0 -= 1;
            self.knots[knot_idx].1 = self.knots[knot_idx - 1].1;
        } else if delta_y > 1 {
            self.knots[knot_idx].1 += 1;
            self.knots[knot_idx].0 = self.knots[knot_idx - 1].0;
        } else if delta_y < -1 {
            self.knots[knot_idx].1 -= 1;
            self.knots[knot_idx].0 = self.knots[knot_idx - 1].0;
        }
    }
}

#[inline(always)]
/// Decodes a line's movement and distance. *WARNING* This is only safe for ASCII puzzle input.
fn decode_line(input: &str) -> (Direction, usize) {
    let direction = input.chars().next().unwrap();
    let count = input[2..].parse::<usize>().unwrap();
    (direction, count)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut bridge = Bridge::default();
    bridge.visited.insert(bridge.knots[1]);
    for line in input.lines() {
        let (direction, count) = decode_line(line);
        for _ in 0..count {
            bridge.tick(direction);
            bridge.update_knot(1);
            bridge.visited.insert(bridge.knots[1]);
        }
    }
    Some(bridge.visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut bridge = Bridge::default();
    bridge.visited.insert(bridge.knots[9]);
    for line in input.lines() {
        let (direction, count) = decode_line(line);
        for _ in 0..count {
            bridge.tick(direction);
            for knot_idx in 1..=9 {
                bridge.update_knot(knot_idx);
            }
            bridge.visited.insert(bridge.knots[9]);
        }
    }
    Some(bridge.visited.len() as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            .to_string();
        assert_eq!(part_two(&input), Some(36));
    }
}
