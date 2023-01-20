use std::collections::VecDeque;

type Cursors = Vec<usize>;

#[allow(dead_code)]
fn debug(recipes: &VecDeque<usize>, cursors: &Cursors) {
    for (i, value) in recipes.iter().enumerate() {
        if i == cursors[0] {
            print!("({value})");
        } else if i == cursors[1] {
            print!("[{value}]");
        } else {
            print!(" {value} ");
        }
    }
    println!()
}

fn make_recipe(recipes: &mut Vec<usize>, cursors: &mut Cursors) {
    let (a, b) = (recipes[cursors[0]], recipes[cursors[1]]);
    let sum = recipes[cursors[0]] + recipes[cursors[1]];
    if sum / 10 > 0 {
        recipes.push(sum / 10);
    }
    recipes.push(sum % 10);
    cursors[0] = (cursors[0] + a + 1) % recipes.len();
    cursors[1] = (cursors[1] + b + 1) % recipes.len();
}

pub fn part_one(input: &str) -> Option<usize> {
    let recipe_count = input.parse::<usize>().unwrap();
    let mut recipes: Vec<usize> = Vec::with_capacity(recipe_count + 12); // Little extra
    recipes.push(3);
    recipes.push(7);
    let mut cursors: Cursors = vec![0, 1];

    while recipes.len() < recipe_count + 10 {
        make_recipe(&mut recipes, &mut cursors);
    }

    let mut total: usize = 0;
    let mut multiplier: usize = 1_000_000_000;
    for x in recipes.iter().skip(recipe_count).take(10) {
        total += x * multiplier;
        multiplier /= 10;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<usize> {
    let recipe_count = input.parse::<usize>().unwrap();
    let mut recipes: Vec<usize> = Vec::with_capacity(recipe_count + 12); // Little extra
    recipes.push(3);
    recipes.push(7);
    let mut cursors: Cursors = vec![0, 1];

    let recipe_digits = format!("{recipe_count}")
        .chars()
        .map(|b| b.to_string().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut count = 0;
    let num_digits = recipe_digits.len();
    loop {
        make_recipe(&mut recipes, &mut cursors);
        while num_digits + count < recipes.len() {
            if recipe_digits == recipes[count..(count + num_digits)] {
                return Some(count);
            }
            count += 1;
        }
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(5158916779));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(9));
    }
}
