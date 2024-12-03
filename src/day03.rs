extern crate core;

use crate::day03::Instruction::Mul;
use crate::DayResult;
use logos::{Lexer, Logos};
use std::time::Instant;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let instructions = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&instructions).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&instructions).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

fn decode_mul(lex: &mut Lexer<Instruction>) -> Option<(usize, usize)> {
    let slice = lex.slice();
    let nums = slice
        .replace("mul(", "")
        .replace(")", "")
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    Some((nums[0], nums[1]))
}

#[derive(Debug, Logos, PartialEq)]
enum Instruction {
    #[regex(r"mul\([0-9]{1,3},[0-9]{1,3}\)", decode_mul)]
    Mul((usize, usize)),
    #[token("do()")]
    Do,
    #[token("don't()")]
    Dont,
}

fn parse(input: &str) -> Vec<Instruction> {
    Instruction::lexer(input)
        .filter_map(|x| x.ok())
        .collect::<Vec<_>>()
}

fn part_1(instructions: &Vec<Instruction>) -> usize {
    let mut sum = 0;
    for i in instructions {
        if let Mul((a, b)) = i {
            sum += a * b
        }
    }
    sum
}

fn part_2(instructions: &Vec<Instruction>) -> usize {
    let mut sum = 0;
    let mut enabled = true;
    for i in instructions {
        match i {
            Mul((a, b)) => {
                if enabled {
                    sum += a * b
                }
            }
            Instruction::Do => enabled = true,
            Instruction::Dont => enabled = false,
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let x = parse(&input);
        assert_eq!(part_1(&x), 161);
    }

    #[test]
    fn test_part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let x = parse(&input);
        assert_eq!(part_2(&x), 48);
    }
}
