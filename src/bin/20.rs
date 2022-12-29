use itertools::Itertools;

const SIZE: usize = 5000;

type Mixer = [Item; SIZE];

#[derive(Default, Clone, Copy, Debug)]
struct Item {
    next: usize,
    prev: usize,
    value: isize,
}

fn find_after(n: usize, mixer: &Mixer) -> isize {
    let (mut cursor, _) = mixer.iter().find_position(|item| item.value == 0).unwrap();
    for _ in 0..n {
        cursor = mixer[cursor].next
    }
    mixer[cursor].value
}

fn build_mixer(input: &str, key: isize) -> Mixer {
    let original = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<isize>>();
    let length = original.len();
    let mut mixer = [Item::default(); SIZE];

    for (idx, value) in original.iter().enumerate() {
        mixer[idx].value = *value * key;
        mixer[idx].next = if idx + 1 == length { 0 } else { idx + 1 };
        mixer[idx].prev = if idx == 0 { length - 1 } else { idx - 1 };
    }
    mixer
}

fn run_mixer(mixer: &mut Mixer) {
    let length = mixer.len() as isize;
    for (index, _) in mixer.clone().iter().enumerate() {
        let shift_amount = mixer[index].value.abs() % (length - 1);
        // 0 values and moves to the same position in the list are no-ops
        if shift_amount == length || shift_amount == 0 {
            continue;
        }
        // Walk the linked list until the target
        let mut target = index;
        if mixer[index].value >= 0 {
            for _ in 0..shift_amount {
                target = mixer[target].next;
            }
        } else {
            for _ in 0..(shift_amount + 1) {
                target = mixer[target].prev;
            }
        }
        // Remove the item by stitching the adjacent nodes together
        mixer[mixer[index].prev].next = mixer[index].next;
        mixer[mixer[index].next].prev = mixer[index].prev;
        // Now insert the item between two more nodes
        let new_prev_item_idx = target;
        let new_next_item_idx = mixer[target].next;

        mixer[new_prev_item_idx].next = index;
        mixer[new_next_item_idx].prev = index;

        mixer[index].next = new_next_item_idx;
        mixer[index].prev = new_prev_item_idx;
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut mixer = build_mixer(input, 1);
    run_mixer(&mut mixer);
    Some(find_after(1000, &mixer) + find_after(2000, &mixer) + find_after(3000, &mixer))
}

pub fn part_two(input: &str) -> Option<isize> {
    const DECRYPTION_KEY: isize = 811589153;
    let mut mixer = build_mixer(input, DECRYPTION_KEY);
    for _ in 0..10 {
        run_mixer(&mut mixer);
    }
    Some(find_after(1000, &mixer) + find_after(2000, &mixer) + find_after(3000, &mixer))
}

fn main() {
    let input = &aoc::read_file("inputs", 20);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
