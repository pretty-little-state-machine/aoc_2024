use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

const GRID_HEIGHT: usize = 41;
const GRID_WIDTH: usize = 161;

#[derive(Copy, Clone, Eq, PartialEq)]
struct PathState {
    cost: usize,
    position: usize,
}

#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

#[derive(Debug)]
struct Terrain {
    grid: [[usize; GRID_WIDTH]; GRID_HEIGHT],
    source_id: usize,
    target_id: usize,
}

impl Terrain {
    fn new(input: &str) -> Self {
        let mut grid = [[99; GRID_WIDTH]; GRID_HEIGHT];
        let mut source_id: usize = 0;
        let mut target_id: usize = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, byte) in line.bytes().enumerate() {
                if 83 == byte {
                    source_id = y * GRID_WIDTH + x;
                } else if 69 == byte {
                    target_id = y * GRID_WIDTH + x;
                }
                grid[y][x] = ascii_byte_to_height(byte);
            }
        }
        Self {
            grid,
            source_id,
            target_id,
        }
    }

    fn build_graph(&self) -> Vec<Vec<Edge>> {
        let mut graph: Vec<Vec<Edge>> = Vec::new();

        for (y_idx, y) in self.grid.iter().enumerate() {
            for (x_idx, _x) in y.iter().enumerate() {
                let edges = self.get_neighbors(x_idx, y_idx);
                graph.push(edges);
            }
        }
        graph
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<Edge> {
        let mut neighbors: Vec<Edge> = Vec::new();
        // Right
        if x + 1 < GRID_WIDTH && (self.grid[y][x] + 1 >= self.grid[y][x + 1]) {
            neighbors.push(Edge {
                node: y * GRID_WIDTH + x + 1,
                cost: 1,
            });
        }
        // Left
        if x as isize > 0 && (self.grid[y][x] + 1 >= self.grid[y][x - 1]) {
            neighbors.push(Edge {
                node: y * GRID_WIDTH + x - 1,
                cost: 1,
            });
        }
        // Below
        if y + 1 < GRID_HEIGHT && (self.grid[y][x] + 1 >= self.grid[y + 1][x]) {
            neighbors.push(Edge {
                node: (y + 1) * GRID_WIDTH + x,
                cost: 1,
            });
        }
        // Above
        if y as isize > 0 && (self.grid[y][x] + 1 >= self.grid[y - 1][x]) {
            neighbors.push(Edge {
                node: (y - 1) * GRID_WIDTH + x,
                cost: 1,
            });
        }
        neighbors
    }

    fn get_lowest_points(&self) -> Vec<usize> {
        let mut lowest_points: Vec<usize> = Vec::new();
        for (y_idx, y) in self.grid.iter().enumerate() {
            for (x_idx, x) in y.iter().enumerate() {
                if *x == 1 {
                    lowest_points.push(y_idx * GRID_WIDTH + x_idx);
                }
            }
        }
        lowest_points
    }
}

/// Maps an item ascii character byte to the corresponding priority value.
#[inline(always)]
fn ascii_byte_to_height(input: u8) -> usize {
    (match input {
        83 => 1,                // `S` == lowest point
        69 => 26,               // 'E' == highest point
        97..=122 => input - 96, // Map `a..z` to `1..26`
        _ => unreachable!("Invalid ascii character range: {}", input),
    }) as usize
}

impl PartialOrd<Self> for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // println!("Finding path from {} -> {}", start, goal);
    // Set the distance to all remote nodes to max values
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(PathState {
        cost: 0,
        position: start,
    });

    while let Some(PathState { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }
        for edge in &adj_list[position] {
            let next = PathState {
                cost: cost + edge.cost,
                position: edge.node,
            };
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let terrain = Terrain::new(input);
    let graph = terrain.build_graph();
    shortest_path(&graph, terrain.source_id, terrain.target_id)
}

pub fn part_two(input: &str) -> Option<usize> {
    let terrain = Terrain::new(input);
    let graph = terrain.build_graph();
    let smallest = terrain
        .get_lowest_points()
        .iter()
        .map(|start| {
            if let Some(distance) = shortest_path(&graph, *start, terrain.target_id) {
                distance
            } else {
                usize::MAX
            }
        })
        .k_smallest(1);
    Some(smallest.sum::<usize>())
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
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
