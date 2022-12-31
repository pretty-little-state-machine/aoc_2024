/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use aoc::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
use std::process::Command;

fn main() {
    let total: f64 = (1..=25)
        .map(|day| {
            let day = format!("{day:02}");

            let cmd = Command::new("cargo")
                .args(["run", "--release", "--bin", &day])
                .output()
                .unwrap();

            println!("----------");
            println!("{ANSI_BOLD}| Day {day} |{ANSI_RESET}");
            println!("----------");

            let output = String::from_utf8(cmd.stdout).unwrap();
            let is_empty = output.is_empty();

            println!(
                "{}",
                if is_empty {
                    "Not solved."
                } else {
                    output.trim()
                }
            );

            if is_empty {
                0_f64
            } else {
                aoc::parse_exec_time(&output)
            }
        })
        .sum();

    println!("{ANSI_BOLD}Total:{ANSI_RESET} {ANSI_ITALIC}{total:.2}ms{ANSI_RESET}");
}
