#![feature(extract_if)]
#![feature(let_chains)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod helpers;

use colored::Color::{Green, Red};
use colored::*;
use std::fs;
use std::time::Duration;

type DayResult = (Option<Duration>, (String, Duration), (String, Duration));

fn run_day(day: usize, func: fn(&str) -> DayResult, color: Color) -> Duration {
    // Load the file before calling the function for accurate timing
    let contents =
        fs::read_to_string(format!("./src/input/day_{:0>2}.txt", day)).expect("File not found.");

    let (parse_duration, (p1, p1_duration), (p2, p2_duration)) = func(&contents);
    let mut total_duration = p1_duration + p2_duration;
    if let Some(p) = parse_duration {
        total_duration += p;
    }

    let title = match color {
        Red => format!("🎄Day {day} ({total_duration:?}) 🎄\n~~~~~~~~~~~~~~~~~~~~~").bright_red(),
        Green => {
            format!("🎄Day {day} ({total_duration:?}) 🎄\n~~~~~~~~~~~~~~~~~~~~~").bright_green()
        }
        _ => format!("🎄Day {day} ({total_duration:?}) 🎄\n~~~~~~~~~~~~~~~~~~~~~").white(),
    };
    println!("{title}");
    if let Some(p) = parse_duration {
        println!("Parse : ({p:?})");
    }
    print!("{}", "Part 1: ".white());
    print!("{}", p1.as_str().bold().white());
    println!(" ({p1_duration:?})");
    print!("{}", "Part 2: ".white());
    print!("{}", p2.as_str().bold().white());
    println!(" ({p2_duration:?})\n");
    total_duration
}

fn main() {
    let mut final_runtime = Duration::new(0, 0);
    final_runtime += run_day(1, day01::run, Red);
    final_runtime += run_day(2, day02::run, Green);
    final_runtime += run_day(3, day03::run, Red);
    final_runtime += run_day(4, day04::run, Green);
    final_runtime += run_day(6, day06::run, Green);
    final_runtime += run_day(7, day07::run, Red);
    final_runtime += run_day(8, day08::run, Green);

    print!("{}", "Final Runtime: ".to_string().bold().white());
    if final_runtime < Duration::new(0, 800_000_000) {
        println!("{}", format!("{final_runtime:?}\n").bold().green());
    } else if final_runtime < Duration::new(0, 0) {
        println!("{}", format!("{final_runtime:?}\n").bold().yellow());
    } else {
        println!("{}", format!("{final_runtime:?}\n").bold().red());
    }
}
