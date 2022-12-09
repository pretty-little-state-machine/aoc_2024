const GRID_SIZE: usize = 600;
const START: usize = GRID_SIZE / 2;

type Point = (usize, usize); // (x, y)
type Direction = char;

#[derive(Debug)]
struct Bridge {
    grid: Box<[[usize; GRID_SIZE]; GRID_SIZE]>,
    head: Point,
    tail: Point,
}

impl Default for Bridge {
    /// Initialize the simulation with the cursor of the knot in the middle of the bridge
    fn default() -> Self {
        Self {
            grid: Box::new([[0; GRID_SIZE]; GRID_SIZE]),
            head: (START, START),
            tail: (START, START),
        }
    }
}

impl Bridge {
    /// Advent of code output appears to use 0,0 as the bottom left so we will do the same.
    pub fn print(&self) {
        for (row_idx, row) in self.grid.iter().enumerate().rev() {
            for (col_idx, col) in row.iter().enumerate() {
                if self.head.1 == row_idx && self.head.0 == col_idx {
                    print!("H")
                } else if self.tail.1 == row_idx && self.tail.0 == col_idx {
                    print!("T");
                } else if *col == 0 {
                    print!(".")
                } else {
                    print!("#")
                }
            }
            println!();
        }
        println!();
    }

    pub fn tick(&mut self, direction: Direction) {
        match direction {
            'R' => self.head.0 += 1,
            'L' => self.head.0 -= 1,
            'U' => self.head.1 += 1,
            'D' => self.head.1 -= 1,
            _ => unreachable!("Unsupported movement!"),
        }
    }

    /// Updates the tail's position based on the head's movement. head == tail is a No-op.
    #[inline(always)]
    fn update_tail(&mut self) {
        let delta_x = self.head.0 as isize - self.tail.0 as isize;
        let delta_y = self.head.1 as isize - self.tail.1 as isize;

        if delta_x > 1 {
            self.tail.0 += 1;
            self.tail.1 = self.head.1;
        } else if delta_x < -1 {
            self.tail.0 -= 1;
            self.tail.1 = self.head.1;
        }
        if delta_y > 1 {
            self.tail.1 += 1;
            self.tail.0 = self.head.0;
        } else if delta_y < -1 {
            self.tail.1 -= 1;
            self.tail.0 = self.head.0;
        }
    }

    #[inline(always)]
    fn mark_tail_visited(&mut self) {
        self.grid[self.tail.1][self.tail.0] = 1;
    }

    #[inline(always)]
    fn count_visited(&self) -> usize {
        self.grid
            .iter()
            .map(|r| -> usize { r.iter().sum() })
            .sum()
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
    bridge.mark_tail_visited();
    for line in input.lines() {
        let (direction, count) = decode_line(line);
        for _ in 0..count {
            bridge.tick(direction);
            bridge.update_tail();
            bridge.mark_tail_visited();
        }
    };
    Some(bridge.count_visited() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
