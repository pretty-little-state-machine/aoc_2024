fn encode_snafu(mut input: isize) -> String {
    // Probably a better idea to use real division here, but we're messing with strings anyway
    let mut result = "".to_string();
    loop {
        let (n, carry) = match input % 5 {
            4 => ("-", 1),
            3 => ("=", 1),
            2 => ("2", 0),
            1 => ("1", 0),
            0 => ("0", 0),
            _ => unreachable!("Invalid digit"),
        };
        input /= 5;
        input += carry;
        result = format!("{n}{result}");
        if 0 == input {
            break;
        }
    }
    result
}

fn decode_snafu(input: &str) -> isize {
    let mut sum = 0;
    let mut base: isize = 1;
    input.chars().rev().for_each(|c| {
        sum += match c {
            '-' => -base,
            '=' => base * -2,
            x => x.to_string().parse::<isize>().unwrap() * base,
        };
        base *= 5;
    });
    sum
}

pub fn part_one(input: &str) -> Option<String> {
    let sum: isize = input.lines().map(decode_snafu).sum();
    Some(encode_snafu(sum))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 25);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_decoder() {
        decode_snafu("1=-0-2");

        let brochure: [(&str, isize); 13] = [
            ("1=-0-2", 1747),
            ("12111", 906),
            ("2=0=", 198),
            ("21", 11),
            ("2=01", 201),
            ("111", 31),
            ("20012", 1257),
            ("112", 32),
            ("1=-1=", 353),
            ("1-12", 107),
            ("12", 7),
            ("1=", 3),
            ("122", 37),
        ];
        for (snafu, decimal) in brochure {
            assert_eq!(snafu, encode_snafu(decimal));
            assert_eq!(decimal, decode_snafu(snafu));
        }
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
