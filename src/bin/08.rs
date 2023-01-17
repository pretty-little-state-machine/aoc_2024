use rustc_hash::FxHashMap;

/// The tree is stored as an arena of Packets with the key as the offset within the payload.
type Packets = FxHashMap<usize, Packet>;

#[derive(Debug)]
struct Packet {
    child_node_offsets: Vec<usize>,
    metadata: Vec<usize>,
}

impl Packet {
    /// Returns the node's value for Part II
    fn calc_value(&self, packets: &Packets) -> usize {
        if self.child_node_offsets.is_empty() {
            self.metadata.iter().sum::<usize>()
        } else {
            let mut sum: usize = 0;
            for metadata_entry in &self.metadata {
                if metadata_entry - 1 < self.child_node_offsets.len() {
                    sum += packets
                        .get(&self.child_node_offsets[*metadata_entry - 1])
                        .unwrap()
                        .calc_value(packets);
                }
            }
            sum
        }
    }
}

fn build_tree(payload: &Vec<usize>, packets: &mut Packets, offset: usize) -> usize {
    let num_children = payload[offset];
    let num_metadata = payload[offset + 1];
    let mut packet = Packet {
        child_node_offsets: vec![],
        metadata: vec![],
    };
    let mut cursor: usize = offset + 2;
    let mut child_lengths: Vec<usize> = Vec::new();
    if num_children > 0 {
        while packet.child_node_offsets.len() < num_children {
            packet.child_node_offsets.push(cursor);
            let child_node_length = build_tree(payload, packets, cursor);
            child_lengths.push(child_node_length);
            cursor += child_node_length;
        }
    }
    for x in 0..num_metadata {
        packet.metadata.push(payload[cursor + x]);
    }
    let length = child_lengths.iter().sum::<usize>() + num_metadata + 2;
    packets.insert(offset, packet);
    length
}

pub fn part_one(input: &str) -> Option<usize> {
    let payload: Vec<usize> = input
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    let mut packets = FxHashMap::default();
    build_tree(&payload, &mut packets, 0);
    Some(
        packets
            .values()
            .map(|packet| packet.metadata.iter().sum::<usize>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let payload: Vec<usize> = input
        .split(' ')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();
    let mut packets = FxHashMap::default();
    build_tree(&payload, &mut packets, 0);
    Some(packets.get(&0).unwrap().calc_value(&packets))
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(138));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(66));
    }
}
