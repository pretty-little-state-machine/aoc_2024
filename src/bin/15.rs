use crate::UnitKind::{Elf, Goblin, Wall};
use itertools::Itertools;
use pathfinding::prelude::bfs;
use std::collections::BTreeMap;
#[allow(unused_imports)] // Visualization uses sleep
use std::thread::sleep;
#[allow(unused_imports)] // Visualization uses sleep
use std::time;

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
    attack: usize,
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
fn add_cavern_unit(kind: UnitKind, point: Point, cavern: &mut Cavern, elf_attack: usize) {
    let attack = if Elf == kind { elf_attack } else { 3 };
    cavern.insert(
        point,
        Unit {
            kind,
            hp: 200,
            pos: point,
            attack,
        },
    );
}

fn new_cavern(input: &str, elf_attack: usize) -> Cavern {
    let mut cavern = BTreeMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                'E' => add_cavern_unit(Elf, Point { x, y }, &mut cavern, elf_attack),
                'G' => add_cavern_unit(Goblin, Point { x, y }, &mut cavern, elf_attack),
                '#' => add_cavern_unit(Wall, Point { x, y }, &mut cavern, elf_attack),
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
    #[rustfmt::skip]
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
        target.hp = target.hp.saturating_sub(unit.attack);
        if target.hp == 0 {
            cavern.remove(point);
        }
        true
    } else {
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
    if elves.is_empty() {
        Some(Goblin)
    } else if goblins.is_empty() {
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
    total_hp * rounds
}

/// Obtains empty squares around a point. Useful for pathfinding and target detection. All moves
/// have a cost of 1.
#[inline(always)]
fn cavern_successors(cavern: &Cavern, p: &Point) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    // The successor list must be in _reading order_
    add_empty_point(Point { x: p.x, y: p.y - 1 }, &mut points, cavern);
    add_empty_point(Point { x: p.x - 1, y: p.y }, &mut points, cavern);
    add_empty_point(Point { x: p.x + 1, y: p.y }, &mut points, cavern);
    add_empty_point(Point { x: p.x, y: p.y + 1 }, &mut points, cavern);
    // points.into_iter().map(|p| (p, 1)).collect()
    points
}

/// Returns the best movement for a unit. It finds the closest enemy unit with available attack
/// squares. If two paths are equally good then reading distance breaks the tie.
/// If no valid position is valid then None is returned.
#[inline(always)]
fn find_best_movement(cavern: &Cavern, unit: &Unit) -> Option<Point> {
    // If an enemy is in range (distance of 1) then no movement is required
    if get_unit_kind_points(cavern, unit.get_enemy_kind())
        .iter()
        .map(|point| point.distance(&unit.pos))
        .contains(&1)
    {
        return None;
    }

    //  Distance to Goal, Goal Point, Path to Goal
    let mut all_path_options: Vec<(usize, Point, Vec<Point>)> = Vec::new();
    for goal in get_unit_kind_neighbors(cavern, unit.get_enemy_kind()) {
        if let Some(path) = bfs(&unit.pos, |p| cavern_successors(cavern, p), |p| *p == goal) {
            all_path_options.push((path.len(), goal, path));
        }
    }

    // Sort by Distance, then Goal (reading order), then first step!
    all_path_options.sort_by_key(|option| (option.0, option.1));
    if all_path_options.is_empty() {
        None
    } else {
        all_path_options.first().unwrap().2.get(1).copied()
    }
}

#[allow(dead_code)] // For visualizations
#[inline(always)]
fn debug(cavern: &Cavern) {
    for y in 0..=cavern.keys().max_by_key(|p| p.y).unwrap().y {
        // Display Battlefield
        for x in 0..=cavern.keys().max_by_key(|p| p.x).unwrap().x {
            if let Some(unit) = cavern.get(&Point { x, y }) {
                match unit.kind {
                    Elf => print!("\x1b[92mE\x1b[0m"),
                    Goblin => print!("\x1b[91mG\x1b[0m"),
                    Wall => print!("\x1b[90m█\x1b[0m"),
                }
            } else {
                print!(" ");
            }
        }
        print!("   ");
        // Display Unit Health
        for x in 0..cavern.keys().max_by_key(|p| p.x).unwrap().x {
            if let Some(unit) = cavern.get(&Point { x, y }) {
                if unit.kind == Wall {
                    continue;
                }
                match unit.kind {
                    Elf => print!("E({}) ", unit.hp),
                    Goblin => print!("G({}) ", unit.hp),
                    Wall => {}
                }
            }
        }
        println!();
    }
    println!();
}

/// Simulates the battle. Returns the victor and the score
fn run_battle(cavern: &mut Cavern, break_on_elf_death: bool) -> (UnitKind, usize) {
    let mut rounds = 0;
    let num_elves = get_unit_kind_points(cavern, Elf).len();
    loop {
        // Part II states that any Elf death is immediately a goblin victory
        if break_on_elf_death && num_elves != get_unit_kind_points(cavern, Elf).len() {
            return (Goblin, 0);
        }
        let mut unit_points = cavern.keys().copied().collect::<Vec<Point>>();
        unit_points.sort();
        let mut new_points = Vec::new();
        for point in &unit_points {
            if let Some(unit) = cavern.get(point) {
                let unit = *unit;
                if unit.kind == Wall {
                    continue;
                }
                // If a unit moves into a previous unit's place don't double-execute the behavior
                if new_points.contains(point) {
                    continue;
                }
                if let Some(victor) = finished_combat(cavern) {
                    return (victor, score_game(cavern, victor, rounds));
                }
                if let Some(new_point) = find_best_movement(cavern, &unit) {
                    let mut saved_unit = cavern.remove(point).unwrap();
                    saved_unit.pos = new_point;
                    new_points.push(new_point);
                    cavern.insert(new_point, saved_unit);
                    attempt_attack(&saved_unit, cavern);
                } else {
                    attempt_attack(&unit, cavern);
                }
            }
        }

        // sleep(time::Duration::from_millis(125));
        // debug(cavern);
        // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        rounds += 1;
        if let Some(victor) = finished_combat(cavern) {
            return (victor, score_game(cavern, victor, rounds));
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut cavern: Cavern = new_cavern(input, 3);
    let (_, score) = run_battle(&mut cavern, false);
    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut attack = 3;
    loop {
        let mut cavern: Cavern = new_cavern(input, attack);
        let (victor, score) = run_battle(&mut cavern, true);
        if Elf == victor {
            return Some(score);
        }
        attack += 1;
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UnitKind::{Elf, Goblin};

    #[test]
    /// Tests the attack priorities for a mob. In this simulation the elf is attacking three
    /// different goblins.
    fn test_attempt_attack() {
        let input = "#######
#.GEG.#
#..G#.#
#.G.#G#
#######";
        let mut cavern = new_cavern(input, 3);
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
        let mut cavern = new_cavern(input, 3);
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
    /// See https://www.reddit.com/r/adventofcode/comments/a6f100/day_15_details_easy_to_be_wrong_on/
    /// for some more test cases.
    fn test_part_one() {
        #[rustfmt::skip]
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
#########", 18740), (
"####
##E#
#GG#
####", 13400), (
"#####
#GG##
#.###
#..E#
#.#G#
#.E##
#####", 13987)];
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
