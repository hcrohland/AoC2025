use std::fmt::Display;

use anyhow::bail;

use crate::solutions::*;

pub mod filereader;

pub trait DaySolver {
    fn solve_part1(&self, _input: &Vec<String>) -> anyhow::Result<i64> {
        bail!("😱 Part 1 not yet implemented!")
    }
    fn solve_part2(&self, _input: &Vec<String>) -> anyhow::Result<i64> {
        bail!("😱 Part 2 not yet implemented!")
    }
}

pub(crate) struct Day {
    day: u8,
    solver: Box<dyn DaySolver>,
}

impl Day {
    pub fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        self.solver.solve_part1(input)
    }

    pub fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        self.solver.solve_part2(input)
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.day.fmt(f)
    }
}

impl TryFrom<u8> for Day {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let day = value;
        let solver: Box<dyn DaySolver> = match value {
            1 => Box::new(day1::Solver),
            2 => Box::new(day2::Solver),
            3 => Box::new(day3::Solver),
            4..13 => bail!("day {value} not yet implemented"),
            _ => bail!("🚨 Day argument should be between 1 and 12"),
        };
        Ok(Day { day, solver })
    }
}
