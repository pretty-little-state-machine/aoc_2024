use itertools::Itertools;

fn build_elf_bags(input: &str) -> Vec<u32> {
    let mut bags: Vec<u32> = Vec::with_capacity(500);
    let mut total: u32 = 0;

    for line in input.split('\n') {
        match line.trim_end().parse::<u32>() {
            Ok(calories) => total += calories,
            Err(_) => {
                bags.push(total);
                total = 0
            }
        }
    }
    bags.push(total);
    bags
}

/// Find the Elf carrying the most Calories.
/// How many total Calories is that Elf carrying?
pub fn part_one(input: &str) -> Option<u32> {
    let max_calories = build_elf_bags(input).into_iter().k_largest(1);
    Some(max_calories.sum())
}

/// Find the top three Elves carrying the most Calories.
/// How many Calories are those Elves carrying in total?
pub fn part_two(input: &str) -> Option<u32> {
    let bags = build_elf_bags(input).into_iter().k_largest(3);
    Some(bags.sum())
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
