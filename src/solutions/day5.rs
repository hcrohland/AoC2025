use std::{ops::Deref, str::FromStr};

use crate::aoc::DaySolver;
use anyhow::anyhow;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let (fresh, ids) = parse_input(input)?;
        Ok(ids.iter().fold(
            0i64,
            |acc, item| if fresh.check_id(*item) { acc + 1 } else { acc },
        ))
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let (fresh, _) = parse_input(input)?;
        Ok(fresh.get_id_sum())
    }
}

struct Range {
    min: i64,
    max: i64,
}

impl FromStr for Range {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rangevec = s
            .split("-")
            .map(|x| x.parse::<i64>())
            .collect::<Result<Vec<_>, _>>()?;
        let min = *rangevec.get(0).ok_or(anyhow!("parse error"))?;
        let max = *rangevec.get(1).ok_or(anyhow!("parse error"))? + 1;
        Ok(Range { min, max })
    }
}

struct Fresh(Vec<Range>);

impl Fresh {
    fn new(input: &[String]) -> anyhow::Result<Fresh> {
        let mut f = input
            .iter()
            .map(|x| x.parse())
            .collect::<Result<Vec<_>, _>>()?;
        f.sort_by(|a: &Range, b: &Range| a.min.cmp(&b.min));
        Ok(Fresh(f))
    }

    fn check_id(&self, item: i64) -> bool {
        self.iter()
            .any(|range| range.min <= item && range.max >= item)
    }

    fn get_id_sum(&self) -> i64 {
        let mut end = 0i64;
        self.iter().fold(0, |res, range| {
            let new = (range.max - range.min.max(end)).max(0);
            end = end.max(range.max);
            res + new
        })
    }
}

fn parse_input(input: &Vec<String>) -> anyhow::Result<(Fresh, Vec<i64>)> {
    let mut splits = input.split(|s| s.len() == 0);

    let fresh = Fresh::new(splits.next().ok_or(anyhow!("invalid input"))?)?;
    let ids = splits
        .next()
        .ok_or(anyhow!("invalid input"))?
        .iter()
        .map(|x| x.parse::<i64>())
        .collect::<Result<_, _>>()?;
    Ok((fresh, ids))
}

impl Deref for Fresh {
    type Target = Vec<Range>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_strings() -> Vec<String> {
        "3-5
10-14
16-20
12-18

1
5
8
11
17
32

"
        .lines()
        .into_iter()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_solve_part1() -> anyhow::Result<()> {
        assert_eq!(Solver.solve_part1(&test_strings())?, 3);
        Ok(())
    }

    #[test]
    fn test_solve_part2() -> anyhow::Result<()> {
        assert_eq!(Solver.solve_part2(&test_strings())?, 14);
        Ok(())
    }
}
