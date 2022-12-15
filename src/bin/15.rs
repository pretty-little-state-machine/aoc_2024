use itertools::{all, any};
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Beacons = HashMap<(isize, isize), Beacon>;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}

struct Beacon {
    location: Point,
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    location: Point,
    beacon_key: (isize, isize),
    range: usize,
}

impl Sensor {
    pub fn new(location: Point, beacon: Point) -> Self {
        Self {
            location,
            beacon_key: (beacon.x, beacon.y),
            range: manhattan(location, beacon),
        }
    }
}

#[inline(always)]
fn manhattan(a: Point, b: Point) -> usize {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as usize
}

/// Returns true if the given point can be covered by the sensor array
#[inline(always)]
fn is_covered(sensors: &[Sensor], location: Point) -> bool {
    any(
        sensors
            .iter()
            .map(|s| s.range >= manhattan(s.location, location))
            .collect::<Vec<bool>>(),
        |b| b,
    )
}

fn build_sensor_array(input: &str) -> (Vec<Sensor>, Beacons) {
    let mut sensors = Vec::new();
    let mut beacons = Beacons::new();
    let re = Regex::new(r".*x=([-\d]+).*y=([-\d]+):.*x=([-\d]+).*y=([-\d]+)").unwrap();
    for cap in re.captures_iter(input) {
        let beacon_location = Point {
            x: cap[3].parse::<isize>().unwrap(),
            y: cap[4].parse::<isize>().unwrap(),
        };
        beacons.insert(
            (beacon_location.x, beacon_location.y),
            Beacon {
                location: beacon_location,
            },
        );
        sensors.push(Sensor::new(
            Point {
                x: cap[1].parse::<isize>().unwrap(),
                y: cap[2].parse::<isize>().unwrap(),
            },
            beacon_location,
        ));
    }
    (sensors, beacons)
}

pub fn part_one(input: &str) -> Option<u32> {
    let row_to_test = 2000000;
    let (sensor_array, beacons) = build_sensor_array(input);
    let mut total: u32 = 0;

    for x in -15_500_000..=15_500_000 {
        if is_covered(&sensor_array, Point { x, y: row_to_test }) {
            total += 1;
        }
    }
    // Subtract beacons that were on the line
    for (_, beacon) in beacons {
        if beacon.location.y == row_to_test {
            total -= 1;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    // const MAX_SEARCH_DISTANCE: usize = 4_000_000;
    const MAX_SEARCH_DISTANCE: usize = 20;
    let (sensor_array, beacons) = build_sensor_array(input);
    let mut missing_beacon: Option<Beacon> = None;
    for y in 0..=MAX_SEARCH_DISTANCE {
        for x in 0..=MAX_SEARCH_DISTANCE {
            if !is_covered(&sensor_array, Point { x: x as isize, y: y as isize }) {
                missing_beacon = Some(Beacon { location: Point { x: x as isize, y: y as isize } });
            }
        }
    }
    let mut tuning_frequency: u32 = 0;
    if let Some(beacon) = missing_beacon {
        tuning_frequency = (beacon.location.x * 4_000_000 + beacon.location.y) as u32;
    }
    Some(tuning_frequency)
}

fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_covered() {
        let sensor = Sensor::new(Point { x: 2, y: 18 }, Point { x: -2, y: 15 });
        assert_eq!(is_covered(&[sensor], Point { x: -2, y: 15 }), true);
        assert_eq!(is_covered(&[sensor], Point { x: -2, y: 16 }), true);
        assert_eq!(is_covered(&[sensor], Point { x: -1, y: 15 }), true);
        assert_eq!(is_covered(&[sensor], Point { x: -1, y: 16 }), true);
        assert_eq!(is_covered(&[sensor], Point { x: -3, y: 16 }), true);
        assert_eq!(is_covered(&[sensor], Point { x: -2, y: 14 }), false);
        assert_eq!(is_covered(&[sensor], Point { x: -3, y: 15 }), false);
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
