use crate::UnitKind::{Elf, Goblin, Wall};
use pathfinding::prelude::{yen};
use std::collections::BTreeMap;

type Cavern = BTreeMap<Point, Unit>;


/// Point attributes are set so we  can sort on them in _reading order_, left to right, top down.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct Point {
    y: usize,
    x: usize,
}

impl Point {
    fn distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
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

impl Unit {
    fn get_enemy_kind(&self) -> UnitKind {
        match self.kind {
            Elf => Goblin,
            Goblin => Elf,
            Wall => unreachable!("Walls can't have an enemy!"),
        }
    }
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
    let mut cavern = BTreeMap::default();
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

/// Attempts to attack the lowest HP enemy in range and returns `true` if an attack was successful.
/// If there is a tie the read-order determines the correct target.
#[inline(always)]
fn attempt_attack(unit: &Unit, cavern: &mut Cavern) -> bool {
    let mut choices: Vec<(usize, Point)> = Vec::new();
    let enemy_kind = unit.get_enemy_kind();
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
                choices.push((target.hp, point)); // Order is critical for sorting later.
            }
        }
    }

    choices.sort();
    if let Some((_hp, point)) = choices.first() {
        // Apply the damage
        let target = cavern.get_mut(point).unwrap();
        target.hp = target.hp.saturating_sub(3);
        print!(
            "{:?} ({}, {}) attacks {:?} ({},{}) for 3 damage! {} HP left.\n",
            unit.kind, unit.pos.x, unit.pos.y, target.kind, target.pos.x, target.pos.y, target.hp
        );
        if target.hp == 0 {
            print!("{:?} at {}, {} has died!", target.kind, target.pos.x, target.pos.y);
            cavern.remove(point);
        }
        true
    }  else {
        print!(
            "{:?} ({}, {}) has nothing to attack!\n",
            unit.kind, unit.pos.x, unit.pos.y
        );
        false
    }
}

#[inline(always)]
fn add_empty_point(point: Point, points: &mut Vec<Point>, cavern: &Cavern) {
    if !cavern.contains_key(&point) {
        points.push(point)
    }
}

/// Returns the points containing a certain unit kind
#[inline(always)]
fn get_unit_kind_points(cavern: &Cavern, unit_kind: UnitKind) -> Vec<Point> {
    cavern
        .values()
        .filter_map(|unit| {
            if unit.kind == unit_kind {
                Some(unit.pos)
            } else {
                None
            }
        })
        .collect()
}

#[inline(always)]
fn get_unit_kind_neighbors(cavern: &Cavern, kind: UnitKind) -> Vec<Point> {
    let targets: Vec<Point> = get_unit_kind_points(cavern, kind);
    // Now only _empty_ adjacent points around each target
    let mut points: Vec<Point> = Vec::with_capacity(targets.len() * 4);
    for t in targets {
        // These points must be in _reading order_
        add_empty_point(Point { x: t.x, y: t.y - 1 }, &mut points, cavern);
        add_empty_point(Point { x: t.x - 1, y: t.y }, &mut points, cavern);
        add_empty_point(Point { x: t.x + 1, y: t.y }, &mut points, cavern);
        add_empty_point(Point { x: t.x, y: t.y + 1 }, &mut points, cavern);
    }
    points
}

/// Returns the victorious UnitKind if a victory was one, otherwise None as combat is ongoing.
#[inline(always)]
fn finished_combat(cavern: &Cavern) -> Option<UnitKind> {
    let elves = get_unit_kind_points(cavern, Elf);
    let goblins = get_unit_kind_points(cavern, Goblin);
    if elves.len() == 0 {
        Some(Goblin)
    } else if goblins.len() == 0 {
        Some(Elf)
    } else {
        None
    }
}

/// The final score is the wining army's total HP multiplied by the number of rounds.
#[inline(always)]
fn score_game(cavern: &Cavern, kind: UnitKind, rounds: usize) -> usize {
    let points = get_unit_kind_points(cavern, kind);
    let mut total_hp = 0;
    for point in points {
        total_hp += cavern.get(&point).unwrap().hp;
    }
    println!("Total hit points: {}, rounds: {}", total_hp, rounds);
    total_hp * rounds
}

/// Obtains empty squares around a point. Useful for pathfinding and target detection. All moves
/// have a cost of 1.
#[inline(always)]
fn cavern_successors(cavern: &Cavern, p: &Point) -> Vec<(Point, usize)> {
    let mut points: Vec<Point> = Vec::new();
    // The successor list must be in _reading order_
    add_empty_point(Point { x: p.x, y: p.y - 1 }, &mut points, cavern);
    add_empty_point(Point { x: p.x - 1, y: p.y }, &mut points, cavern);
    add_empty_point(Point { x: p.x, y: p.y + 1 }, &mut points, cavern);
    add_empty_point(Point { x: p.x + 1, y: p.y }, &mut points, cavern);
    points.into_iter().map(|p| (p, 1)).collect()
}

/// Returns the best movement for a unit. It finds the closest enemy unit with available attack
/// squares. If two paths are equally good then reading distance breaks the tie.
/// If no valid position is valid then None is returned.
#[inline(always)]
fn find_best_movement(cavern: &Cavern, unit: &Unit) -> Option<Point> {
    let mut all_path_options: Vec<(Point, (Vec<Point>, usize))> = Vec::new();
    for goal in get_unit_kind_neighbors(&cavern, unit.get_enemy_kind()) {
        let paths = yen(
            &unit.pos,
            |p| cavern_successors(&cavern, p),
            |p| *p == goal,
            4,
        );
        paths
            .iter()
            .for_each(|p| all_path_options.push((goal, p.clone())));
    }
    // Sort by Distance, then Goal (reading order) if there is a tie.
    all_path_options.sort_by_key(|option| (option.1.1, option.0));

    if let Some((_goal, (path, _cost))) = all_path_options.first() {
        Some(*path.get(1).unwrap())
    } else {
        None
    }
}

#[inline(always)]
fn debug(cavern: &Cavern) {
    for y in 0..=cavern.keys().max_by_key(|p| p.y).unwrap().y {
        // Display Battlefield
        for x in 0..=cavern.keys().max_by_key(|p| p.x).unwrap().x {
            if let Some(unit) = cavern.get(&Point{x, y}) {
                /*print!("{}", match unit.kind {
                    Elf => '🧝',
                    Goblin => '👹',
                    Wall => '🪨'
                });
                 */
                print!("{}", match unit.kind {
                    Elf => 'E',
                    Goblin => 'G',
                    Wall => '#'
                });
            } else {
                // print!("{}", '⬛');
                print!("{}", ' ');
            }
        }
        print!("   ");
        // Display Unit Health
        for x in 0..cavern.keys().max_by_key(|p| p.x).unwrap().x {
            if let Some(unit) = cavern.get(&Point{x, y}) {
                if unit.kind == Wall {
                    continue;
                }
                match unit.kind {
                    Elf => print!("E({}) ", unit.hp),
                    Goblin =>  print!("G({}) ", unit.hp),
                    Wall => {},
                }
            }
        }
        print!("\n");
    }
    print!("\n");
}


pub fn part_one(input: &str) -> Option<usize> {
    let mut cavern: Cavern = new_cavern(input);
    debug(&cavern);
    let mut rounds = 0;
    loop {
        if rounds == 500 {
            println!("Breaking...");
            break;
        }
        print!("====ROUND {rounds}=====\n");
        let mut unit_points = cavern.keys().map(|p| *p).collect::<Vec<Point>>();
        unit_points.sort();
        for point in unit_points {
            if let Some(unit) =  cavern.get(&point) {
                let unit = unit.clone();
                if unit.kind == Wall {
                    continue;
                }
                print!("TURN: {:?}\n", unit);
                // First we try to attack
                if let Some(victor) = finished_combat(&cavern) {
                    return Some(score_game(&cavern, victor, rounds));
                }
                if attempt_attack(&unit, &mut cavern) {
                    print!("\n");
                    continue;
                }
                // Then movement happens if there was no attack. There might not be a best move.
                if let Some(new_point) = find_best_movement(&cavern, &unit) {
                    let mut removed_unit = cavern.remove(&point).unwrap();
                    removed_unit.pos = new_point;
                    cavern.insert(new_point, removed_unit);
                    print!(
                        "{:?} moved from {},{} to {},{}\n",
                        unit.kind, unit.pos.x, unit.pos.y, removed_unit.pos.x, removed_unit.pos.y
                    );
                    // Moved units are allowed to attempt another attack
                    attempt_attack(&removed_unit, &mut cavern);
                    if let Some(victor) = finished_combat(&cavern) {
                        return Some(score_game(&cavern, victor, rounds));
                    }
                }
                print!("\n");
            }
        }

        debug(&cavern);
        rounds += 1;
        print!("\n");
    }
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
    /// Tests the attack priorities for a mob. In this simulation the elf is attacking three
    /// different goblins.
    fn test_attempt_attack() {
        let input = "#######
#.GEG.#
#..G#.#
#.G.#G#
#######";
        let mut cavern = new_cavern(input);
        let elf = cavern.get(&Point { x: 3, y: 1 }).unwrap().clone();

        const GOBLIN_LEFT: Point = Point { x: 2, y: 1 };
        const GOBLIN_DOWN: Point = Point { x: 3, y: 2 };
        const GOBLIN_RIGHT: Point = Point { x: 4, y: 1 };

        attempt_attack(&elf, &mut cavern);
        assert_eq!(cavern.get(&GOBLIN_LEFT).unwrap().hp, 197);
        assert_eq!(true, attempt_attack(&elf, &mut cavern));
        assert_eq!(true, attempt_attack(&elf, &mut cavern));
        assert_eq!(true, attempt_attack(&elf, &mut cavern));
        assert_eq!(cavern.get(&GOBLIN_LEFT).unwrap().hp, 188);

        // Manually tweak HP of the bottom goblin
        cavern.get_mut(&GOBLIN_DOWN).unwrap().hp = 100;
        assert_eq!(true, attempt_attack(&elf, &mut cavern));
        assert_eq!(cavern.get(&GOBLIN_DOWN).unwrap().hp, 97);
        // Now tweak Goblin at the right to have the same HP as the lower goblin. It shoudld take
        // the damage due to reading order.
        cavern.get_mut(&GOBLIN_RIGHT).unwrap().hp = 97;
        assert_eq!(true, attempt_attack(&elf, &mut cavern));
        assert_eq!(cavern.get(&GOBLIN_RIGHT).unwrap().hp, 94);
        // Delete the goblins and now attacks should return false
        cavern.remove(&GOBLIN_LEFT).unwrap();
        cavern.remove(&GOBLIN_RIGHT).unwrap();
        cavern.remove(&GOBLIN_DOWN).unwrap();
        assert_eq!(false, attempt_attack(&elf, &mut cavern));
    }

    #[test]
    /// Tests a cavern to find adjacent cells. This cavern is taken from the examples provided.
    fn test_get_unit_kind_neighbors() {
        let input = "#######
#E..G.#
#...#.#
#.G.#G#
#######";
        let mut cavern = new_cavern(input);
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
        // Remove the Elf and ensure that there are no targets
        cavern.remove(&Point { y: 1, x: 1 });
        assert_eq!(get_unit_kind_neighbors(&cavern, Elf), vec![]);
    }

    #[test]
    fn test_part_one() {
        #[cfg_attr(rustfmt, rustfmt_skip)]
        let trials = [(
"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######", 27730),(
"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######", 36334),(
"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######", 39514),(
"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######", 27755), (
"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######", 28944), (
"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########", 18740)];
        // let input = aoc::read_file("examples", 15);
        for (input, score) in trials {
            assert_eq!(part_one(&input), Some(score));
        }
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
