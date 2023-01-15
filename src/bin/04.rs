use rustc_hash::FxHashMap;

type Guard = usize;
type SleepHistogram = FxHashMap<Guard, FxHashMap<usize, usize>>;
type SleepTotals = FxHashMap<Guard, usize>;

// I really don't want to pull in Chrono here
#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct Timestamp {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl Timestamp {
    fn new(input: &str) -> Self {
        Self {
            year: input[1..5].parse::<usize>().unwrap(),
            month: input[6..8].parse::<usize>().unwrap(),
            day: input[9..11].parse::<usize>().unwrap(),
            hour: input[12..14].parse::<usize>().unwrap(),
            minute: input[15..17].parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Action {
    FallsAsleep,
    WakesUp,
    StartsShift { guard: usize },
}

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq)]
struct LogEntry {
    timestamp: Timestamp,
    action: Action,
}

impl LogEntry {
    fn new(input: &str) -> Self {
        let timestamp = Timestamp::new(input);
        let action = match &input[19..] {
            a if a.contains("falls asleep") => Action::FallsAsleep,
            a if a.contains("wakes up") => Action::WakesUp,
            a if a.contains("Guard") => Action::StartsShift {
                guard: a
                    .split(['#', ' '])
                    .collect::<Vec<_>>()
                    .get(2)
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            },
            _ => unreachable!("Unknown Guard Action"),
        };
        Self { timestamp, action }
    }
}

#[inline(always)]
fn calculate_guard_stats(log: &Vec<LogEntry>) -> (SleepTotals, SleepHistogram) {
    let mut guard_minutes = FxHashMap::default();
    let mut guard_sleep_histogram = FxHashMap::default();

    let mut current_guard: usize = 0;
    let mut last_asleep: usize = 0;

    for entry in log {
        match entry.action {
            Action::StartsShift { guard } => current_guard = guard,
            Action::FallsAsleep => last_asleep = entry.timestamp.minute,
            Action::WakesUp => {
                // Hashmap for total minutes slept
                let minutes_slept = entry.timestamp.minute - last_asleep;
                if let Some(guard) = guard_minutes.get_mut(&current_guard) {
                    *guard += minutes_slept;
                } else {
                    guard_minutes.insert(current_guard, minutes_slept);
                }

                // Track which minutes were slept on
                guard_sleep_histogram
                    .entry(current_guard)
                    .or_insert_with(FxHashMap::default);
                if let Some(guard) = guard_sleep_histogram.get_mut(&current_guard) {
                    for minute in last_asleep..entry.timestamp.minute {
                        if let Some(minute_count) = guard.get_mut(&minute) {
                            *minute_count += 1;
                        } else {
                            guard.insert(minute, 1);
                        }
                    }
                }
            }
        }
    }
    (guard_minutes, guard_sleep_histogram)
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut log: Vec<LogEntry> = input.lines().map(LogEntry::new).collect();
    log.sort();
    let (guard_minutes, guard_sleep_histogram) = calculate_guard_stats(&log);

    let (laziest_guard, _) = guard_minutes.iter().max_by_key(|v| v.1).unwrap();
    let (favorite_minute, _) = guard_sleep_histogram
        .get(laziest_guard)
        .unwrap()
        .iter()
        .max_by_key(|v| v.1)
        .unwrap();

    Some(laziest_guard * favorite_minute)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut log: Vec<LogEntry> = input.lines().map(LogEntry::new).collect();
    log.sort();
    let (_, guard_sleep_histogram) = calculate_guard_stats(&log);

    let mut most_seen_minute: usize = 0;
    let mut most_seen_count: usize = 0;
    let mut target_guard: usize = 0;
    for (guard, minutes) in guard_sleep_histogram {
        let (favorite_minute, count) = minutes.iter().max_by_key(|v| v.1).unwrap();
        if *count > most_seen_count {
            most_seen_count = *count;
            most_seen_minute = *favorite_minute;
            target_guard = guard;
        }
    }

    Some(target_guard * most_seen_minute)
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
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(240));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4455));
    }
}
