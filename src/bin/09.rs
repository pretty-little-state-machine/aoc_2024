use std::collections::VecDeque;

fn run_game(num_players: usize, num_marbles: usize) -> usize {
    let mut scores: Vec<usize> = vec![0; num_players];
    let mut board = VecDeque::with_capacity(num_marbles);

    // Insert the first marble so the rotation can happen
    board.push_back(0);
    for marble in 1..num_marbles {
        // Special case - Multiple of 23
        if 0 == marble % 23 {
            scores[marble % num_players] += marble;
            // Rotate counter-clockwise six times, not seven since we will be removing at the back
            for _ in 0..6 {
                let tmp = board.pop_back().unwrap();
                board.push_front(tmp);
            }
            let removed = board.pop_back().unwrap(); // This normalizes the rotation to 7 times
            scores[marble % num_players] += removed;
        } else {
            // Rotate clockwise to the right twice
            for _ in 0..2 {
                let tmp = board.pop_front().unwrap();
                board.push_back(tmp);
            }
            board.push_front(marble);
        }
    }
    *scores.iter().max().unwrap()
}

fn parse_input(input: &str) -> (usize, usize) {
    let splits = input.split(' ').collect::<Vec<&str>>();
    (
        splits[0].parse::<usize>().unwrap(),
        splits[6].parse::<usize>().unwrap(),
    )
}

pub fn part_one(input: &str) -> Option<usize> {
    let (players, marbles) = parse_input(input);
    Some(run_game(players, marbles))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (players, marbles) = parse_input(input);
    Some(run_game(players, marbles * 100))
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(8317));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(74765078));
    }
}
