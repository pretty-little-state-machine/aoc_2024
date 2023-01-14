use itertools::{enumerate, Itertools};

#[inline(always)]
fn remove_shared_letter(a: &[char], b: &[char]) -> String {
    let mut tmp: Vec<char> = Vec::with_capacity(30);
    for (offset, letter) in enumerate(b) {
        if a[offset] == *letter {
            tmp.push(*letter);
        }
    }
    tmp.iter().collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut two_counts = 0;
    let mut three_counts = 0;
    for line in input.lines() {
        let counts = line.chars().counts();
        two_counts += counts.values().contains(&2) as u32;
        three_counts += counts.values().contains(&3) as u32;
    }
    Some(two_counts * three_counts)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut letters: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut answer: String = "".to_string();
    'outer: while let Some(cursor) = letters.pop() {
        for line in &letters {
            let mut matches: usize = 0;
            for (offset, letter) in enumerate(line) {
                matches += (cursor[offset] != *letter) as usize;
            }
            if matches == 1 {
                answer = remove_shared_letter(&cursor, line);
                break 'outer;
            }
        }
    }
    Some(answer)
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
        assert_eq!(part_one(&input), Some(12));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some("fgij".to_string()));
    }
}
