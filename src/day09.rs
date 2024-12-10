extern crate core;

use crate::DayResult;
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use std::collections::VecDeque;
use std::thread::current;
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let mut data = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&mut data.clone()).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&mut data.clone()).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

#[derive(Debug, Default, Clone)]
struct File {
    id: usize,
}

#[derive(Debug, Clone)]
struct Disk {
    blocks: Vec<Option<File>>,
    file_sizes: FxHashMap<usize, usize>,
}

impl Default for Disk {
    fn default() -> Disk {
        Disk {
            blocks: Vec::with_capacity(100_000),
            file_sizes: FxHashMap::default(),
        }
    }
}

fn parse(input: &str) -> Disk {
    let mut disk = Disk::default();
    let diskmap: Vec<usize> = input
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    let mut file_index = 0;
    diskmap.chunks(2).for_each(|x| {
        if let Some(file_size) = x.first() {
            disk.file_sizes.insert(file_index, *file_size);
            for _ in 0..*file_size {
                disk.blocks.push(Some(File { id: file_index }));
            }
        }
        file_index += 1;
        if let Some(free_size) = x.get(1) {
            for _ in 0..*free_size {
                disk.blocks.push(None);
            }
        }
    });
    disk
}

#[inline(always)]
fn fragment(d: &mut Disk) {
    let mut free_spaces = VecDeque::new();
    d.blocks.iter().enumerate().for_each(|(i, block)| {
        if block.is_none() {
            free_spaces.push_front(i);
        }
    });
    let moves_to_make = free_spaces.len() - 1;
    for (move_count, i) in (0..d.blocks.len()).rev().enumerate() {
        if let Some(f) = &d.blocks[i] {
            if let Some(space) = free_spaces.pop_back() {
                d.blocks.swap(space, i);
                free_spaces.push_front(i); // Add the reclaimed space
            }
        }
        if move_count == moves_to_make {
            break;
        }
    }
}

fn defrag(d: &mut Disk) {
    let mut free_spaces: VecDeque<(usize, usize)> = VecDeque::new(); // (Index, Length)
    let mut free_idx = None;
    let mut free_len = 0;
    d.blocks
        .iter()
        .enumerate()
        .for_each(|(i, block)| match (block, free_idx) {
            (None, None) => {
                free_idx = Some(i);
                free_len = 1;
            }
            (Some(_), Some(x)) => {
                free_spaces.push_front((x, free_len));
                free_idx = None;
                free_len = 0;
            }
            (None, Some(_)) => {
                free_len += 1;
            }
            _ => {}
        });

    let mut current_file_id = usize::MAX;
    let mut files_moved: FxHashSet<usize> = FxHashSet::default();

    for (i) in (0..d.blocks.len()).rev() {
        if let Some(f) = &d.blocks[i] {
            if f.id != current_file_id {
                println!("Testing File: {}, {:?}", f.id, free_spaces.iter().last());
                current_file_id = f.id;
                if let Some((space_idx, space_len)) = free_spaces.iter().last() {
                    if space_len >= d.file_sizes.get(&f.id).unwrap() {
                        println! {"Can move File ID: {} to Space Idx {}", f.id, space_idx}
                    }
                }
            }
        }
    }

    println!("{:?}", d.file_sizes);
    fragment(d);
}

fn part_1(d: &mut Disk) -> usize {
    fragment(d);
    d.blocks
        .iter()
        .enumerate()
        .map(|(idx, f)| match f {
            Some(f) => idx * f.id,
            None => 0,
        })
        .sum()
}

fn part_2(d: &mut Disk) -> usize {
    defrag(d);

    /*d.blocks.iter().for_each(|b| {
        match b {
            Some(f) => print!("{}", f.id),
            None => print!("."),
        }
    });

     */
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("./src/example/day_09.txt").expect("File not found.");
        let mut x = parse(&input);
        assert_eq!(part_1(&mut x), 1928);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./src/example/day_09.txt").expect("File not found.");
        let mut x = parse(&input);
        assert_eq!(part_2(&mut x), 34);
    }
}
