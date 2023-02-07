use crate::num_traits::FromPrimitive;
use crate::Opcode::{
    Addi, Addr, Bani, Banr, Bori, Borr, Eqir, Eqri, Eqrr, Gtir, Gtri, Gtrr, Muli, Mulr, Seti, Setr,
};
use rustc_hash::FxHashSet;

#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

#[derive(Default, Debug)]
struct Device {
    reg: [usize; 6],
    ip_binding: Option<usize>,
    instr_ptr: usize,
}

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

impl Opcode {
    fn from_str(input: &str) -> Self {
        match input {
            "addr" => Addr,
            "addi" => Addi,
            "mulr" => Mulr,
            "muli" => Muli,
            "banr" => Banr,
            "bani" => Bani,
            "borr" => Borr,
            "bori" => Bori,
            "setr" => Setr,
            "seti" => Seti,
            "gtir" => Gtir,
            "gtri" => Gtri,
            "gtrr" => Gtrr,
            "eqir" => Eqir,
            "eqri" => Eqri,
            "eqrr" => Eqrr,
            _ => unreachable!("Unsupported opcode: {input}"),
        }
    }
}

impl Device {
    fn execute(&mut self, [op, a, b, c]: [usize; 4]) {
        if let Some(ip_binding) = self.ip_binding {
            self.reg[ip_binding] = self.instr_ptr
        }
        match Opcode::from_usize(op).expect("Unknown Opcode") {
            Addr => self.reg[c] = self.reg[a] + self.reg[b],
            Addi => self.reg[c] = self.reg[a] + b,
            Mulr => self.reg[c] = self.reg[a] * self.reg[b],
            Muli => self.reg[c] = self.reg[a] * b,
            Banr => self.reg[c] = self.reg[a] & self.reg[b],
            Bani => self.reg[c] = self.reg[a] & b,
            Borr => self.reg[c] = self.reg[a] | self.reg[b],
            Bori => self.reg[c] = self.reg[a] | b,
            Setr => self.reg[c] = self.reg[a],
            Seti => self.reg[c] = a,
            Gtir => self.reg[c] = if a > self.reg[b] { 1 } else { 0 },
            Gtri => self.reg[c] = if self.reg[a] > b { 1 } else { 0 },
            Gtrr => self.reg[c] = if self.reg[a] > self.reg[b] { 1 } else { 0 },
            Eqir => self.reg[c] = if a == self.reg[b] { 1 } else { 0 },
            Eqri => self.reg[c] = if self.reg[a] == b { 1 } else { 0 },
            Eqrr => self.reg[c] = if self.reg[a] == self.reg[b] { 1 } else { 0 },
        }
        if let Some(ip_binding) = self.ip_binding {
            self.instr_ptr = self.reg[ip_binding];
            self.instr_ptr += 1;
        }
    }
}

/// Returns a tuple of the Instruction Pointer binding and the program's code
fn parse_input(input: &str) -> (usize, Vec<[usize; 4]>) {
    let mut program = Vec::new();
    let mut pointer_binding = 0;
    for line in input.lines() {
        if line.contains('#') {
            let reg = line.split(' ').collect::<Vec<&str>>();
            pointer_binding = reg.get(1).unwrap().parse::<usize>().unwrap();
        } else {
            let s = line.split(' ').collect::<Vec<&str>>();
            program.push([
                Opcode::from_str(s[0]) as usize,
                s[1].parse::<usize>().unwrap(),
                s[2].parse::<usize>().unwrap(),
                s[3].parse::<usize>().unwrap(),
            ]);
        }
    }
    (pointer_binding, program)
}

/// Returns the lowest value allowed in Register 0 to HALT the machine
pub fn part_one(input: &str) -> Option<usize> {
    let mut device = Device::default();
    let (pointer, program) = parse_input(input);
    device.ip_binding = Some(pointer);
    let mut target_register: usize;

    const BREAKPOINT: usize = 28; // Equality check preventing HALT instruction
    loop {
        if let Some(command) = program.get(device.instr_ptr) {
            // Automatically determine the register based on the command's "A" register
            target_register = command[1];
            if device.instr_ptr == BREAKPOINT {
                break;
            }
            device.execute(*command);
        }
    }
    // Register 3 contains the value checked against Register 0 to allow for a HALT
    Some(device.reg[target_register])
}

/// Returns the highest value allowed in Register 0 to HALT the machine
pub fn part_two(input: &str) -> Option<usize> {
    let mut device = Device::default();
    let (pointer, program) = parse_input(input);
    device.ip_binding = Some(pointer);

    const SNAPSHOT: usize = 28; // Equality check preventing HALT instruction
    let mut seen_values = FxHashSet::default();
    let mut last_value = 0;
    loop {
        if let Some(command) = program.get(device.instr_ptr) {
            if device.instr_ptr == SNAPSHOT {
                // Automatically determine the register based on the command's "A" register
                let r = device.reg[command[1]];
                if seen_values.contains(&r) {
                    break;
                } else {
                    seen_values.insert(r);
                    last_value = r;
                }
            }
            device.execute(*command);
        }
    }
    Some(last_value)
}

fn main() {
    let input = &aoc::read_file("inputs", 21);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(3173684));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 21);
        assert_eq!(part_two(&input), None);
    }
}
