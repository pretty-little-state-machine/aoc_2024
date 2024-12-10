extern crate core;

use std::collections::VecDeque;
use std::ops::Div;
use crate::DayResult;
use fxhash::{FxHashMap, FxHashSet};
use rayon::prelude::*;
use std::time::Instant;
use image::{Rgb, RgbImage};
use itertools::Itertools;

pub fn run(input: &str) -> DayResult {
    let start = Instant::now();
    let data = parse(input);
    let parse_duration = start.elapsed();

    let start = Instant::now();
    let p1 = part_1(&data).to_string();
    let p1_duration = start.elapsed();

    let start = Instant::now();
    let p2 = part_2(&data).to_string();
    let p2_duration = start.elapsed();
    (Some(parse_duration), (p1, p1_duration), (p2, p2_duration))
}

#[derive(Debug, Default)]
struct TopoMap {
    topo: FxHashMap<(isize, isize), u8>,
    trailheads: Vec<(isize, isize)>,
}

fn parse(input: &str) -> TopoMap {
    let mut topo_map = TopoMap::default();
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if (b - 48) == 0 {
                topo_map.trailheads.push((x as isize, y as isize));
            }
            topo_map.topo.insert((x as isize, y as isize), b - 48);
        }
    }
    topo_map
}

fn draw_viz(t: &TopoMap, visited_nodes: &FxHashSet<(isize, isize)>, iteration: usize) {
    const COLORS: [Rgb<u8>; 10] = [
        Rgb([162, 74, 2]),
        Rgb([176, 92, 0]),
        Rgb([189, 111, 0]),
        Rgb([202, 130, 0]),
        Rgb([213, 150, 1]),
        Rgb([202, 154,16]),
        Rgb([216, 178, 24]),
        Rgb([230,203,35]),
        Rgb([241,229,47]),
        Rgb([251,255,61]),
    ];
    const PATH_COLORS: [Rgb<u8>; 10] = [
        Rgb([0, 57, 0]),
        Rgb([8, 76, 15]),
        Rgb([20, 96, 30]),
        Rgb([30, 117, 45]),
        Rgb([40,138, 61]),
        Rgb([49,161,78]),
        Rgb([58,183,95]),
        Rgb([65,207,114]),
        Rgb([73,231,133]),
        Rgb([79,255,153]),

    ];
    let dim = t.topo.keys().map(|(x, _)| *x).max().unwrap() as u32;
    let scale = 8;
    let mut image = RgbImage::new(dim * scale, dim * scale);
    for y in 0..dim {
        for x in 0..dim {
            if let Some(v) = t.topo.get(&(x as isize, y as isize)) {
                let color = if visited_nodes.contains(&(x as isize, y as isize)) {
                    PATH_COLORS[*v as usize]
                } else {
                    COLORS[*v as usize]
                };
                for sy in scale*y..scale*y + scale{
                    for sx in scale*x .. scale *x + scale {
                        //println!("{}, {}", (x + sx) % dim, (y + sy) % dim);
                        image.put_pixel(sx, sy, color);
                    }
                }
            }
        }
    }
    image.save(format!("src/viz/{:06}.png", iteration)).unwrap();
}


fn bfs(t: &TopoMap, count_unique_trails: bool) -> usize {
    //let mut iteration = 0;
    t.trailheads
        .par_iter()
        .map(|trailhead| {
            let mut visited = FxHashSet::default();
            let mut queue = VecDeque::new();
            queue.push_back(*trailhead);
            let mut score = 0;
            // draw_viz(t, &visited, 0);
            while let Some(position) = queue.pop_front() {
                // draw_viz(t, &visited, iteration);
                //iteration += 1;
                //println!("{:?}::: {:?} :: {:?} :: {:?}", score, t.topo.get(&position), position, visited);
                if ! count_unique_trails {
                    if visited.contains(&position) {
                        continue;
                    }
                    visited.insert(position);
                }
                if let Some(x) = t.topo.get(&position) {
                    if *x == 9 {
                        score += 1;
                        continue;
                    }
                    visited.insert(position);
                    [
                        (position.0 - 1, position.1),
                        (position.0 + 1, position.1),
                        (position.0, position.1 - 1),
                        (position.0, position.1 + 1),
                    ]
                    .iter()
                    .for_each(|p| {
                        if let Some(neighbor) = t.topo.get(p) {
                            if x + 1 == *neighbor {
                                queue.push_back(*p);
                            }
                        }
                    });
                }
            }
            score
        })
        .sum()
}

fn part_1(t: &TopoMap) -> usize {
    bfs(t, false)
}

fn part_2(t: &TopoMap) -> usize {
    bfs(t, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part_one() {
        let input = fs::read_to_string("./src/example/day_10.txt").expect("File not found.");
        let x = parse(&input);
        assert_eq!(part_1(&x), 36);
    }

    #[test]
    fn test_part_two() {
        let input = fs::read_to_string("./src/example/day_10.txt").expect("File not found.");
        let x = parse(&input);
        assert_eq!(part_2(&x), 81);
    }
}
