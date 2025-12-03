use crate::aoc::DaySolver;
use anyhow::{Result, bail};

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, _input: &Vec<String>) -> Result<i64> {
        bail!("😱 Part 1 not yet implemented!")
    }

    fn solve_part2(&self, _input: &Vec<String>) -> Result<i64> {
        bail!("😱 Part 2 not yet implemented!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_strings() -> Vec<String> {
        vec![""].into_iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_solve_part1() -> Result<()> {
        Ok(())
    }

    #[test]
    fn test_solve_part2() -> Result<()> {
        Ok(())
    }
}
