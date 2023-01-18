use regex::Regex;
use rustc_hash::FxHashSet;

type Bounds = (isize, isize, isize, isize);
/// A point in 2d space with a velocity vector represented by (dx,dy)
#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl Point {
    fn new(input: &str) -> Self {
        let re = Regex::new(r".*[< ]([-\d]+)[, ]+([-\d]+)>.*[< ]([-\d]+)[, ]+([-\d]+)>").unwrap();
        let caps = re.captures(input).unwrap();
        Self {
            x: caps[1].parse::<isize>().unwrap(),
            y: caps[2].parse::<isize>().unwrap(),
            dx: caps[3].parse::<isize>().unwrap(),
            dy: caps[4].parse::<isize>().unwrap(),
        }
    }

    fn tick(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

fn get_bounds(points: &[Point]) -> Bounds {
    (
        points.iter().min_by_key(|p| p.x).unwrap().x,
        points.iter().min_by_key(|p| p.y).unwrap().y,
        points.iter().max_by_key(|p| p.x).unwrap().x,
        points.iter().max_by_key(|p| p.y).unwrap().y,
    )
}

fn get_message(points: &[Point], bounds: Bounds) -> String {
    // Build a hashset for quick point checking
    let mut message: String = String::new();
    let mut point_set = FxHashSet::default();
    points.iter().for_each(|p| {
        point_set.insert((p.x, p.y));
    });
    for y in bounds.1..=bounds.3 {
        for x in bounds.0..=bounds.2 {
            if point_set.contains(&(x, y)) {
                message.push('#');
            } else {
                message.push('.');
            }
        }
        message.push('\n');
    }
    message
}

/// Runs the simulation and returns the message string and how long it took
fn run_sign(input: &str) -> (String, usize) {
    let mut points = input.lines().map(Point::new).collect::<Vec<Point>>();
    const MESSAGE_HEIGHT: usize = 9; // 7 for tests, 9 for real
    let mut message: String = String::new();
    let mut time = 0;
    for _ in 0..100_000 {
        let bounds = get_bounds(&points);
        if bounds.1.abs_diff(bounds.3) == MESSAGE_HEIGHT {
            message = get_message(&points, bounds);
            break;
        }
        #[allow(clippy::needless_range_loop)] // Clippy doesn't know I'm mutating inside.
        for i in 0..points.len() {
            points[i].tick();
        }
        time += 1;
    }
    (message, time)
}

pub fn part_one(input: &str) -> Option<String> {
    let (message, _) = run_sign(input);
    Some(message)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, time) = run_sign(input);
    Some(time)
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let mut message = String::new();
        message.push_str(
            "#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
",
        );
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(message));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
