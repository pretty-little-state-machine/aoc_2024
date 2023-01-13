use rustc_hash::FxHashSet;

pub fn part_one(input: &str) -> Option<isize> {
    Some(
        input
            .lines()
            .fold(0, |acc, v: &str| acc + v.parse::<isize>().unwrap()),
    )
}

pub fn part_two(input: &str) -> Option<isize> {
    let numbers = input
        .lines()
        .map(|v| v.parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let mut seen_freqs = FxHashSet::with_capacity_and_hasher(150_000, Default::default());
    let mut sum: isize = 0;
    'outer: loop {
        for v in &numbers {
            sum += v;
            if !seen_freqs.insert(sum) {
                break 'outer;
            }
        }
    }
    Some(sum)
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
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(10));
    }
}
