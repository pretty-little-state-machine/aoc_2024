#![feature(let_chains)]

use crate::MonkeyAction::{Add, Divide, Multiply, Subtract, Yell};
use rustc_hash::FxHashMap;

type Monkeys = FxHashMap<String, Monkey>;

#[derive(Clone, Default, Debug)]
struct Monkey {
    action: MonkeyAction,
    original_action: MonkeyAction,
    value: isize,
    left: String,
    right: String,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
enum MonkeyAction {
    #[default]
    Yell,
    Add,
    Divide,
    Multiply,
    Subtract,
}

fn build_monkeys(input: &str, monkeys: &mut Monkeys) {
    let name = input[0..4].to_string();
    let mut monkey = Monkey::default();
    if input.len() == 17 {
        monkey.right = input[6..10].to_string();
        monkey.left = input[13..17].to_string();
        monkey.action = match input.chars().nth(11).unwrap() {
            '+' => Add,
            '/' => Divide,
            '*' => Multiply,
            '-' => Subtract,
            _ => unreachable!("Invalid MonkeyAction Operation"),
        }
    } else {
        monkey.action = Yell;
        monkey.value = input[6..].parse::<isize>().unwrap();
    };
    monkey.original_action = monkey.action.clone();
    monkeys.insert(name, monkey);
}

/// Runs the monkey yelling simulation and returns a hashmap of parents for Part II use.
fn resolve_monkey_shouts(monkeys: &mut Monkeys) -> FxHashMap<String, String> {
    let key_map = monkeys.clone();
    let keys = key_map.keys().collect::<Vec<&String>>();
    let mut parents: FxHashMap<String, String> = FxHashMap::default();

    loop {
        if monkeys.get(&"root".to_string()).unwrap().action == Yell {
            break;
        }
        // Resolve as much as we can each pass
        for key in &keys {
            let m = monkeys.get(*key).unwrap();
            if m.action == Yell {
                continue;
            }

            let a = monkeys.get(&*m.right).unwrap().clone();
            let b = monkeys.get(&*m.left).unwrap().clone();
            parents.insert(
                (*m.right).parse().unwrap(),
                ((*key).clone()).parse().unwrap(),
            );
            parents.insert(
                (*m.left).parse().unwrap(),
                ((*key).clone()).parse().unwrap(),
            );
            let m = monkeys.get_mut(*key).unwrap();

            if a.action == Yell && b.action == Yell {
                match m.action {
                    Yell => (),
                    Add => m.value = a.value + b.value,
                    Divide => m.value = a.value / b.value,
                    Multiply => m.value = a.value * b.value,
                    Subtract => m.value = a.value - b.value,
                }
                m.action = Yell;
            }
        }
    }
    parents
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut monkeys = FxHashMap::default();
    input.lines().for_each(|l| build_monkeys(l, &mut monkeys));
    resolve_monkey_shouts(&mut monkeys);
    Some(monkeys.get(&*"root".to_string()).unwrap().value)
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut monkeys = FxHashMap::default();
    input.lines().for_each(|l| build_monkeys(l, &mut monkeys));
    let parents = resolve_monkey_shouts(&mut monkeys);

    let mut current_child = "humn".to_string();
    let mut path: Vec<String> = Vec::with_capacity(100);
    // path.push(current_child.clone());
    while let Some(parent) = parents.get(&current_child) {
        path.push(parent.clone());
        if parent == "root" {
            break;
        }
        current_child = (*parent.clone()).parse().unwrap();
    }
    // Determine if we need to match left or right side of the overall tree
    let root = monkeys.get(&"root".to_string()).unwrap();
    let child_to_match = if root.left == current_child {
        root.right.clone()
    } else {
        root.left.clone()
    };

    // Now search down and gather the path to HUMN
    let value_to_match = monkeys.get(&child_to_match).unwrap().value;
    let mut last_visited = "humn".to_string();
    // Operations are the computation and a bool that is true if the operation is LHS
    let mut ops: Vec<(MonkeyAction, bool)> = Vec::new();
    let mut values = Vec::new();
    for p in path {
        if "root" == p {
            continue;
        }
        let monkey = monkeys.get(&p).unwrap();
        // Resolve the OPPOSITE side of the tree from the walk up from HUMN for our equation
        if monkey.right == last_visited {
            let monkey_val = monkey.left.clone();
            let value = monkeys.get(&monkey_val).unwrap().value;
            values.push(value);
            match monkey.original_action {
                Yell => (),
                Add => ops.push((Add, true)),
                Divide => ops.push((Divide, true)),
                Multiply => ops.push((Multiply, true)),
                Subtract => ops.push((Subtract, true)),
            }
        } else {
            let monkey_val = monkey.right.clone();
            let value = monkeys.get(&monkey_val).unwrap().value;
            values.push(value);
            match monkey.original_action {
                Yell => (),
                Add => ops.push((Add, false)),
                Divide => ops.push((Divide, false)),
                Multiply => ops.push((Multiply, false)),
                Subtract => ops.push((Subtract, false)),
            }
        };
        last_visited = p;
    }

    // Reverse the path and solve the linear equation
    let mut x = value_to_match;
    for (idx, (action, lhs)) in ops.iter().enumerate().rev() {
        let v = *values.get(idx).unwrap();
        if *lhs {
            match action {
                Yell => (),
                Add => x -= v,
                Divide => x *= v,
                Multiply => x /= v,
                Subtract => x += v,
            }
        } else {
            match action {
                Yell => (),
                Add => x -= v,
                Divide => x = v / x,
                Multiply => x /= v,
                Subtract => x = v - x,
            }
        }
    }
    Some(x)
}

fn main() {
    let input = &aoc::read_file("inputs", 21);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
