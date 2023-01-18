use rustc_hash::FxHashMap;

type Pots = FxHashMap<isize, bool>;

/// A growing rule.
/// Note that only true results are in the example but the real problem can return false!
/// ...## => #
/// ....# => .
#[derive(Debug)]
struct Rule {
    input: [bool; 5],
    output: bool,
}

impl Rule {
    fn new(input: &str) -> Self {
        let chars = input.chars().collect::<Vec<char>>();
        let mut rule_input = [false; 5];
        for i in 0..5 {
            rule_input[i] = decode_char(chars[i]);
        }
        Self {
            input: rule_input,
            output: decode_char(chars[9]),
        }
    }
}

#[inline(always)]
fn decode_char(input: char) -> bool {
    match input {
        '.' => false,
        '#' => true,
        _ => unreachable!("Unknown input"),
    }
}

/// Hashmap is used over a Vec since we can have negative indexes
fn parse_input(input: &str, extra_pots: isize) -> (Pots, Vec<Rule>) {
    const HEADER_SIZE: isize = 15;
    let lines = input.split('\n').collect::<Vec<&str>>();

    let mut pots = FxHashMap::default();
    // Pre-populate the hashmap a large set of empty pots to the left and right
    for i in 0..extra_pots {
        pots.insert(i, false);
        pots.insert(-i, false);
    }
    // Build pots
    let first_line = lines[0];
    for (i, c) in first_line.chars().enumerate() {
        if (i as isize) < HEADER_SIZE {
            continue;
        }
        let index = i as isize - HEADER_SIZE;
        pots.insert(index, decode_char(c));
    }
    // Build Rules
    let mut rules = Vec::new();
    for line in &lines[2..] {
        rules.push(Rule::new(line))
    }
    (pots, rules)
}

fn run_simulation(pots: &mut Pots, rules: &Vec<Rule>, generations: usize) -> isize {
    let min_pot = *pots.keys().min().unwrap();
    let max_pot = *pots.keys().max().unwrap();
    let mut differences: Vec<isize> = Vec::with_capacity(200);
    let mut last_value = sum_pots(pots);

    for g in 0..generations {
        let mut new_pots = pots.clone();
        // Scan across the pots - Note the +/-2 since we have to look to the left and right.
        for key in min_pot + 2..=max_pot - 2 {
            let window = [
                *pots.get(&(key - 2)).unwrap(),
                *pots.get(&(key - 1)).unwrap(),
                *pots.get(&key).unwrap(),
                *pots.get(&(key + 1)).unwrap(),
                *pots.get(&(key + 2)).unwrap(),
            ];
            let mut result = false; // Catch-all since examples only include growing rules
            for rule in rules {
                if window == rule.input {
                    result = rule.output;
                }
            }
            *new_pots.get_mut(&key).unwrap() = result;
        }
        let sum = sum_pots(pots);
        differences.push(sum - last_value);
        last_value = sum;
        *pots = new_pots.clone();

        // Check for convergences between the previous few generations
        if g > 20 {
            let d = differences.iter().rev().collect::<Vec<&isize>>();
            if d[0] == d[1] && d[0] == d[2] {
                return (generations as isize - g as isize) * d[0] + sum;
            }
        } else if g > 200 {
            panic!("Plants are not converging!");
        }
    }
    sum_pots(pots)
}

fn sum_pots(pots: &Pots) -> isize {
    let min_pot = *pots.keys().min().unwrap();
    let max_pot = *pots.keys().max().unwrap();
    let mut sum = 0;
    for i in min_pot..=max_pot {
        if *pots.get(&i).unwrap() {
            sum += i;
        }
    }
    sum
}

pub fn part_one(input: &str) -> Option<isize> {
    let (mut pots, rules) = parse_input(input, 125); // Big enough for part 1
    Some(run_simulation(&mut pots, &rules, 20))
}

pub fn part_two(input: &str) -> Option<isize> {
    let (mut pots, rules) = parse_input(input, 200); // A little more for part 2
    Some(run_simulation(&mut pots, &rules, 50_000_000_000))
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(325));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(999999999374));
    }
}
