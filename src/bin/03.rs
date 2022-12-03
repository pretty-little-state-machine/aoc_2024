use itertools::Itertools;
use std::collections::HashSet;

/// Notes on optimization:
/// - The input values are always ASCII text so string manipulation is avoided for calculating
///   priorities. Numerical offsets are used for fast character to priority mapping.

/// Bag item priority
type Priority = u32;

/// Intersects two hashsets. `a` is left with only the values found in `b`
pub fn intersect(a: &mut HashSet<Priority>, b: &mut HashSet<Priority>) {
    a.retain(|e| b.contains(e));
}

/// Takes a rucksack inventory definition string and returns a vector of **unique** item priorities
/// for each compartment. The sum of compartment item counts is always even.
fn get_compartment_hashsets(input: &str) -> Vec<HashSet<Priority>> {
    let compartment_size = input.bytes().len() / 2;
    input
        .bytes()
        .map(ascii_byte_to_priority)
        .into_iter()
        .chunks(compartment_size)
        .into_iter()
        .map(|c| c.collect::<HashSet<Priority>>())
        .collect()
}

/// Converts a rucksack string into a Hashset of item priorities
fn rucksack_to_priority_set(input: &str) -> HashSet<Priority> {
    input
        .bytes()
        .map(ascii_byte_to_priority)
        .into_iter()
        .collect::<HashSet<Priority>>()
}

/// Maps an item ascii character byte to the corresponding priority value.
#[inline(always)]
fn ascii_byte_to_priority(input: u8) -> Priority {
    (match input {
        65..=90 => input - 38,
        97..=122 => input - 96,
        _ => unreachable!("Invalid ascii character range: {}", input),
    }) as Priority
}

/// Finds the intersection of shared item priorities between the first and second compartments in
/// each bag. The sum of all priority intersections is the puzzle solution.
pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let mut compartments = get_compartment_hashsets(line);
        let (a, b) = compartments.split_at_mut(1);
        intersect(&mut a[0], &mut b[0]);
        total += a[0].iter().sum::<Priority>();
    }
    Some(total)
}

/// Finds the intersection of an item with the same priority across three bags. The sum of all
/// priority intersections is the puzzle solution.
pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let lines = &input.lines().collect::<Vec<&str>>();
    for line in lines.chunks(3) {
        let a = &mut rucksack_to_priority_set(line.first().unwrap());
        let b = &mut rucksack_to_priority_set(line.get(1).unwrap());
        let c = &mut rucksack_to_priority_set(line.get(2).unwrap());
        intersect(a, b);
        intersect(a, c);
        total += a.iter().sum::<Priority>();
    }

    Some(total)
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_priorities() {
        assert_eq!(16, ascii_byte_to_priority("p".as_bytes()[0]));
        assert_eq!(38, ascii_byte_to_priority("L".as_bytes()[0]));
        assert_eq!(42, ascii_byte_to_priority("P".as_bytes()[0]));
        assert_eq!(22, ascii_byte_to_priority("v".as_bytes()[0]));
        assert_eq!(20, ascii_byte_to_priority("t".as_bytes()[0]));
        assert_eq!(19, ascii_byte_to_priority("s".as_bytes()[0]));
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
