use itertools::any;
use regex::Regex;
use std::collections::HashMap;

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
    range: usize,
}

impl Sensor {
    pub fn new(location: Point, beacon: Point) -> Self {
        Self {
            location,
            range: manhattan(location, beacon),
        }
    }

    /// A perimeter is drawn around the scanning range as a diagonal line starting one block higher
    /// than the range. (0,0) is considered the top-left corner of the field for relative movement.
    ///
    /// Only values between 0 and search_distance inclusively are added to the Vec.
    pub fn get_perimeter_squares(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::with_capacity(100);
        let mut current_point = Point {
            x: self.location.x,
            y: self.location.y - self.range as isize - 1,
        };
        points.push(current_point);
        // Top to Right
        for _ in 0..=self.range {
            current_point.x += 1;
            current_point.y += 1;
            points.push(current_point);
        }
        // Right to Bottom
        for _ in 0..=self.range {
            current_point.x -= 1;
            current_point.y += 1;
            points.push(current_point);
        }
        // Bottom to Left
        for _ in 0..=self.range {
            current_point.x -= 1;
            current_point.y -= 1;
            points.push(current_point);
        }
        // Left to Top (Omitting the last block since it was done first)
        for _ in 0..self.range {
            current_point.x += 1;
            current_point.y -= 1;
            points.push(current_point);
        }
        points
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

pub fn part_one(input: &str) -> Option<usize> {
    let row_to_test = 2000000;
    //let row_to_test = 10;
    let (mut sensor_array, beacons) = build_sensor_array(input);
    // Filter sensors that aren't covering the current row at all
    sensor_array.retain(|s| {
        (s.location.y - s.range as isize <= row_to_test)
            && (s.location.y + s.range as isize >= row_to_test)
    });
    // Find all X Squares for the given row per sensor. We can use the delta_y to determine the
    // range of X since the covered space always shrinks linearly with delta_y.
    let mut range: (isize, isize) = (0, 0);
    for sensor in sensor_array {
        let delta_y = -((row_to_test - sensor.location.y).abs());
        let min = -(sensor.range as isize + delta_y) + sensor.location.x;
        let max = (sensor.range as isize + delta_y) + sensor.location.x;
        if min < range.0 {
            range.0 = min
        }
        if max > range.1 {
            range.1 = max
        }
    }
    // We must add 1 to make this math _inclusive_
    let mut total = range.1 - range.0 + 1;
    // Subtract any beacons that were on the line
    for (_, beacon) in beacons {
        if beacon.location.y == row_to_test {
            total -= 1;
        }
    }
    Some(total as usize)
}

pub fn part_two(input: &str) -> Option<usize> {
    const MAX_SEARCH_DISTANCE: isize = 4_000_000;
    //const MAX_SEARCH_DISTANCE: isize = 20;
    let (sensor_array, _) = build_sensor_array(input);
    let mut missing_beacon: Option<Beacon> = None;

    let mut missing_beacons: Vec<Point> = Vec::new();
    'sensor: for sensor in &sensor_array {
        for square in sensor.get_perimeter_squares() {
            if square.x >= 0
                && square.y >= 0
                && square.x <= MAX_SEARCH_DISTANCE
                && square.y <= MAX_SEARCH_DISTANCE
                && !is_covered(&sensor_array, square)
            {
                missing_beacons.push(square);
                missing_beacon = Some(Beacon { location: square });
                break 'sensor;
            }
        }
    }
    let mut tuning_frequency: usize = 0;
    if let Some(beacon) = missing_beacon {
        tuning_frequency = beacon.location.x as usize * 4_000_000 + beacon.location.y as usize;
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
    fn test_get_perimeter() {
        let sensor = Sensor::new(Point { x: 2, y: 18 }, Point { x: -2, y: 15 });
        assert_eq!(sensor.range, 7);
        assert_eq!(32, sensor.get_perimeter_squares().len());
    }

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
