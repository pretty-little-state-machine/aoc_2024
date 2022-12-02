use crate::Outcome::{Draw, Loss, Win};
use crate::Shape::{Paper, Rock, Scissors};

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

/// Converts a &str to Shape - Used for Part I
#[inline(always)]
fn str_to_shape(letter: &[u8; 1]) -> Shape {
    match letter {
        b"A" | b"X" => Rock,
        b"B" | b"Y" => Paper,
        b"C" | b"Z" => Scissors,
        _ => unreachable!("Invalid puzzle move input, must be A|B|C|X|Y|Z"),
    }
}

/// Converts a &str to a target outcome - Used for Part II
#[inline(always)]
fn str_to_outcome(letter: &[u8; 1]) -> Outcome {
    match letter {
        b"X" => Loss,
        b"Y" => Draw,
        b"Z" => Win,
        _ => unreachable!("Invalid target outcome, must be X|Y|Z"),
    }
}

/// Returns a shape that will result in the specified match outcome based on the elf's shape.
#[inline(always)]
fn get_move_for_outcome(elf_shape: Shape, outcome: Outcome) -> Shape {
    match outcome {
        Win => match elf_shape {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        },
        Loss => match elf_shape {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        },
        Draw => elf_shape,
    }
}

#[inline(always)]
fn get_round_outcome(elf_shape: Shape, player_shape: Shape) -> Outcome {
    match elf_shape {
        Rock => match player_shape {
            Rock => Draw,
            Paper => Win,
            Scissors => Loss,
        },
        Paper => match player_shape {
            Rock => Loss,
            Paper => Draw,
            Scissors => Win,
        },
        Scissors => match player_shape {
            Rock => Win,
            Paper => Loss,
            Scissors => Draw,
        },
    }
}

/// Use the two columns of input as the two shapes and calculate the total score of the game.
pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let mut chars = line.as_bytes();
        let elf_shape = str_to_shape(&[chars[0]]);
        let player_shape = str_to_shape(&[chars[2]]);

        total += player_shape as u32;
        total += get_round_outcome(elf_shape, player_shape) as u32;
    }
    Some(total)
}

/// Use the first column as the elf's shape and the second column as the strategy to execute.
pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let mut chars = line.as_bytes();
        let elf_shape = str_to_shape(&[chars[0]]);
        let target_outcome = str_to_outcome(&[chars[2]]);

        total += get_move_for_outcome(elf_shape, target_outcome) as u32;
        total += target_outcome as u32;
    }
    Some(total)
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
