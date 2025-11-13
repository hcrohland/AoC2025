mod aoc;

use crate::aoc::{
    day1solver::Day1Solver, day2solver::Day2Solver, day3solver::Day3Solver, day4solver::Day4Solver,
    day5solver::Day5Solver, day6solver::Day6Solver, day7solver::Day7Solver, day8solver::Day8Solver,
    day9solver::Day9Solver, day10solver::Day10Solver, day11solver::Day11Solver,
    day12solver::Day12Solver, daysolver::*, filereader,
};
use std::env;

pub fn main() {
    let day: u8 = env::args().find_map(|arg| arg.parse().ok()).unwrap();
    println!("🎄 Selected day: {day}");

    let solver: Box<dyn DaySolver> = match day {
        1 => Box::new(Day1Solver),
        2 => Box::new(Day2Solver),
        3 => Box::new(Day3Solver),
        4 => Box::new(Day4Solver),
        5 => Box::new(Day5Solver),
        6 => Box::new(Day6Solver),
        7 => Box::new(Day7Solver),
        8 => Box::new(Day8Solver),
        9 => Box::new(Day9Solver),
        10 => Box::new(Day10Solver),
        11 => Box::new(Day11Solver),
        12 => Box::new(Day12Solver),
        _ => panic!(),
    };

    let input = filereader::read_file(&format!("input/{}.txt", day));

    match solver.solve_part1(&input) {
        Ok(i) => println!("🛷 The result of part 1 is {i}."),
        Err(e) => println!("{}", e),
    }

    match solver.solve_part2(&input) {
        Ok(i) => println!("🛷 The result of part 2 is {i}."),
        Err(e) => println!("{}", e),
    }
}
