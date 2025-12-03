use std::{ops::Deref, str::FromStr};

use crate::aoc::DaySolver;
use anyhow::{Result, anyhow};

pub struct Solver;

impl Solver {
    fn solve_part(&self, input: &Vec<String>, size: usize) -> Result<i64> {
        input.iter().try_fold(0, |acc, string| {
            let battery: Battery = string.parse()?;
            Ok(acc + battery.get_joltage(size))
        })
    }
}

#[derive(Debug, PartialEq)]
struct Battery(Vec<u32>);

impl Battery {
    fn get_index_for_max(&self, start: usize, rest: usize) -> usize {
        self[start..self.len() - rest]
            .iter()
            .enumerate()
            .fold(
                0,
                |acc, (index, num)| if *num > self[acc + start] { index } else { acc },
            )
            + start
    }

    fn get_joltage(&self, size: usize) -> i64 {
        (0..size)
            .rfold((0i64, 0), |(res, start), rest| {
                let first = self.get_index_for_max(start, rest);
                (res * 10 + self[first] as i64, first + 1)
            })
            .0
    }
}

impl FromStr for Battery {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let vec = s
            .chars()
            .try_fold(Vec::new(), |mut acc, c| {
                acc.push(c.to_digit(10)?);
                Some(acc)
            })
            .ok_or(anyhow!("Nondigits found in {s}"))?;
        Ok(Battery(vec))
    }
}

impl Deref for Battery {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> Result<i64> {
        self.solve_part(input, 2)
    }

    fn solve_part2(&self, input: &Vec<String>) -> Result<i64> {
        self.solve_part(input, 12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_strings() -> Vec<String> {
        vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ]
        .iter()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_utils() -> Result<()> {
        let battery: Battery = "676543211111119".parse()?;
        assert_eq!(
            battery,
            Battery(vec![6, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1, 9])
        );
        assert_eq!(battery.get_index_for_max(0, 1), 1);
        assert_eq!(battery.get_joltage(2), 79);
        Ok(())
    }

    #[test]
    fn test_solve_part1() -> Result<()> {
        let input = test_strings();
        assert_eq!(Solver.solve_part1(&input)?, 357);
        Ok(())
    }

    #[test]
    fn test_solve_part2() -> Result<()> {
        let input = test_strings();
        assert_eq!(Solver.solve_part2(&input)?, 3121910778619);
        Ok(())
    }
}
