struct Item {
    id: usize,
    worry_level: usize,
    current_monkey: usize,
}

struct Monkey {
    operation: char,
    parameter: String,
    test_division: usize,
    target_monkey_true: usize,
    target_monkey_false: usize,
}

impl Item {
    /// Inspects an item, modifies it's value and assigns it to the new monkey if applicable.
    fn run_inspection(&mut self, monkeys: &Vec<Monkey>) {
        let monkey = monkeys.get(monkeys.current_monkey).unwrap();
        let b = if monkey.parameter.starts_with('o') {
            monkeys.worry_level
        } else {
            monkey.parameter.parse::<usize>().unwrap()
        };
        monkeys.worry_level = if monkey.operation == '+' {
            (monkeys.worry_level + b) / 3
        } else {
            (monkeys.worry_level * b) / 3
        };
        if monkeys.worry_level % monkey.test_division == 0 {
            monkeys.current_monkey = monkey.target_monkey_true
        } else {
            monkeys.current_monkey = monkey.target_monkey_fals e
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
