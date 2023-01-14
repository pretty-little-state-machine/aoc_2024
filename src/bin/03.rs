use itertools::all;
use regex::Regex;
use rustc_hash::FxHashSet;
use std::error::Error;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Default, Debug)]
struct Claim {
    id: usize,
    left_margin: usize,
    top_margin: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn new(input: &str) -> Result<Claim, Box<dyn Error>> {
        let re = Regex::new(r"#(\d+).*@\s(\d+),(\d+):\s(\d+)x(\d+)")?;
        if let Some(caps) = re.captures(input) {
            Ok(Claim {
                id: caps[1].parse()?,
                left_margin: caps[2].parse()?,
                top_margin: caps[3].parse()?,
                width: caps[4].parse()?,
                height: caps[5].parse()?,
            })
        } else {
            Err("No regex captures found.".to_string().into())
        }
    }
}

#[inline(always)]
fn overlaps(a: &Claim, b: &Claim) -> bool {
    let a1 = Point {
        x: a.left_margin,
        y: a.top_margin,
    };
    let a2 = Point {
        x: a.left_margin + a.width - 1,
        y: a.top_margin + a.height - 1,
    };
    let b1 = Point {
        x: b.left_margin,
        y: b.top_margin,
    };
    let b2 = Point {
        x: b.left_margin + b.width - 1,
        y: b.top_margin + b.height - 1,
    };
    !(b1.x > a2.x || b2.x < a1.x || b1.y > a2.y || b2.y < a1.y)
}

pub fn part_one(input: &str) -> Option<usize> {
    let claims: Vec<Claim> = input.lines().map(|l| Claim::new(l).unwrap()).collect();
    let mut cloth = FxHashSet::with_capacity_and_hasher(375_000, Default::default());
    let mut overlapping_area: usize = 0;

    for claim in &claims {
        for y in 1..=claim.height {
            for x in 1..=claim.width {
                let point = Point {
                    x: claim.left_margin + x,
                    y: claim.top_margin + y,
                };
                if !cloth.insert(point) {
                    overlapping_area += 1;
                }
            }
        }
    }
    Some(overlapping_area)
}

pub fn part_two(input: &str) -> Option<usize> {
    let claims: Vec<Claim> = input.lines().map(|l| Claim::new(l).unwrap()).collect();
    let mut matching_id: Option<usize> = None;
    for claim in &claims {
        let matches = &claims
            .iter()
            .map(|c| (c.id == claim.id) || (!overlaps(claim, c)))
            .collect::<Vec<bool>>();
        if all(matches, |b| *b) {
            matching_id = Some(claim.id);
            break;
        }
    }
    matching_id
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(4));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(3));
    }
}
