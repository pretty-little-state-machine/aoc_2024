use regex::Regex;
use rustc_hash::FxHashMap;
use std::cmp::{max, min};

/// Defines a valve with a few fields:
///
/// id:
///   The string ID from the input file, ex: "AA", "BB", "II", etc....
///
/// flow_rate: usize
///   The flow rate of the valve, which may be 0
///
/// mask: usize
///   This value will be used later in the path finding to easily store visited state with boolean
///   math. Ex: AA = 001, BB = 010, CC = 100, DD = 1000, etc... thus the state value of 6 (1010)
///   would mean that valves `AA` and `DD` were visited.
///
/// neighbors: Vec<String>
///   The vec of neighbor strings from the input file, ex: vec!["AA", "BB", "II"]
#[derive(Debug, Eq, Hash, Clone, PartialEq)]
struct Valve {
    id: String,
    flow_rate: usize,
    mask: usize,
    neighbors: Vec<String>,
}

/// Parses the input string into a Valve struct containing a vector of neighboring nodes and flow
impl Valve {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r".*([A-Z]{2}).*=(\d+).*[se]\s([A-Z,\s]+)").unwrap();
        let cap = re.captures(input).unwrap();
        Self {
            id: cap[1].to_string(),
            flow_rate: cap[2].parse::<usize>().unwrap(),
            mask: 0,
            neighbors: cap[3]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        }
    }
}

/// Builds up a few datastructures that will be used in the problem.
///
/// **valve_distances** Vec<Vec<usize>>
///   A fully-meshed graph of valves where the value represents the time to move between valves. It
///   has to include all valves since even valves with 0 flow rate contribute to movement distance.
///
/// **valves** FxHashMap<String, Valve> - All the valves in a Hashmap using their String name
///
fn build_structures(
    input: &str,
) -> (
    FxHashMap<String, FxHashMap<String, usize>>,
    FxHashMap<String, Valve>,
) {
    // Decode the valve input text into a Vec<Valve> for easier datastructure construction later
    let mut valves: Vec<Valve> = input.lines().map(Valve::new).collect();

    // Bitstring Visited creation
    valves
        .iter_mut()
        .enumerate()
        .for_each(|(idx, valve)| valve.mask = 1 << idx);

    // Construct the Neighbors table using the IDs - Just used to build the Floyd Warshall table
    let mut neighbors: FxHashMap<String, Vec<String>> = FxHashMap::default();
    valves.iter().for_each(|v| {
        neighbors.insert(v.id.clone(), v.neighbors.clone());
    });

    // Floyd Warshall Setup - Nodes that connect to other nodes get a value of 1, else a huge value
    let mut valve_distances: FxHashMap<String, FxHashMap<String, usize>> = FxHashMap::default();
    for (row_valve, row_neighbors) in neighbors.iter() {
        valve_distances.insert(row_valve.clone(), FxHashMap::default());
        for (col_valve, _) in neighbors.iter() {
            if row_neighbors.contains(col_valve) {
                valve_distances
                    .get_mut(row_valve)
                    .unwrap()
                    .insert(col_valve.clone(), 1);
            } else {
                valve_distances
                    .get_mut(row_valve)
                    .unwrap()
                    .insert(col_valve.clone(), 99);
            }
        }
    }

    // Tabulate Floyd Warshall table iteratively
    for (k, _) in neighbors.iter() {
        for (i, _) in neighbors.iter() {
            for (j, _) in neighbors.iter() {
                let new_value = min(
                    *valve_distances.get(i).unwrap().get(j).unwrap(),
                    *valve_distances.get(i).unwrap().get(k).unwrap()
                        + *valve_distances.get(k).unwrap().get(j).unwrap(),
                );
                *valve_distances.get_mut(i).unwrap().get_mut(j).unwrap() = new_value;
            }
        }
    }
    (
        valve_distances,
        FxHashMap::from_iter(valves.iter().map(|v| (v.id.clone(), v.clone()))),
    )
}

/// This is a depth-first search across the Floyd Warshall graph. However only valves that have a
/// flow > 0 are considered as potential targets for the walk to reduce the search space. If any
/// state of walks results in a higher flow the state is updated with the better maximum value.
///
/// **Key Point**
/// The state is just a bit-mask of visited nodes. It does not contain the order that nodes were
/// visited in. For example, the nodes AA -> DD -> BB would share state `0b1011` but so would path
/// AA -> BB -> DD. This does not matter for this advent problem; and that is why the max value for
/// any state needs to be updated each call against the previous max that state may have had.
///
fn run<'a>(
    valves: &FxHashMap<String, Valve>, // All valves
    valve_distances: &FxHashMap<String, FxHashMap<String, usize>>, // All valves
    valve: String,                     // The current valve to search from
    time_left: usize,                  // Remaining time
    state: usize,                      // A usize of masking bits for visited valves
    flow: usize,                       // The current flow for the state
    answer: &'a mut FxHashMap<usize, usize>, // The best flow total for every state scanned
) -> &'a mut FxHashMap<usize, usize> {
    // If the flow just found is better, update our state with the new best flow
    if answer.get(&state).is_none() {
        answer.insert(state, 0);
    } else {
        let current_best = *answer.get(&state).unwrap();
        *answer.get_mut(&state).unwrap() = max(current_best, flow);
    }
    // Iterate over all valves that could potentially  add to the flow
    for (
        next_valve,
        Valve {
            flow_rate, mask, ..
        },
    ) in valves.iter()
    {
        // Useless valves and opened valves are skipped
        if mask & state != 0 || *flow_rate == 0 {
            continue;
        }
        // Move to the valve and open in, which takes an extra minute
        let new_time = time_left
            .saturating_sub(
                *valve_distances
                    .get(&valve)
                    .unwrap()
                    .get(next_valve)
                    .unwrap(),
            )
            .saturating_sub(1);
        if new_time == 0 {
            continue; // Base case - Out of time!
        }
        run(
            valves,
            valve_distances,
            next_valve.clone(),
            new_time,
            state | mask,
            flow + new_time * *flow_rate,
            answer,
        );
    }
    answer
}

/// Just myself with 30 minutes
pub fn part_one(input: &str) -> Option<usize> {
    let (valve_distances, valves) = build_structures(input);
    let mut answer = FxHashMap::default();
    run(
        &valves,
        &valve_distances,
        "AA".to_string(),
        30,
        0,
        0,
        &mut answer,
    );
    Some(*answer.values().max().unwrap())
}

/// The elephant and I with 26 minutes to spare
pub fn part_two(input: &str) -> Option<usize> {
    // This is all the same as Part 1, run a nice DFS over every possible valve state
    let (valve_distances, valves) = build_structures(input);
    let mut answer = FxHashMap::default();
    run(
        &valves,
        &valve_distances,
        "AA".to_string(),
        26, // Gotta train that elephant up!
        0,
        0,
        &mut answer,
    );
    // But we don't use the answer directly! Instead we take all the pairs of states that don't have
    // any overlap and pick two that have the best sum. This is the key insight to solving this
    // without having a "concurrent walking" strategy. Also scales to more than two players if we
    // want it to. Thankfully we can filter "overlapping" states due to our state being fit for
    // boolean math.
    let mut best: usize = 0;
    for (outer_state, outer_flow) in &answer {
        for (inner_state, inner_flow) in &answer {
            if outer_state & inner_state == 0 {
                best = max(best, *outer_flow + *inner_flow);
            }
        }
    }
    Some(best)
}

fn main() {
    let input = &aoc::read_file("inputs", 16);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
