use crate::num_traits::FromPrimitive;
use regex::Regex;
use rustc_hash::FxHashSet;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

#[derive(Default, Debug)]
struct Device {
    reg: [usize; 4],
}

/// Uncomment the `determine_opcodes` function part 2 to re-derive these values.
#[derive(Debug, Hash, Eq, Copy, Clone, Primitive, PartialEq)]
enum Opcode {
    Addr = 1,
    Addi = 13,
    Mulr = 15,
    Muli = 14,
    Banr = 0,
    Bani = 9,
    Borr = 8,
    Bori = 5,
    Setr = 3,
    Seti = 7,
    Gtir = 6,
    Gtri = 12,
    Gtrr = 4,
    Eqir = 10,
    Eqri = 2,
    Eqrr = 11,
}

impl Device {
    fn execute(&mut self, [op, a, b, c]: [usize; 4]) {
        match Opcode::from_usize(op).expect("Unknown Opcode") {
            Opcode::Addr => self.reg[c] = self.reg[a] + self.reg[b],
            Opcode::Addi => self.reg[c] = self.reg[a] + b,
            Opcode::Mulr => self.reg[c] = self.reg[a] * self.reg[b],
            Opcode::Muli => self.reg[c] = self.reg[a] * b,
            Opcode::Banr => self.reg[c] = self.reg[a] & self.reg[b],
            Opcode::Bani => self.reg[c] = self.reg[a] & b,
            Opcode::Borr => self.reg[c] = self.reg[a] | self.reg[b],
            Opcode::Bori => self.reg[c] = self.reg[a] | b,
            Opcode::Setr => self.reg[c] = self.reg[a],
            Opcode::Seti => self.reg[c] = a,
            Opcode::Gtir => self.reg[c] = if a > self.reg[b] { 1 } else { 0 },
            Opcode::Gtri => self.reg[c] = if self.reg[a] > b { 1 } else { 0 },
            Opcode::Gtrr => self.reg[c] = if self.reg[a] > self.reg[b] { 1 } else { 0 },
            Opcode::Eqir => self.reg[c] = if a == self.reg[b] { 1 } else { 0 },
            Opcode::Eqri => self.reg[c] = if self.reg[a] == b { 1 } else { 0 },
            Opcode::Eqrr => self.reg[c] = if self.reg[a] == self.reg[b] { 1 } else { 0 },
        }
    }
}

#[derive(Default, Debug)]
struct Sample {
    before: [usize; 4],
    cmd: [usize; 4],
    after: [usize; 4],
}

fn parse_before_after(input: &str) -> [usize; 4] {
    let re = Regex::new(r".*\[([\d]+), ([\d]+), ([\d]+), ([\d]+)]").unwrap();
    let caps = re.captures(input).unwrap();
    [
        caps[1].parse::<usize>().unwrap(),
        caps[2].parse::<usize>().unwrap(),
        caps[3].parse::<usize>().unwrap(),
        caps[4].parse::<usize>().unwrap(),
    ]
}

fn parse_cmd(input: &str) -> [usize; 4] {
    let re = Regex::new(r"([\d]+) ([\d]+) ([\d]+) ([\d]+)").unwrap();
    let caps = re.captures(input).unwrap();
    [
        caps[1].parse::<usize>().unwrap(),
        caps[2].parse::<usize>().unwrap(),
        caps[3].parse::<usize>().unwrap(),
        caps[4].parse::<usize>().unwrap(),
    ]
}

fn parse_samples(input: &str) -> Vec<Sample> {
    let parts: Vec<&str> = input.split("\n\n\n").collect();
    let lines: Vec<&str> = parts[0].split('\n').collect();
    let mut samples = Vec::new();
    for line in lines.chunks(4) {
        samples.push(Sample {
            before: parse_before_after(line[0]),
            cmd: parse_cmd(line[1]),
            after: parse_before_after(line[2]),
        });
    }
    samples
}

/// Swaps out the operand and checks every opcode for a match.
fn count_opcode_matches(sample: &Sample) -> usize {
    let mut device = Device::default();
    let mut matches = 0;
    for x in 0..=15 {
        device.reg = sample.before;
        let op = [x, sample.cmd[1], sample.cmd[2], sample.cmd[3]];
        device.execute(op);
        if device.reg == sample.after {
            matches += 1;
        }
    }
    matches
}

pub fn part_one(input: &str) -> Option<usize> {
    let samples = parse_samples(input);
    let matches = samples
        .iter()
        .map(count_opcode_matches)
        .collect::<Vec<usize>>();
    Some(matches.iter().filter(|&m| *m >= 3).count())
}

#[allow(dead_code)]
/// Executes the samples over and over gradually to discover which opcodes are which integers
fn determine_opcodes(samples: &Vec<Sample>) {
    let mut device = Device::default();
    let mut remaining_opcodes: Vec<Opcode> = Vec::with_capacity(16);
    for x in 0_usize..16 {
        remaining_opcodes.push(Opcode::from_usize(x).unwrap());
    }
    while !remaining_opcodes.is_empty() {
        for sample in samples {
            let mut candidates = FxHashSet::default();
            for opcode in remaining_opcodes.clone() {
                let x = opcode as usize;
                device.reg = sample.before;
                device.execute([x, sample.cmd[1], sample.cmd[2], sample.cmd[3]]);
                if device.reg == sample.after {
                    candidates.insert((sample.cmd[0], opcode));
                }
            }
            if candidates.len() == 1 {
                let (x, opcode) = *candidates.iter().collect::<Vec<_>>().first().unwrap();
                println!("Opcode {opcode:?} == {x}");
                remaining_opcodes.retain(|&o| o != *opcode);
            }
        }
    }
}

fn parse_program(input: &str) -> Vec<[usize; 4]> {
    let mut program = Vec::with_capacity(1_000);
    let parts: Vec<&str> = input.split("\n\n\n").collect();
    for line in parts[1].split('\n').collect::<Vec<&str>>() {
        if !line.is_empty() {
            program.push(parse_cmd(line));
        }
    }
    program
}

pub fn part_two(input: &str) -> Option<usize> {
    // Uncomment to derive the opcode enum usize values
    // let samples = parse_samples(input);
    // determine_opcodes(&samples);
    let program = parse_program(input);
    let mut device = Device::default();
    program.iter().for_each(|&cmd| device.execute(cmd));
    Some(device.reg[0])
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
        assert_eq!(part_one(&input), Some(1));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
