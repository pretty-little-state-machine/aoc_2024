use petgraph::algo::{floyd_warshall, NegativeCycle};
use petgraph::prelude::NodeIndex;
use petgraph::{Directed, Graph};
use regex::Regex;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::thread::current;

type Pipeline = FxHashMap<usize, Valve>;

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
struct Valve {
    id: usize,
    node_index: Option<NodeIndex>,
    flow_rate: usize,
    neighbors: Vec<usize>,
    is_open: bool,
}

/// Converts the valve string into a unique usize reference so we avoid strings as keys
#[inline(always)]
fn valve_str_to_idx(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    (*chars.get(0).unwrap() as usize * 1000) + (*chars.get(1).unwrap() as usize)
}

impl Valve {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r".*([A-Z]{2}).*=(\d+).*[se]\s([A-Z,\s]+)").unwrap();
        let cap = re.captures(input).unwrap();
        Self {
            id: valve_str_to_idx(&cap[1]),
            node_index: None,
            flow_rate: cap[2].parse::<usize>().unwrap(),
            neighbors: cap[3]
                .split(", ")
                .map(valve_str_to_idx)
                .collect::<Vec<usize>>(),
            is_open: false,
        }
    }
}

fn gml(pipeline: &Pipeline) {
    println!("graph\n[");
    for (k, _) in pipeline.iter() {
        println!("node\n[\nid {}\n]\n", k)
    }
    for (k, v) in pipeline.iter() {
        for e in &v.neighbors {
            println!("edge\n[\nsource {}\ntarget {}\n]\n", k, e)
        }
    }
    println!("]")
}

fn build_graph(input: &str) -> (Graph<usize, usize, Directed>, Pipeline) {
    let mut graph: Graph<usize, usize, Directed> = Graph::default();
    let mut valves = Pipeline::default();

    for line in input.lines() {
        let mut valve = Valve::new(line);
        valve.node_index = Some(graph.add_node(valve.flow_rate));
        valves.insert(valve.id, valve);
    }

    for (_, valve) in &valves {
        for neighbor in &valve.neighbors {
            graph.add_edge(
                valve.node_index.unwrap(),
                valves.get(&neighbor).unwrap().node_index.unwrap(),
                1,
            );
        }
    }

    (graph, valves)
}

fn find_best_next(
    current_valve_id: usize,
    node_distances: &HashMap<(NodeIndex, NodeIndex), usize>,
    pipeline: &Pipeline,
) -> (Option<usize>, Option<usize>) {
    let current_valve = pipeline.get(&current_valve_id).unwrap();
    let mut best_flow_rate: usize = 0;
    let mut best_next_valve_id: Option<usize> = None;
    let mut best_node_distance: Option<usize> = None;
    'outer: for step in 0..25_usize {
        for (_, valve) in pipeline {
            // We don't care about the current valve we are at or any already open valves
            if current_valve.node_index.unwrap() == valve.node_index.unwrap() || valve.is_open {
                continue;
            }
            if let Some(distance) =
            node_distances.get(&(current_valve.node_index.unwrap(), valve.node_index.unwrap()))
            {
                let flow_rate = (step.saturating_sub(*distance) * valve.flow_rate);
                // println!("{} Estimated Flow Rate {}, {}", step, decode_valve_id(valve.id), flow_rate);
                if flow_rate >= best_flow_rate {
                    best_flow_rate = flow_rate;
                    best_next_valve_id = Some(valve.id);
                    best_node_distance = Some(*distance);
                }
                // There can't be a better solution regardless of how big it is
                if flow_rate >= 50 {
                    break 'outer;
                }
            }
        }
    }
    (best_next_valve_id, best_node_distance)
}

fn decode_valve_id(valve_id: usize) -> String {
    let a = valve_id / 1000;
    let b = valve_id % 100;
    format!("{}{}", char::from_u32(a as u32).unwrap(), char::from_u32(b as u32).unwrap()).to_string()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (graph, mut pipeline) = build_graph(input);
    let node_distances = floyd_warshall(&graph, |edge| 1).unwrap();
    let mut current_valve_id = 65065_usize; // AA

    let mut minutes: usize = 1;
    let mut pressure_per_tick: usize = 0;
    let mut total_pressure_released: usize = 0;
    while minutes <= 30 {
        if let (Some(next_valve_id), Some(distance)) =
        find_best_next(current_valve_id, &node_distances, &mut pipeline)
        {
            // Move to it while letting out pressure
            for _ in 0..distance {
                /*println!("\n=======MINUTE {}=======", minutes);
                println!(
                    "Moving from {} to {}...",
                    decode_valve_id(current_valve_id),
                    decode_valve_id(next_valve_id)
                );
                */
                total_pressure_released += pressure_per_tick;
                // println!("Pressure increased by {}", pressure_per_tick);
                minutes += 1;
                if minutes == 30 {
                    break;
                }
            }
            // Set it as the current valve and open it
            /*
            println!("\n=======MINUTE {}=======", minutes);
            println!("Opening Valve {}", decode_valve_id(next_valve_id));
             */
            let new_valve = pipeline.get_mut(&next_valve_id).unwrap();
            new_valve.is_open = true;
            // We must tick BEFORE we open as the valve won't increase flow until the next tick
            total_pressure_released += pressure_per_tick;
            // println!("Pressure increased by {}", pressure_per_tick);
            // Now the valve is open and the flow may increase for future ticks.
            pressure_per_tick += new_valve.flow_rate;
            current_valve_id = new_valve.id;
            minutes += 1;
        } else {
            // println!("\n=======MINUTE {}=======\n Waiting....", minutes);
            // We've done what we can with valves, now we have to wait for the pressure to release
            minutes += 1;
        }
    }
    Some(total_pressure_released as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}
