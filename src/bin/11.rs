use itertools::Itertools;
use std::collections::VecDeque;

type Monkeys = Vec<Monkey>;

struct Toss {
    worry_level: usize,
    target_monkey: usize,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    number_of_inspections: usize,
    operation: char,
    parameter: Option<usize>,
    divisible: usize,
    target_monkey_true: usize,
    target_monkey_false: usize,
}

/// Inspects an item, modifies it's value and assigns it to the new monkey if applicable.
fn run_inspections(
    monkey_id: usize,
    monkeys: &mut [Monkey],
    worry_reducer: usize,
    cycle_modulo: usize,
) -> Vec<Toss> {
    let monkey = monkeys.get_mut(monkey_id).unwrap();
    let mut tosses = Vec::new();
    for _ in 0..monkey.items.len() {
        monkey.number_of_inspections += 1;
        let old_worry_level = monkey.items.pop_front().unwrap();

        let b = if monkey.parameter.is_some() {
            monkey.parameter.unwrap()
        } else {
            old_worry_level
        };

        let worry_level = if monkey.operation == '+' {
            ((old_worry_level + b) / worry_reducer) % cycle_modulo
        } else {
            ((old_worry_level * b) / worry_reducer) % cycle_modulo
        };

        let target_monkey = if worry_level % monkey.divisible == 0 {
            monkey.target_monkey_true
        } else {
            monkey.target_monkey_false
        };

        tosses.push(Toss {
            worry_level,
            target_monkey,
        })
    }
    tosses
}

impl Monkey {
    pub fn from_str(input: Vec<&str>) -> Self {
        Self {
            items: Self::extract_items(input.get(1).unwrap()),
            number_of_inspections: 0,
            operation: Self::extract_operation(input.get(2).unwrap()),
            parameter: Self::extract_parameter(input.get(2).unwrap()),
            divisible: Self::extract_usize(input.get(3).unwrap(), 21),
            target_monkey_true: Self::extract_usize(input.get(4).unwrap(), 29),
            target_monkey_false: Self::extract_usize(input.get(5).unwrap(), 30),
        }
    }
    fn extract_items(input: &str) -> VecDeque<usize> {
        input.chars().collect::<Vec<char>>()[18..]
            .iter()
            .collect::<String>()
            .split(", ")
            .map(|item| item.parse::<usize>().unwrap())
            .collect()
    }
    fn extract_operation(input: &str) -> char {
        *input.chars().collect::<Vec<char>>().get(23).unwrap()
    }
    fn extract_parameter(input: &str) -> Option<usize> {
        if let Ok(value) = input.chars().collect::<Vec<char>>()[25..]
            .iter()
            .collect::<String>()
            .parse::<usize>()
        {
            Some(value)
        } else {
            None
        }
    }
    fn extract_usize(input: &str, offset: usize) -> usize {
        input.chars().collect::<Vec<char>>()[offset..]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    }
}

fn get_monkey_business(monkeys: &Monkeys) -> usize {
    monkeys
        .iter()
        .map(|m| m.number_of_inspections)
        .k_largest(2)
        .reduce(|a, x| a * x)
        .unwrap()
}

fn run_rounds(monkeys: &mut Monkeys, rounds: usize, worry_division: usize, cycle_modulo: usize) {
    for _ in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            let tosses = run_inspections(monkey_idx, monkeys, worry_division, cycle_modulo);
            for toss in tosses {
                monkeys
                    .get_mut(toss.target_monkey)
                    .unwrap()
                    .items
                    .push_back(toss.worry_level);
            }
        }
    }
}

fn build_monkeys(input: &str) -> Monkeys {
    let mut monkeys = Vec::new();
    for block in &input.lines().chunks(7) {
        let block: Vec<_> = block.collect();
        monkeys.push(Monkey::from_str(block));
    }
    monkeys
}

fn get_cycle_modulo(monkeys: &Monkeys) -> usize {
    monkeys
        .iter()
        .map(|m| m.divisible)
        .reduce(|acc, d| acc * d)
        .unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = build_monkeys(input);
    let cycle_modulo = get_cycle_modulo(&monkeys);
    run_rounds(&mut monkeys, 20, 3, cycle_modulo);
    Some(get_monkey_business(&monkeys))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = build_monkeys(input);
    let cycle_modulo = get_cycle_modulo(&monkeys);
    run_rounds(&mut monkeys, 10_000, 1, cycle_modulo);
    Some(get_monkey_business(&monkeys))
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
    fn test_part_one() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
