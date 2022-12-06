use itertools::Itertools;

fn find_unique_marker_offset(input: &str, window_size: usize) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    let windows = chars.windows(window_size);
    let mut frame_start_offset: usize = window_size;

    for frame in windows {
        if frame.iter().all_unique() {
            break;
        } else {
            frame_start_offset += 1;
        }
    }
    frame_start_offset
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_unique_marker_offset(input, 4) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_unique_marker_offset(input, 14) as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
