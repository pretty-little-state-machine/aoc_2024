use pathfinding::prelude::dfs;
use petgraph::algo::{astar, floyd_warshall, NegativeCycle};
use petgraph::prelude::{Bfs, DfsPostOrder, NodeIndex};
use petgraph::visit::{depth_first_search, Dfs, IntoNodeReferences};
use petgraph::{Directed, Graph};
use regex::Regex;
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use itertools::Itertools;

type Pipeline = FxHashMap<usize, Valve>;

#[derive(Debug, Eq, Hash, Clone, PartialEq)]
struct Valve {
    id: usize,
    fw_index: Option<NodeIndex>,
    fm_index: Option<NodeIndex>,
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
            fw_index: None,
            fm_index: None,
            flow_rate: cap[2].parse::<usize>().unwrap(),
            neighbors: cap[3]
                .split(", ")
                .map(valve_str_to_idx)
                .collect::<Vec<usize>>(),
            is_open: false,
        }
    }
}

fn build_graph(input: &str) -> (Graph<usize, usize, Directed>, Pipeline) {
    let mut graph: Graph<usize, usize, Directed> = Graph::default();
    let mut valves = Pipeline::default();

    for line in input.lines() {
        let mut valve = Valve::new(line);
        valve.fw_index = Some(graph.add_node(valve.flow_rate));
        valves.insert(valve.id, valve);
    }

    for (_, valve) in &valves {
        for neighbor in &valve.neighbors {
            graph.add_edge(
                valve.fw_index.unwrap(),
                valves.get(&neighbor).unwrap().fw_index.unwrap(),
                1,
            );
        }
    }

    (graph, valves)
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


fn walk_steam() {
    if t > 30 {}

}




pub fn part_one(input: &str) -> Option<u32> {
    const AA: usize = 65065;
    let (fw_graph, mut pipeline) = build_graph(input);
    let node_distances = floyd_warshall(&fw_graph, |edge| 1).unwrap();
    // Prune valves that don't release any steam except for AA which we must keep.
    pipeline.retain(|_, v| v.flow_rate > 0 || v.id == AA);
    // Build a graph of all valves to all valves but use the floyd-warshall weights for edges.
    let mut full_mesh = Graph::<&Valve, usize>::new();
    let mut mesh_map = FxHashMap::default();
    pipeline.iter().for_each(|(_, v)| {
        mesh_map.insert(v.id, full_mesh.add_node(&v));
    });
    // Add edges to the full mesh
    for combo in mesh_map.iter().permutations(2) {
        let a = combo[0];
        let b = combo[1];
        let fw_a = pipeline.get(&a.0).unwrap().fw_index.unwrap();
        let fw_b = pipeline.get(&b.0).unwrap().fw_index.unwrap();
        let weight = node_distances.get(&(fw_a, fw_b)).unwrap();
        full_mesh.add_edge(*a.1, *b.1, *weight + 1);
    }

    let node_aa = mesh_map.get(&AA).unwrap();
    let mut dfs = DfsPostOrder::new(&full_mesh, *node_aa);
    while let Some(node) = dfs.next(&full_mesh) {
        println!("node: {:?}", full_mesh.node_references()));
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    return None;
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
