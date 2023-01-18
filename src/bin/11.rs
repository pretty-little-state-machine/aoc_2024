use rustc_hash::FxHashMap;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct FuelCell {
    power_level: isize,
}

impl FuelCell {
    fn new(point: Point, grid_serial: isize) -> Self {
        let rack_id = point.x + 10;
        let mut power_level = (rack_id * point.y + grid_serial) * rack_id;
        power_level = (power_level % 1000) / 100;
        power_level -= 5;
        Self { power_level }
    }
}

fn build_grid(grid_serial: isize) -> FxHashMap<Point, FuelCell> {
    let mut grid = FxHashMap::with_capacity_and_hasher(300 * 300, Default::default());
    for y in 1..=300 {
        for x in 1..=300 {
            grid.insert(Point { x, y }, FuelCell::new(Point { x, y }, grid_serial));
        }
    }
    grid
}

/// See: https://en.wikipedia.org/wiki/Summed-area_table
fn fast_sum(summed_area: &FxHashMap<Point, isize>, size: isize) -> (isize, Point) {
    let mut best_sum = 0;
    let mut best_corner: Point = Point { x: 0, y: 0 };
    for y in 1..=(300 - size) {
        for x in 1..=(300 - size) {
            let (a, b, c, d) = (
                summed_area.get(&Point { x, y }).unwrap(),
                summed_area.get(&Point { x: x + size, y }).unwrap(),
                summed_area.get(&Point { x, y: y + size }).unwrap(),
                summed_area
                    .get(&Point {
                        x: x + size,
                        y: y + size,
                    })
                    .unwrap(),
            );
            let sum = d + a - b - c;
            if sum > best_sum {
                best_sum = sum;
                best_corner = Point { x: x + 1, y: y + 1 };
            }
        }
    }
    (best_sum, best_corner)
}

/// Take the naive grid and turn it into a summed area grid for faster windowing later
/// See: https://en.wikipedia.org/wiki/Summed-area_table
fn make_summed_area(grid: &FxHashMap<Point, FuelCell>) -> FxHashMap<Point, isize> {
    let mut summed_area: FxHashMap<Point, isize> =
        FxHashMap::with_capacity_and_hasher(300 * 300, Default::default());
    // Corner
    summed_area.insert(
        Point { x: 1, y: 1 },
        grid.get(&Point { x: 1, y: 1 }).unwrap().power_level,
    );
    // First Column
    for y in 2..=300 {
        summed_area.insert(
            Point { x: 1, y },
            grid.get(&Point { x: 1, y }).unwrap().power_level
                + summed_area.get(&Point { x: 1, y: y - 1 }).unwrap(),
        );
    }
    // First Row
    for x in 2..=300 {
        summed_area.insert(
            Point { x, y: 1 },
            grid.get(&Point { x, y: 1 }).unwrap().power_level
                + summed_area.get(&Point { x: x - 1, y: 1 }).unwrap(),
        );
    }
    // Fill in the rest
    for y in 2..=300 {
        for x in 2..=300 {
            summed_area.insert(
                Point { x, y },
                grid.get(&Point { x, y }).unwrap().power_level
                    + summed_area.get(&Point { x, y: y - 1 }).unwrap()
                    + summed_area.get(&Point { x: x - 1, y }).unwrap()
                    - summed_area.get(&Point { x: x - 1, y: y - 1 }).unwrap(),
            );
        }
    }
    summed_area
}

pub fn part_one(input: &str) -> Option<String> {
    let grid_serial = input.parse::<isize>().unwrap();
    let grid = build_grid(grid_serial);
    let summed_area = make_summed_area(&grid);
    let (_, best_corner) = fast_sum(&summed_area, 3);
    Some(format!("{},{}", best_corner.x, best_corner.y))
}

pub fn part_two(input: &str) -> Option<String> {
    let grid_serial = input.parse::<isize>().unwrap();
    let grid = build_grid(grid_serial);
    let summed_area = make_summed_area(&grid);

    let mut best_corner: Point = Point { x: 0, y: 0 };
    let mut best_sum: isize = 0;
    let mut best_size: isize = 0;
    for n in 1..300 {
        let (sum, corner) = fast_sum(&summed_area, n);
        if sum > best_sum {
            best_sum = sum;
            best_corner = corner;
            best_size = n;
        }
    }
    Some(format!("{},{},{}", best_corner.x, best_corner.y, best_size))
}

fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_fuel_cell() {
        assert_eq!(FuelCell::new(Point { x: 3, y: 5 }, 8).power_level, 4);
        assert_eq!(FuelCell::new(Point { x: 122, y: 79 }, 57).power_level, -5);
        assert_eq!(FuelCell::new(Point { x: 217, y: 196 }, 39).power_level, 0);
        assert_eq!(FuelCell::new(Point { x: 101, y: 153 }, 71).power_level, 4);
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_one(&input), Some("21,61".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_two(&input), Some("232,251,12".to_string()));
    }
}
