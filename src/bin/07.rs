use crate::Status::{Done, InProgress, Pending};
use itertools::all;
use std::collections::BTreeMap;

type StepArena = BTreeMap<char, Step>;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
enum Status {
    #[default]
    Pending,
    InProgress,
    Done,
}

#[derive(Debug, Default, Clone)]
struct Step {
    id: char,
    status: Status,
    requires: Vec<char>,
    time: usize,
}

impl Step {
    /// Builds a new step with the time to build based on the char.
    /// ASCII Codes: A-Z == 65-90
    fn new(id: char) -> Self {
        Self {
            id,
            status: Pending,
            requires: Vec::default(),
            time: ((id as u8) - 64) as usize + 60,
        }
    }

    fn can_be_done(&self, arena: &StepArena) -> bool {
        all(
            self.requires
                .iter()
                .map(|c| arena.get(c).unwrap().status == Done),
            |b| b,
        )
    }
}

#[inline(always)]
fn line_to_chars(input: &str) -> (char, char) {
    let bytes = input.as_bytes();
    (bytes[5] as char, bytes[36] as char)
}

/// Builds the Steps in an Arena.
fn build_step_arena(input: &str) -> StepArena {
    let mut arena = BTreeMap::new();
    for line in input.lines() {
        let (required, target) = line_to_chars(line);
        arena.entry(required).or_insert(Step::new(required));
        arena.entry(target).or_insert(Step::new(target));
    }
    arena
}

fn hydrate_requirements(input: &str, arena: &mut StepArena) {
    for line in input.lines() {
        let (required, target) = line_to_chars(line);
        arena.get_mut(&target).unwrap().requires.push(required);
    }
}

fn find_starting_point(arena: &StepArena) -> char {
    *arena
        .iter()
        .map(|(k, v)| (k, v.requires.len()))
        .min_by_key(|(_, v)| *v)
        .unwrap()
        .0
}

pub fn part_one(input: &str) -> Option<String> {
    let mut order: Vec<char> = Vec::new();
    let mut arena = build_step_arena(input);
    hydrate_requirements(input, &mut arena);
    let start = find_starting_point(&arena);
    order.push(start);
    arena.get_mut(&start).unwrap().status = Done;
    loop {
        let working_arena = arena.clone();
        for (_, step) in arena.iter_mut() {
            if step.status == Done {
                continue;
            }
            if step.can_be_done(&working_arena) {
                step.status = Done;
                order.push(step.id);
                break;
            }
        }
        if order.len() == arena.len() {
            break;
        }
    }
    let answer: String = order.iter().collect();
    Some(answer)
}

fn all_jobs_done(arena: &StepArena) -> bool {
    all(arena.iter().map(|(_, v)| v.status == Done), |b| b)
}

pub fn part_two(input: &str) -> Option<usize> {
    const NUM_WORKERS: usize = 5;
    let mut arena = build_step_arena(input);
    let arena_keys = arena.keys().copied().collect::<Vec<char>>();
    hydrate_requirements(input, &mut arena);

    let start = find_starting_point(&arena);
    arena.get_mut(&start).unwrap().status = InProgress;
    let mut elapsed: usize = 0;
    let mut jobs_in_progress: usize = 1;

    loop {
        // First we tick progress jobs
        for key in &arena_keys {
            let step = &mut arena.get_mut(key).unwrap();
            if step.status == InProgress {
                step.time = step.time.saturating_sub(1);
                if step.time == 0 {
                    step.status = Done;
                    jobs_in_progress = jobs_in_progress.saturating_sub(1);
                }
            }
        }
        // Now we allocate work in case a higher letter caused a lower one to be available
        for key in &arena_keys {
            let working_arena = arena.clone();
            let step = &mut arena.get_mut(key).unwrap();
            if step.status == Pending
                && jobs_in_progress < NUM_WORKERS
                && step.can_be_done(&working_arena)
            {
                jobs_in_progress += 1;
                step.status = InProgress;
            }
        }
        if all_jobs_done(&arena) {
            break;
        }
        elapsed += 1;
    }
    Some(elapsed)
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some("CABDFE".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(15));
    }
}
