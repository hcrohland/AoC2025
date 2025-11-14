mod aoc;

use crate::aoc::{
    day1solver::Day1Solver, day2solver::Day2Solver, day3solver::Day3Solver, day4solver::Day4Solver,
    day5solver::Day5Solver, day6solver::Day6Solver, day7solver::Day7Solver, day8solver::Day8Solver,
    day9solver::Day9Solver, day10solver::Day10Solver, day11solver::Day11Solver,
    day12solver::Day12Solver, daysolver::*, filereader,
};
use std::{env, time::Instant};

pub fn main() {
    let day: u8 = match env::args().find_map(|arg| arg.parse().ok()) {
        Some(i) => i,
        None => {
            eprintln!("🚨 Missing or malformed day argument");
            return;
        }
    };

    match day {
        1 => run(day, Day1Solver),
        2 => run(day, Day2Solver),
        3 => run(day, Day3Solver),
        4 => run(day, Day4Solver),
        5 => run(day, Day5Solver),
        6 => run(day, Day6Solver),
        7 => run(day, Day7Solver),
        8 => run(day, Day8Solver),
        9 => run(day, Day9Solver),
        10 => run(day, Day10Solver),
        11 => run(day, Day11Solver),
        12 => run(day, Day12Solver),
        _ => eprintln!("🚨 Day argument should be between 1 and 12"),
    }
}

fn run(day: u8, solver: impl DaySolver) {
    let input = match filereader::read_file(&format!("input/{}.txt", day)) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("🚨 Error reading input: {}", e);
            return;
        }
    };

    println!("🎄 Selected day: {day}");
    let start = Instant::now();
    match solver.solve_part1(&input) {
        Ok(i) => println!(
            "🛷 The result of part 1 is {i} (calculated in {} µs).",
            start.elapsed().as_micros()
        ),
        Err(e) => println!("{}", e),
    }

    let start = Instant::now();
    match solver.solve_part2(&input) {
        Ok(i) => println!(
            "🛷 The result of part 2 is {i} (calculated in {} µs).",
            start.elapsed().as_micros()
        ),
        Err(e) => println!("{}", e),
    }
}
