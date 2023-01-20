use crate::UnitKind::{Elf, Goblin, Wall};
use rustc_hash::FxHashMap;

type Cavern = FxHashMap<Point, Unit>;

/// Point attributes are set so we  can sort on them in _reading order_, left to right, top down.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct Point {
    y: usize,
    x: usize,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(u8)]
enum UnitKind {
    Elf = b'E',
    Goblin = b'G',
    Wall = b'#',
}

#[derive(Debug, Copy, Clone)]
struct Unit {
    kind: UnitKind,
    hp: usize,
    pos: Point,
}

#[inline(always)]
fn add_cavern_unit(kind: UnitKind, point: Point, cavern: &mut Cavern) {
    cavern.insert(
        point,
        Unit {
            kind,
            hp: 200,
            pos: point,
        },
    );
}

fn new_cavern(input: &str) -> Cavern {
    let mut cavern = FxHashMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                'E' => add_cavern_unit(Elf, Point { x, y }, &mut cavern),
                'G' => add_cavern_unit(Goblin, Point { x, y }, &mut cavern),
                '#' => add_cavern_unit(Wall, Point { x, y }, &mut cavern),
                '.' => (),
                _ => unreachable!("Unknown unit"),
            };
        }
    }
    cavern
}

/// Attempts to attack the lowest HP enemy in range. If there is a tie the read-order determines
/// the correct target.
#[inline(always)]
fn attempt_attack(unit: &Unit, cavern: &mut Cavern) {
    let mut choices: FxHashMap<Point, usize> = FxHashMap::default();
    let enemy_kind = match unit.kind {
        Elf => Goblin,
        Goblin => Elf,
        Wall => unreachable!("Walls can't attack!"),
    };
    // Order here is critical - we want to scan in "reading order", left to right, top down.
    #[cfg_attr(rustfmt, rustfmt_skip)]
    let test_points = [
        Point { x: unit.pos.x, y: unit.pos.y - 1 },  // North
        Point { x: unit.pos.x - 1,  y: unit.pos.y }, // West
        Point { x: unit.pos.x + 1,  y: unit.pos.y},  // East
        Point { x: unit.pos.x, y: unit.pos.y + 1},   // South
    ];
    for point in test_points {
        if let Some(target) = cavern.get(&point) {
            if target.kind == enemy_kind {
                choices.insert(point, target.hp);
            }
        }
    }

    println!("choices: {choices:?}");
}

#[inline(always)]
fn add_empty_point(point: Point, points: &mut Vec<Point>, cavern: &Cavern) {
    if !cavern.contains_key(&point) {
        points.push(point)
    }
}

fn get_unit_kind_neighbors(cavern: &Cavern, kind: UnitKind) -> Vec<Point> {
    let mut targets: Vec<Point> = cavern
        .values()
        .filter_map(|unit| {
            if unit.kind == kind {
                Some(unit.pos)
            } else {
                None
            }
        })
        .collect();
    // Now only _empty_ adjacent points around each target
    let mut points: Vec<Point> = Vec::with_capacity(targets.len() * 4);
    for t in targets {
        add_empty_point(Point { x: t.x + 1, y: t.y }, &mut points, cavern);
        add_empty_point(Point { x: t.x - 1, y: t.y }, &mut points, cavern);
        add_empty_point(Point { x: t.x, y: t.y + 1 }, &mut points, cavern);
        add_empty_point(Point { x: t.x, y: t.y - 1 }, &mut points, cavern);
    }
    points
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UnitKind::{Elf, Goblin, Wall};

    #[test]
    fn test_attempt_attack() {
        let input = "#######
#.GEG.#
#..G#.#
#.G.#G#
#######";
        let mut cavern = new_cavern(input);
        let elf = cavern.get(&Point { x: 3, y: 1 }).unwrap().clone();
        attempt_attack(&elf, &mut cavern);
    }

    #[test]
    /// Tests a cavern to find adjacent cells. This cavern is taken from the examples provided.
    fn test_get_unit_kind_neighbors() {
        let input = "#######
#E..G.#
#...#.#
#.G.#G#
#######";
        let cavern = new_cavern(input);
        assert_eq!(
            get_unit_kind_neighbors(&cavern, Elf),
            vec![Point { y: 1, x: 2 }, Point { y: 2, x: 1 }]
        );
        assert_eq!(
            get_unit_kind_neighbors(&cavern, Goblin),
            vec![
                Point { y: 1, x: 5 },
                Point { y: 1, x: 3 },
                Point { y: 3, x: 3 },
                Point { y: 3, x: 1 },
                Point { y: 2, x: 2 },
                Point { y: 2, x: 5 }
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
