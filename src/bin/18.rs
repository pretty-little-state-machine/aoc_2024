use rustc_hash::FxHashMap;
use std::collections::VecDeque;

type CubePosition = (isize, isize, isize);
type Droplet<'a> = FxHashMap<CubePosition, &'a Cube>;
// The Air molecules have a position key and a value of the number of obsidian-facing faces
type Air<'a> = FxHashMap<CubePosition, Option<usize>>;

#[derive(Debug)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

fn get_neighboring_faces(position: CubePosition) -> [CubePosition; 6] {
    [
        (position.0 + 1, position.1, position.2),
        (position.0 - 1, position.1, position.2),
        (position.0, position.1 + 1, position.2),
        (position.0, position.1 - 1, position.2),
        (position.0, position.1, position.2 + 1),
        (position.0, position.1, position.2 - 1),
    ]
}

fn get_air_successors(position: CubePosition, droplet: &Droplet) -> Vec<CubePosition> {
    const MIN_BOUND: isize = -2;
    const MAX_BOUND: isize = 23;
    let mut successors: Vec<CubePosition> = Vec::default();
    get_neighboring_faces(position).iter().for_each(|pos| {
        if pos.0 < MAX_BOUND
            && pos.0 > MIN_BOUND
            && pos.1 < MAX_BOUND
            && pos.1 > MIN_BOUND
            && pos.2 < MAX_BOUND
            && pos.2 > MIN_BOUND
            && !droplet.contains_key(pos)
        {
            successors.push(*pos);
        }
    });
    successors
}

fn num_air_to_droplet_faces(position: CubePosition, droplet: &Droplet) -> usize {
    get_neighboring_faces(position)
        .iter()
        .map(|p| droplet.contains_key(p) as usize)
        .sum::<usize>()
}

impl Cube {
    fn from_str(input: &str) -> Self {
        let sides: Vec<isize> = input.split(',').map(|s| s.parse().unwrap()).collect();
        Self {
            x: *sides.first().unwrap(),
            y: *sides.get(1).unwrap(),
            z: *sides.get(2).unwrap(),
        }
    }

    fn unblocked_side_count(&self, droplet: &Droplet) -> usize {
        6 - [
            droplet.contains_key(&(self.x + 1, self.y, self.z)) as usize,
            droplet.contains_key(&(self.x - 1, self.y, self.z)) as usize,
            droplet.contains_key(&(self.x, self.y + 1, self.z)) as usize,
            droplet.contains_key(&(self.x, self.y - 1, self.z)) as usize,
            droplet.contains_key(&(self.x, self.y, self.z + 1)) as usize,
            droplet.contains_key(&(self.x, self.y, self.z - 1)) as usize,
        ]
        .iter()
        .sum::<usize>()
    }

    fn as_key(&self) -> CubePosition {
        (self.x, self.y, self.z)
    }
}

/// Calculate the sum of exposed cubes sides in the droplet.
pub fn part_one(input: &str) -> Option<usize> {
    let mut droplet = Droplet::default();
    let cubes: Vec<Cube> = input.lines().map(Cube::from_str).collect();
    for cube in &cubes {
        droplet.insert(cube.as_key(), cube);
    }
    Some(cubes.iter().map(|c| c.unblocked_side_count(&droplet)).sum())
}

fn bfs(root: CubePosition, air: &mut Air, droplet: &Droplet) {
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(position) = queue.pop_front() {
        let v = air.get_mut(&position).unwrap();
        // Unvisited nodes will have a None value
        if v.is_none() {
            *v = Some(num_air_to_droplet_faces(position, droplet));
            let successors = get_air_successors(position, droplet);
            for s in successors {
                // DO NOT OVERWRITE ANY SUCCESSORS WE HAVE ALREADY SEEN
                air.entry(s).or_insert(None);
                queue.push_back(s);
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut droplet = Droplet::default();
    let cubes: Vec<Cube> = input.lines().map(Cube::from_str).collect();
    for cube in &cubes {
        droplet.insert(cube.as_key(), cube);
    }

    let mut air = Air::default();
    air.insert((0, 0, 0), None);
    bfs((0, 0, 0), &mut air, &droplet);
    Some(
        air.values()
            .map(|value| if let Some(v) = value { v } else { &0 })
            .sum(),
    )
}

fn main() {
    let input = &aoc::read_file("inputs", 18);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
