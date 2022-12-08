#![feature(slice_as_chunks)]
use itertools::all;
use rayon::prelude::*;

/// Builds the forest from the input data.
/// Optimization: Pre-allocating the vector lets us match on parse unwrapping in a for-loop instead
/// of a `filter(is_digit())->map->collect` style solution which saves ~300 usecs on full input.
pub fn build_forest(input: &str) -> Vec<usize> {
    let mut forest = Vec::with_capacity(100 * 100);
    input.chars().for_each(|c| {
        if let Ok(value) = c.to_string().parse::<usize>() {
            forest.push(value)
        }
    });
    forest
}

#[inline(always)]
fn get_column_trees(forest: &[usize], x: usize, width: usize) -> Vec<usize> {
    forest.iter().skip(x).step_by(width).copied().collect()
}

#[inline(always)]
fn get_row_trees(forest: &[usize], y: usize, width: usize) -> Vec<usize> {
    let offset = y * width;
    forest.get(offset..offset + width).unwrap().to_vec()
}

#[inline(always)]
fn is_visible_from_edges(tree_row: &[usize], tree_column: &[usize], x: usize, y: usize) -> bool {
    let tree_height = tree_row.get(x).unwrap();
    // Checking order: West, East, North, South
    all(tree_row.get(..x).unwrap(), |tree_in_path| {
        tree_in_path < tree_height
    }) || all(tree_row.get(x + 1..).unwrap(), |tree_in_path| {
        tree_in_path < tree_height
    }) || all(tree_column.get(..y).unwrap(), |tree_in_path| {
        tree_in_path < tree_height
    }) || all(tree_column.get(y + 1..).unwrap(), |tree_in_path| {
        tree_in_path < tree_height
    })
}

#[inline(always)]
fn get_score(
    tree_row: &[usize],
    tree_column: &[usize],
    x: usize,
    y: usize,
    tree_height: &usize,
) -> usize {
    let mut east: usize = 0;
    let mut west: usize = 0;
    let mut south: usize = 0;
    let mut north: usize = 0;
    // East
    for t in tree_row.get(..x).unwrap().to_vec().iter().rev() {
        east += 1;
        if t >= tree_height {
            break;
        }
    }
    // West
    for t in tree_row.get(x + 1..).unwrap().to_vec().iter() {
        west += 1;
        if t >= tree_height {
            break;
        }
    }
    // North
    for t in tree_column.get(..y).unwrap().to_vec().iter().rev() {
        north += 1;
        if t >= tree_height {
            break;
        }
    }
    // South
    for t in tree_column.get(y + 1..).unwrap().to_vec().iter() {
        south += 1;
        if t >= tree_height {
            break;
        }
    }
    west * east * north * south
}

pub fn part_one(input: &str) -> Option<u32> {
    // This is safe since our input data is a square grid
    let width = input.lines().count();
    let forest = build_forest(input);
    let answer = forest
        .par_iter()
        .enumerate()
        .map(|(index, _)| {
            let x = index % width;
            let y = index / width;
            if x == 0 || y == 0 || (x == width - 1) || (y == width - 1) {
                true
            } else {
                let tree_row = get_row_trees(&forest, y, width);
                let tree_column = get_column_trees(&forest, x, width);
                is_visible_from_edges(&tree_row, &tree_column, x, y)
            }
        })
        .filter(|visible| *visible)
        .count() as u32;
    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    // This is safe since our input data is a square grid
    let width = input.lines().count();
    let forest = build_forest(input);
    let scores = forest
        .par_iter()
        .enumerate()
        .map(|(index, tree_height)| {
            let x = index % width;
            let y = index / width;
            let tree_row = get_row_trees(&forest, y, width);
            let tree_column = get_column_trees(&forest, x, width);
            get_score(&tree_row, &tree_column, x, y, tree_height)
        })
        .collect::<Vec<usize>>();

    Some(*scores.iter().max().unwrap() as u32)
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
        assert_eq!(part_two(&input), Some(8));
    }
}
