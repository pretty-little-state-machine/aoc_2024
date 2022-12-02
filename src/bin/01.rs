#[derive(Default, Debug)]
struct ElfBags {
    calories: u32,
}

fn build_elf_bags(input: &str) -> Vec<ElfBags> {
    let mut bags: Vec<ElfBags> = Vec::new();
    bags.push(ElfBags::default());

    for line in input.split('\n') {
        match line.trim_end().parse::<u32>() {
            Ok(calories) => bags.last_mut().unwrap().calories += calories,
            Err(_) => bags.push(ElfBags::default()),
        }
    }
    bags
}

/// Find the Elf carrying the most Calories.
/// How many total Calories is that Elf carrying?
pub fn part_one(input: &str) -> Option<u32> {
    let bags = build_elf_bags(input);
    let max_calories = bags.iter().max_by_key(|b| b.calories).unwrap().calories;
    Some(max_calories)
}

/// Find the top three Elves carrying the most Calories.
/// How many Calories are those Elves carrying in total?
pub fn part_two(input: &str) -> Option<u32> {
    let mut bags = build_elf_bags(input);
    bags.sort_by_key(|b| b.calories);
    bags.reverse();
    bags.truncate(3);
    let total_cals = bags.iter().map(|b| b.calories).sum();
    Some(total_cals)
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
