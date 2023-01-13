#![allow(dead_code)]

use image::{ImageBuffer, Rgb};
use rand::Rng;

const OFFSET: usize = 10;
const CHAMBER_HEIGHT: usize = 165;
const CHAMBER_WIDTH: usize = 675;
const SAND_START: (usize, usize) = (0, 500 + OFFSET);

struct Chamber {
    grid: Vec<usize>,
    floor_y: usize,
}

#[inline(always)]
fn extract_points(input: &str) -> (usize, usize) {
    let coords: Vec<usize> = input
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    (*coords.first().unwrap(), *coords.get(1).unwrap())
}

#[inline(always)]
fn cell_idx(x: usize, y: usize) -> usize {
    y * CHAMBER_WIDTH + x
}

#[inline(always)]
fn sand_color() -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    let offset = rng.gen_range(0.7..1.0);
    Rgb([
        (217.0 * offset) as u8,
        (217.0 * offset) as u8,
        (135.0 * offset) as u8,
    ])
}

#[inline(always)]
fn rock_color() -> Rgb<u8> {
    let mut rng = rand::thread_rng();
    let offset = rng.gen_range(0.8..1.0);
    Rgb([
        (106.0 * offset) as u8,
        (107.0 * offset) as u8,
        (105.0 * offset) as u8,
    ])
}

impl Chamber {
    fn new(input: &str) -> Self {
        let mut slf = Self {
            grid: vec![0; CHAMBER_WIDTH * CHAMBER_HEIGHT],
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
        for y in SAND_START.0..CHAMBER_HEIGHT - 1 {
            if self.grid[cell_idx(x, y)] != 0 {
                return false;
            }
            if x + 1 >= CHAMBER_WIDTH || x == 0 {
                return false;
            }
            if self.grid[cell_idx(x, y + 1)] == 0 {
                continue;
            } else if self.grid[cell_idx(x - 1, y + 1)] == 0 {
                x -= 1;
                continue;
            } else if self.grid[cell_idx(x + 1, y + 1)] == 0 {
                x += 1;
                continue;
            } else {
                self.grid[cell_idx(x, y)] = 2;
                return true;
            }
        }
        false
    }

    fn img(&self, step: usize) {
        let scale: u32 = 4;
        let start_x: u32 = 400;
        let end_x: u32 = (CHAMBER_WIDTH) as u32;

        let width = (end_x - start_x) * scale;
        let height = CHAMBER_HEIGHT as u32 * scale;

        let mut imgbuf = ImageBuffer::new(width - 300, height);
        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let grid_x = ((x / scale) + start_x) as usize;
            let grid_y = (y / scale) as usize;
            match self.grid[cell_idx(grid_x, grid_y)] {
                0 => *pixel = Rgb([0_u8, 0_u8, 0_u8]),
                1 => *pixel = rock_color(),
                2 => *pixel = sand_color(),
                _ => unreachable!("Undefined block type"),
            }
        }
        let file_name = format!("src/viz/{step:0>4}.png");
        imgbuf.save(file_name).expect("Coulnd't build output image");
    }

    fn add_horizontal_stone(&mut self, start_x: usize, end_x: usize, y: usize) {
        let (x0, x1) = if start_x > end_x {
            (end_x, start_x)
        } else {
            (start_x, end_x)
        };
        for x in x0..=x1 {
            self.grid[cell_idx(x, y)] = 1;
        }
    }

    fn add_vertical_stone(&mut self, start_y: usize, end_y: usize, x: usize) {
        let (y0, y1) = if start_y > end_y {
            (end_y, start_y)
        } else {
            (start_y, end_y)
        };
        for y in y0..=y1 {
            self.grid[cell_idx(x, y)] = 1;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut chamber = Chamber::new(input);
    let mut steps: u32 = 0;
    while chamber.drop_sand() {
        steps += 1;
        // chamber.img(steps as usize);
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
