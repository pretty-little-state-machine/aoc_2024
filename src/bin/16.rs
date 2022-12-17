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
    'outer: for step in 0..10_usize {
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
                if flow_rate > best_flow_rate {
                    best_flow_rate = flow_rate;
                    best_next_valve_id = Some(valve.id);
                    best_node_distance = Some(*distance);
                }
                // There can't be a better solution regardless of how big it is
                if flow_rate >= 25 {
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
    format!(
        "{}{}",
        char::from_u32(a as u32).unwrap(),
        char::from_u32(b as u32).unwrap()
    )
        .to_string()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (graph, mut pipeline) = build_graph(input);
    let node_distances = floyd_warshall(&graph, |edge| 1).unwrap();
    let mut current_valve_id = 65065_usize; // AA

    let mut pressure_per_tick: usize = 0;
    let mut total_pressure_released: usize = 0;

    let mut path: Vec<(usize, usize)> = Vec::new();
    // We won't be visiting more than maybe 10 valves
    for _ in 0..20 {
        if let (Some(next_valve_id), Some(distance)) =
        find_best_next(current_valve_id, &node_distances, &mut pipeline)
        {
            let new_valve = pipeline.get_mut(&next_valve_id).unwrap();
            current_valve_id = new_valve.id;
            new_valve.is_open = true;
            path.push((new_valve.id, distance));
        }
    }

    let mut movement_penalty: usize = path.first().unwrap().1;
    let mut new_flow_rate = 0;
    let mut path_ptr: usize = 0;

    for _ in 0..30 {
        total_pressure_released += pressure_per_tick;
        if movement_penalty > 0 {
            movement_penalty = movement_penalty.saturating_sub(1);
        } else {
            if let Some((valve_index, _)) = path.get(path_ptr) {
                path_ptr += 1;
                let valve = pipeline.get(&valve_index).unwrap();
                new_flow_rate = valve.flow_rate;
                if let Some((_, penalty)) = path.get(path_ptr) {
                    movement_penalty = *penalty;
                }
                pressure_per_tick += new_flow_rate;
            }
        }
    }

    /*
       while minutes <= 30 {
           if let (Some(next_valve_id), Some(distance)) =
           find_best_next(current_valve_id, &node_distances, &mut pipeline)
           {
               // Move to it while letting out pressure
               for _ in 0..distance {
                   total_pressure_released += pressure_per_tick;
                   minutes += 1;
                   if minutes == 30 {
                       break;
                   }
               }
               // Set it as the current valve and open it
               let new_valve = pipeline.get_mut(&next_valve_id).unwrap();
               new_valve.is_open = true;
               // We must tick BEFORE we open as the valve won't increase flow until the next tick
               total_pressure_released += pressure_per_tick;
               // Now the valve is open and the flow may increase for future ticks.
               pressure_per_tick += new_valve.flow_rate;
               current_valve_id = new_valve.id;
               minutes += 1;
           } else {
               minutes += 1;
           }
       }

     */
    Some(total_pressure_released as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (graph, mut pipeline) = build_graph(input);
    let node_distances = floyd_warshall(&graph, |edge| 1).unwrap();
    let mut pal_current_valve_id = 65065_usize; // AA for my elephant pal
    let mut my_current_valve_id = 65065_usize; // AA for me

    let mut pal_move_penalty = 0_usize;
    let mut my_move_penalty = 0_usize;

    let mut pal_new_flow_rate = 0_usize;
    let mut my_new_flow_rate = 0_usize;

    let mut minutes: usize = 1;
    let mut pressure_per_tick: usize = 0;
    let mut total_pressure_released: usize = 0;

    // TODO: Find the paths first! Then execute the simulation.

    while minutes <= 26 {
        total_pressure_released += pressure_per_tick;
        println!(
            "======MINUTE {}=======\n{} pressure releases",
            minutes, pressure_per_tick
        );

        // Move the elephant
        if pal_move_penalty > 0 {
            pal_move_penalty = pal_move_penalty.saturating_sub(1);
            println!(
                "Elephant is moving to {}",
                decode_valve_id(pal_current_valve_id)
            );
        } else {
            // Since we arrived we open the valve, then select a new target valve
            println!("Elephant opened {}", decode_valve_id(pal_current_valve_id));
            pressure_per_tick += pal_new_flow_rate;
            if let (Some(pal_next_best_id), Some(pal_distance)) =
            find_best_next(pal_current_valve_id, &node_distances, &mut pipeline)
            {
                println!(
                    "Elephant selected {} next",
                    decode_valve_id(pal_next_best_id)
                );
                pal_move_penalty = pal_distance - 1;
                // Set it as the current valve and open it
                let pal_new_valve = pipeline.get_mut(&pal_next_best_id).unwrap();
                // "Open the valve" to mark it visited, but don't add pressure yet
                pal_new_flow_rate = pal_new_valve.flow_rate;
                pal_new_valve.is_open = true;
                pal_current_valve_id = pal_new_valve.id;
            }
        }

        // Move me!
        if my_move_penalty > 0 {
            my_move_penalty = my_move_penalty.saturating_sub(1);
            println!("I am moving to {}", decode_valve_id(my_current_valve_id));
        } else {
            // Since we arrived we open the valve, then select a new target valve
            println!("I opened {}", decode_valve_id(my_current_valve_id));
            pressure_per_tick += my_new_flow_rate;
            if let (Some(my_next_best_id), Some(distance)) =
            find_best_next(my_current_valve_id, &node_distances, &mut pipeline)
            {
                println!("I selected {} next", decode_valve_id(my_next_best_id));
                my_move_penalty = distance - 1;
                // Set it as the current valve and open it
                let my_new_valve = pipeline.get_mut(&my_next_best_id).unwrap();
                // "Open the valve" to mark it visited, but don't add pressure yet
                my_new_flow_rate = my_new_valve.flow_rate;
                my_new_valve.is_open = true;
                my_current_valve_id = my_new_valve.id;
            }
        }
        println!();
        minutes += 1;
    }
    Some(total_pressure_released as u32)
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
