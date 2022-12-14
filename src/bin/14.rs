const OFFSET: usize = 1000;
const CHAMBER_HEIGHT: usize = 600;
const CHAMBER_WIDTH: usize = (600 + OFFSET) * 2;
const SAND_START: (usize, usize) = (0, 500 + OFFSET);

struct Chamber {
    grid: Vec<Vec<usize>>,
    floor_y: usize,
}

fn extract_points(input: &str) -> (usize, usize) {
    let coords: Vec<usize> = input.split(",").collect::<Vec<&str>>().iter().map(|v| v.parse::<usize>().unwrap()).collect();
    (*coords.first().unwrap(), *coords.get(1).unwrap())
}

impl Chamber {
    fn new(input: &str) -> Self {
        let mut slf = Self {
            grid: vec![vec![0; CHAMBER_WIDTH]; CHAMBER_HEIGHT],
            floor_y: 0,
        };

        for line in input.lines() {
            let rock_lines: Vec<&str> = line.split(" -> ").collect();
            for rock_line in rock_lines.windows(2) {
                let (x_start, y_start) = extract_points(rock_line[0]);
                let (x_end, y_end) = extract_points(rock_line[1]);
                if y_start > slf.floor_y {
                    slf.floor_y = y_start;
                } else if y_end > slf.floor_y {
                    slf.floor_y = y_end;
                }
                if x_start != x_end {
                    slf.add_horizontal_stone(x_start + OFFSET, x_end + OFFSET, y_start);
                } else {
                    slf.add_vertical_stone(y_start, y_end, x_start + OFFSET);
                }
            }
        }
        slf.floor_y += 2;
        slf
    }

    fn drop_sand(&mut self) -> bool {
        let mut x = SAND_START.1;
        for y in SAND_START.0..=CHAMBER_HEIGHT {
            if self.grid[y][x] != 0 {
                return false; // Can't spawn sand! Tunnel is blocked.
            }
            // println!("DROP: {}, {}", x, y);
            // Fall down
            if y + 1 >= CHAMBER_HEIGHT {
                return false;  // Out of bounds
            } else if self.grid[y + 1][x] == 0 {
                continue;
            }
            // Attempt fall down and to the left
            if x == 0 {
                return false; // Out of bounds
            } else if self.grid[y + 1][x - 1] == 0 {
                x -= 1;
                continue;
            }
            // Attempt fall down and to the right
            if x + 1 >= CHAMBER_WIDTH {
                return false; // Out of bounds
            } else if self.grid[y + 1][x + 1] == 0 {
                x += 1;
                continue;
            }
            // If we aren't out of bounds and haven't continued the loop the grain is at rest
            self.grid[y][x] = 2;
            // println!("STOP: {}, {}", x, y);
            break;
        }
        true
    }

    fn add_horizontal_stone(&mut self, start_x: usize, end_x: usize, y: usize) {
        let (x0, x1) =
            if start_x > end_x {
                (end_x, start_x)
            } else {
                (start_x, end_x)
            };
        for x in x0..=x1 {
            self.grid[y][x] = 1;
        }
    }

    fn add_vertical_stone(&mut self, start_y: usize, end_y: usize, x: usize) {
        let (y0, y1) =
            if start_y > end_y {
                (end_y, start_y)
            } else {
                (start_y, end_y)
            };
        for y in y0..=y1 {
            self.grid[y][x] = 1;
        }
    }

    fn print(&self) {
        for y in 0..CHAMBER_HEIGHT {
            for x in 494..CHAMBER_WIDTH {
                print!("{}",
                       match self.grid[y][x] {
                           0 => ".".to_string(),
                           1 => "#".to_string(),
                           2 => "o".to_string(),
                           _ => unreachable!("Unknown particle in cave grid!"),
                       });
            }
            println!();
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut chamber = Chamber::new(input);
    let mut steps = 0;
    while chamber.drop_sand() {
        steps += 1;
    }
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut chamber = Chamber::new(input);
    chamber.add_horizontal_stone(0, CHAMBER_WIDTH - 1, chamber.floor_y); // Build the floor
    let mut steps = 0;
    while chamber.drop_sand() {
        // chamber.print();
        steps += 1;
    }
    Some(steps)
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
