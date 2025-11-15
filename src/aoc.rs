use std::fmt::Display;

use anyhow::bail;

use daysolver::DaySolver;
use crate::solutions::*;

pub mod daysolver;
pub mod filereader;

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
        write!(f, "{}", self.day)
    }
}

impl TryFrom<u8> for Day {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let day = value;
        let solver = match value {
            1 => day1solver::Day1Solver,
            2..12 => bail!("day {value} not yet implemented"),
            _ => bail!("🚨 Day argument should be between 1 and 12"),
        };
        let solver = Box::new(solver);
        Ok(Day { day, solver })
    }
}
