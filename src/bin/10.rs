type Instruction = (char, isize);

#[derive(Debug)]
struct Cpu {
    x: isize,
    cycles: usize,
}

/// The CPU starts with an X Register value of 1!
impl Default for Cpu {
    fn default() -> Self {
        Self { x: 1, cycles: 0 }
    }
}

impl Cpu {
    #[inline(always)]
    pub fn snapshot_signal(&self, signal_strengths: &mut Vec<isize>) {
        if self.cycles % 20 == 0 {
            signal_strengths.push(self.x * self.cycles as isize)
        }
    }

    #[inline(always)]
    pub fn update_screen(&self, crt_screen: &mut [char; 240]) {
        let row_pixel = (self.cycles % 40) as isize;
        if self.x - 1 == row_pixel || self.x == row_pixel || self.x + 1 == row_pixel {
            crt_screen[self.cycles] = '█';
        }
    }
}

#[inline(always)]
pub fn read_instruction(input: &str) -> Instruction {
    let opcode = input.chars().next().unwrap();
    if opcode == 'a' {
        (opcode, input[5..].parse::<isize>().unwrap())
    } else {
        (opcode, 0) // NOOP
    }
}

/// Constructs the output for the screen in a 6x40 character display as a String with newlines.
/// Uses a technique from StackOverflow to make it quick without building Vecs & Strings:
/// See: https://stackoverflow.com/a/57032118
#[inline(always)]
fn build_crt_output(input: [char; 240]) -> String {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % 40 == 0 {
                Some('\n')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(*c))
        })
        .collect::<String>()
}

/// Reads the signal strength in the CPU every 20 cycles. All the fetch delays for the CPU are
/// handled outside the CPU for speed instead of keeping a state machine inside the CPU with a
/// program counter.
pub fn part_one(input: &str) -> Option<u32> {
    let mut cpu = Cpu::default();
    let mut signal_strengths = Vec::with_capacity(20);
    for line in input.lines() {
        let instruction = read_instruction(line);
        if 'a' == instruction.0 {
            // STAGE ADDX
            cpu.cycles += 1;
            cpu.snapshot_signal(&mut signal_strengths);
            //  EXECUTE ADDX
            cpu.cycles += 1;
            cpu.snapshot_signal(&mut signal_strengths);
            cpu.x += instruction.1;
        } else {
            // EXECUTE NOOP
            cpu.cycles += 1;
            cpu.snapshot_signal(&mut signal_strengths);
        }
    }
    let answer: isize = *signal_strengths.first().unwrap()
        + signal_strengths.iter().skip(2).step_by(2).sum::<isize>();
    Some(answer as u32)
}

/// Chase the beam and draw the display!
pub fn part_two(input: &str) -> Option<String> {
    let mut crt_screen = [' '; 6 * 40];
    let mut cpu = Cpu::default();
    for line in input.lines() {
        let instruction = read_instruction(line);
        cpu.update_screen(&mut crt_screen);
        if 'a' == instruction.0 {
            // STAGE ADDX
            cpu.cycles += 1;
            cpu.update_screen(&mut crt_screen);
            //  EXECUTE ADDX
            cpu.cycles += 1;
            cpu.x += instruction.1;
        } else {
            // EXECUTE NOOP
            cpu.cycles += 1;
        }
    }
    let answer = build_crt_output(crt_screen);
    Some(answer)
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);
        // Changed the result output tests a little cause I like this view better.
        let result = "██  ██  ██  ██  ██  ██  ██  ██  ██  ██  
███   ███   ███   ███   ███   ███   ███ 
████    ████    ████    ████    ████    
█████     █████     █████     █████     
██████      ██████      ██████      ████
███████       ███████       ███████     "
            .to_string();
        assert_eq!(part_two(&input), Some(result));
    }
}
