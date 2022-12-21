use rustc_hash::FxHashMap;
use crate::MonkeyAction::{Yell, Add, Divide, Multiply, Subtract};

type Monkeys = FxHashMap<String, Monkey>;

#[derive(Clone, Default, Debug)]
struct Monkey {
    action: MonkeyAction,
    value: usize,
    left: Option<String>,
    right: Option<String>,
}

#[derive(Clone, Default, Debug, Eq, PartialEq)]
enum MonkeyAction {
    #[default]
    Yell,
    Add,
    Divide,
    Multiply,
    Subtract,
}

fn build_monkeys(input: &str, monkeys: &mut Monkeys) {
    let name = input[0..4].to_string();
    let mut monkey = Monkey::default();
    if input.len() > 10 {
        monkey.right = Some(input[6..10].to_string());
        monkey.left = Some(input[13..17].to_string());
        monkey.action = match input.chars().nth(11).unwrap() {
            '+' => Add,
            '/' => Divide,
            '*' => Multiply,
            '-' => Subtract,
            _ => unreachable!("Invalid MonkeyAction Operation")
        }
    } else {
        monkey.action = Yell;
        monkey.value = input[6..].parse::<usize>().unwrap();
    };
    monkeys.insert(name, monkey);
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = FxHashMap::default();
    input.lines().for_each(|l| build_monkeys(l, &mut monkeys));


    let mut stack: Vec<String> = Vec::with_capacity(monkeys.len());
    stack.push("root".to_string());
    while stack.len() > 0 {
        let monkey = monkeys.get(&*stack.pop().unwrap()).unwrap();
        println!("{:?}", monkey);
    }

    /*
    let key_map = monkeys.clone();
    let keys = key_map.keys().collect::<Vec<&String>>();
    // Horrific lol
    loop {
        if monkeys.get(&"root".to_string()).unwrap().action == Yell {
            break;
        }
        // Resolve as much as we can each pass
        for key in &keys {
            let m = monkeys.get(*key).unwrap();
            if m.action == Yell {
                continue;
            }
            let a = monkeys.get(&*m.right).unwrap().clone();
            let b = monkeys.get(&*m.left).unwrap().clone();
            let m = monkeys.get_mut(*key).unwrap();

            if a.action == Yell && b.action == Yell {
                match m.action {
                    Yell => (),
                    Add => m.value = a.value + b.value,
                    Divide => m.value = a.value / b.value,
                    Multiply => m.value = a.value * b.value,
                    Subtract => m.value = a.value - b.value,
                }
                m.action = Yell;
            }
        }
    }
    Some(monkeys.get(&*"root".to_string()).unwrap().value)

     */
    Some(0)
}

pub fn part_two(input: &str) -> Option<usize> {
    /*
     let mut original_monkeys = FxHashMap::default();
     input.lines().for_each(|l| build_monkeys(l, &mut original_monkeys));
     let key_map = original_monkeys.clone();
     let keys = key_map.keys().collect::<Vec<&String>>();

     // Even worse, but I really just want a win. Please advent, I'm beggin' ya
     let mut number_to_yell = 0;
     'num_gen: for test_number in 0..10_000_000 {
         let mut monkeys = original_monkeys.clone();
         number_to_yell = test_number;
         monkeys.get_mut(&"humn".to_string()).unwrap().value = test_number;
         loop {
             // Actually the equality check, we can ignore the root value and compare children
             let root = monkeys.get(&"root".to_string()).unwrap();
             if root.action == Yell {
                 let a = monkeys.get(root.children.first().unwrap()).unwrap().clone();
                 let b = monkeys.get(root.children.get(1).unwrap()).unwrap().clone();
                 if a.value == b.value {
                     break 'num_gen;
                 }
                 break;
             }
             // Resolve as much as we can each pass
             for key in &keys {
                 let m = monkeys.get(*key).unwrap();
                 if m.action == Yell {
                     continue;
                 }
                 let a = monkeys.get(m.children.first().unwrap()).unwrap().clone();
                 let b = monkeys.get(m.children.get(1).unwrap()).unwrap().clone();
                 let m = monkeys.get_mut(*key).unwrap();

                 if a.action == Yell && b.action == Yell {
                     match m.action {
                         Yell => (),
                         Add => m.value = a.value + b.value,
                         Divide => m.value = a.value / b.value,
                         Multiply => m.value = a.value * b.value,
                         Subtract => m.value = a.value.saturating_sub(b.value),
                     }
                     m.action = Yell;
                 }
             }
         }
     }
     Some(number_to_yell)

     */
    Some(0)
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
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
