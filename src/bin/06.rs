use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Default, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(input: &str) -> Self {
        let values = input
            .split(", ")
            .map(|v| v.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        Self {
            x: values[1],
            y: values[0],
        }
    }

    fn manhattan(self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

/// Returns the bottom-right point the diagram.
fn get_bounds(points: &Vec<Point>) -> (Point, Point) {
    let mut bottom_right = Point::default();
    let mut top_left = Point {
        x: isize::MAX,
        y: isize::MAX,
    };
    for point in points {
        if point.x < top_left.x {
            top_left.x = point.x
        }
        if point.x > bottom_right.x {
            bottom_right.x = point.x
        }
        if point.y > bottom_right.y {
            bottom_right.y = point.y
        }
        if point.y < top_left.y {
            top_left.y = point.y
        }
    }
    (top_left, bottom_right)
}

/// A naive implementation of the Voronoi algorithm.
///
/// CAVEAT: The problem explicitly states that shared edges are not counted in area!
///   NOTE: See `Fortunes algorithm` for a potentially quicker solution.
pub fn voronoi(
    points: &[Point],
    bounds: (Point, Point),
) -> (FxHashMap<Point, usize>, FxHashSet<Point>) {
    let mut area: FxHashMap<Point, usize> = FxHashMap::default();
    let mut prune_list: FxHashSet<Point> = FxHashSet::default();

    for y in bounds.0.y..=bounds.1.y {
        for x in bounds.0.x..=bounds.1.x {
            let mut distances = points
                .iter()
                .map(|target| (*target, target.manhattan(&Point { x, y })))
                .collect::<Vec<(Point, usize)>>();
            distances.sort_by_key(|k| k.1);
            // If the area is at the bounds limit then it's going to grow forever. We build a prune
            // list of these nodes that had the shortest distance to the edge.
            if x == bounds.0.x || x == bounds.1.x || y == bounds.0.x || y == bounds.1.x {
                prune_list.insert(distances.first().unwrap().0);
                continue;
            }
            // If the two smallest distances are the same we discard the result per the problem.
            if distances.first().unwrap().1 != distances.get(1).unwrap().1 {
                let (target, _) = &distances.first().unwrap();
                if area.contains_key(target) {
                    let value = area.get_mut(target).unwrap();
                    *value += 1;
                } else {
                    area.insert(*target, 1);
                }
            }
        }
    }
    (area, prune_list)
}

pub fn part_one(input: &str) -> Option<usize> {
    let points = input.lines().map(Point::new).collect::<Vec<Point>>();
    let bounds = get_bounds(&points);
    let (mut area, prune_list) = voronoi(&points, bounds);
    area.retain(|p, _| !prune_list.contains(p));
    Some(*area.values().max().unwrap())
}

pub fn part_two(input: &str) -> Option<usize> {
    //const MAX_DISTANCE: usize = 32;
    const MAX_DISTANCE: usize = 10_000;
    let points = input.lines().map(Point::new).collect::<Vec<Point>>();
    let bounds = get_bounds(&points);
    let mut answer: usize = 0;
    for y in bounds.0.y..=bounds.1.y {
        for x in bounds.0.x..=bounds.1.x {
            let sum_distance: usize = points
                .iter()
                .map(|target| target.manhattan(&Point { x, y }))
                .sum();
            if sum_distance < MAX_DISTANCE {
                answer += 1;
            }
        }
    }
    Some(answer)
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(17));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(16));
    }
}
