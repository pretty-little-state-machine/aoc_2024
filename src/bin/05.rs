use rayon::prelude::*;

/// Polymerizes the molecule.
///
/// ASCII Values as byte:
/// A-Z == 65-90
/// a-z == 97-122
///
/// Optimizations:
///  * Each loop will polymerize all possibilities per line. The `skip_next` variable fixes issues
///    where patterns like `cCc` appear.
#[inline(always)]
fn polymerize(input: &str) -> usize {
    let mut bytes = input.bytes().collect::<Vec<u8>>();
    loop {
        let mut reaction = false;
        let mut working = bytes.clone();
        let mut skip_next = false;

        let mut indexes_to_remove: Vec<usize> = Vec::new();
        for (index, current) in bytes.iter().enumerate() {
            if skip_next {
                skip_next = false;
                continue;
            }
            if let Some(next) = bytes.get(index + 1) {
                if current.abs_diff(*next) == 32 {
                    indexes_to_remove.push(index);
                    reaction = true;
                    skip_next = true;
                }
            }
        }
        // Scrub the indexes while accounting for the fact that vector is shrinking
        for (offset, index) in indexes_to_remove.iter().enumerate() {
            working.remove(index - 2 * offset);
            working.remove(index - 2 * offset);
        }
        bytes = working;
        if !reaction {
            break;
        }
    }
    bytes.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(polymerize(input))
}

pub fn part_two(input: &str) -> Option<usize> {
    let polymer_base = input;
    let solutions = (65..=90).into_par_iter().map(|value| {
        let char_upper = (value as u8) as char;
        let char_lower = ((value + 32) as u8) as char;
        let working = polymer_base.replace([char_upper, char_lower], "");
        polymerize(&working)
    });
    Some(solutions.min().unwrap())
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(4));
    }
}
