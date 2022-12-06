use aoc::helpers::parse_text_matrix;
use std::collections::VecDeque;

/// A stack move operation
#[derive(Debug)]
struct CraneMove {
    quantity: usize,
    source: usize,
    target: usize,
}

impl CraneMove {
    pub fn from_str(input: &str) -> Self {
        let splits = input.split(' ').collect::<Vec<&str>>();
        Self {
            quantity: splits.get(1).unwrap().parse::<usize>().unwrap(),
            source: splits.get(3).unwrap().parse::<usize>().unwrap(),
            target: splits.get(5).unwrap().parse::<usize>().unwrap(),
        }
    }

    /// Execute the crane operation. Converts the 0-indexed stack we have to a 1-indexed crane
    /// target stack.
    pub fn operation_crane_9000(self, stacks: &mut [VecDeque<char>]) {
        for _ in 0..self.quantity {
            let container = stacks.get_mut(self.source - 1).unwrap().pop_back().unwrap();
            stacks
                .get_mut(self.target - 1)
                .unwrap()
                .push_back(container);
        }
    }

    pub fn operation_crane_9001(self, stacks: &mut [VecDeque<char>]) {
        let mut crane_cargo = VecDeque::with_capacity(30);

        for _ in 0..self.quantity {
            let container = stacks.get_mut(self.source - 1).unwrap().pop_back().unwrap();
            crane_cargo.push_front(container);
        }
        for container in crane_cargo {
            stacks
                .get_mut(self.target - 1)
                .unwrap()
                .push_back(container);
        }
    }
}

fn build_moves(input: &str) -> Vec<CraneMove> {
    input
        .lines()
        .filter(|line| line.contains('m'))
        .map(CraneMove::from_str)
        .collect::<Vec<CraneMove>>()
}

fn get_top_of_stacks(stacks: Vec<VecDeque<char>>) -> String {
    stacks
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.back().unwrap())
        .collect()
}

/// Run the crane operations then find the top contents of each stack
pub fn part_one(input: &str) -> Option<String> {
    let mut stacks = parse_text_matrix(input, '[', 4, 1);
    for operation in build_moves(input) {
        operation.operation_crane_9000(&mut stacks);
    }
    Some(get_top_of_stacks(stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stacks = parse_text_matrix(input, '[', 4, 1);
    for operation in build_moves(input) {
        operation.operation_crane_9001(&mut stacks);
    }
    Some(get_top_of_stacks(stacks))
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crane_move_from_str() {
        let crane = CraneMove::from_str("move 1 from 2 to 1");
        let expected = CraneMove {
            quantity: 1,
            source: 2,
            target: 1,
        };
        assert_eq!(expected.quantity, crane.quantity);
        assert_eq!(expected.source, crane.source);
        assert_eq!(expected.target, crane.target);
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
