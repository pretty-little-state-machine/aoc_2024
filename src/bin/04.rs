/// Cleaning range for each elf. Start and end are both inclusive.
#[derive(Copy, Clone, Debug)]
struct CleaningRange {
    start: u32,
    end: u32,
}

impl CleaningRange {
    /// Parses an elf range str reference into a CleaningRange struct.
    pub fn from_str(input: &str) -> CleaningRange {
        let integers: Vec<u32> = input
            .split('-')
            .map(|x| x.parse::<u32>().expect("Cleaning range must be an integer"))
            .collect();
        CleaningRange {
            start: *integers
                .first()
                .expect("Cleaning range must have two values"),
            end: *integers
                .get(1)
                .expect("Cleaning range must have two values"),
        }
    }

    /// Returns true if one cleaning range is fully covered by another.
    pub fn fully_overlap(a: &CleaningRange, b: &CleaningRange) -> bool {
        (a.start <= b.start && a.end >= b.end) || (b.start <= a.start && b.end >= a.end)
    }

    /// Returns true if two cleaning ranges overlap in any way.
    pub fn partial_overlap(a: &CleaningRange, b: &CleaningRange) -> bool {
        if a.start <= b.start {
            a.end >= b.start
        } else {
            b.end >= a.start
        }
    }
}

fn parse_input_line(input: &str) -> (CleaningRange, CleaningRange) {
    let ranges = input
        .split(',')
        .map(CleaningRange::from_str)
        .collect::<Vec<CleaningRange>>();
    (*ranges.first().unwrap(), *ranges.get(1).unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .lines()
        .filter(|line| {
            let (a, b) = parse_input_line(line);
            CleaningRange::fully_overlap(&a, &b)
        })
        .count() as u32;
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total = input
        .lines()
        .filter(|line| {
            let (a, b) = parse_input_line(line);
            CleaningRange::partial_overlap(&a, &b)
        })
        .count() as u32;
    Some(total)
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_range_from_str() {
        let clean_range = CleaningRange::from_str("34-83");
        assert_eq!(34, clean_range.start);
        assert_eq!(83, clean_range.end);
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
