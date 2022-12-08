#![feature(slice_as_chunks)]

use itertools::{all, Itertools};

/// Builds the forest from the input data.
/// Optimization: Pre-allocating the vector lets us match on parse unwrapping in a for-loop instead
/// of a `filter(is_digit())->map->collect` style solution which saves ~300 usecs on full input.
pub fn build_forest(input: &str) -> Vec<usize> {
    let mut forest = Vec::with_capacity(100 * 100);
    input
        .chars()
        .for_each(|c| match c.to_string().parse::<usize>() {
            Ok(value) => forest.push(value),
            Err(_) => (),
        });
    forest
}

fn is_visible_from_north(forest: &Vec<usize>, x: usize, y: usize, width: usize) -> bool {
    let tree_column = forest.iter().skip(x).step_by(width).copied().collect::<Vec<usize>>();
    let tree_height = tree_column.get(y).unwrap();
    if y > 0 {
        let trees_to_edge = tree_column.get(..y).unwrap();
        all(trees_to_edge, |tree_in_path| tree_in_path < tree_height)
    } else {
        true
    }
}

fn is_visible_from_south(forest: &Vec<usize>, x: usize, y: usize, width: usize) -> bool {
    let tree_column = forest.iter().skip(x).step_by(width).copied().collect::<Vec<usize>>();
    let tree_height = tree_column.get(y).unwrap();
    let trees_to_edge = tree_column.get(y+1..).unwrap();
    all(trees_to_edge, |tree_in_path| tree_in_path < tree_height)
}

fn is_visible_from_east(forest: &Vec<usize>, x: usize, y: usize, width: usize) -> bool {
    let offset = y * width;
    let tree_row = forest.get(offset..offset + width).unwrap();
    let tree_height = tree_row.get(x).unwrap();

    let trees_to_edge = tree_row.get(x + 1..).unwrap();
    all(trees_to_edge, |tree_in_path| tree_in_path < tree_height)
}

fn is_visible_from_west(forest: &Vec<usize>, x: usize, y: usize, width: usize) -> bool {
    let offset = y * width;
    let tree_row = forest.get(offset..offset + width).unwrap();
    let tree_height = tree_row.get(x).unwrap();

    if x > 0 {
        let trees_to_edge = tree_row.get(..x).unwrap();
        all(trees_to_edge, |tree_in_path| tree_in_path < tree_height)
    } else {
        true
    }
}

fn is_visible_from_edges(forest: &Vec<usize>, x: usize, y: usize, width: usize) -> bool {
    visible_from_east(&forest, x, y, width)
        || visible_from_west(&forest, x, y, width)
        || visible_from_south(&forest, x, y, width)
        || visible_from_north(&forest, x, y, width)
}

pub fn part_one(input: &str) -> Option<u32> {
    // This is safe since our input data is a square grid
    let width = input.lines().count();
    let forest = build_forest(input);
    let mut total = 0;
    for y in 0..width {
        for x in 0..width {
            if is_visible_from_edges(&forest, x, y, width) {
                total += 1;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
