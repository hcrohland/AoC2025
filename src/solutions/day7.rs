use std::ops::{Deref, Mul};

use crate::aoc::DaySolver;
use anyhow::{Result, anyhow};

pub struct Solver;
impl Solver {
    fn solve(&self, input: &Vec<String>) -> anyhow::Result<Rays> {
        let mut input = input.iter();
        let rays: Rays = input.next().unwrap().try_into()?;
        input
            .map(|str| str.try_into())
            .try_fold(rays, |r, splitter| {
                Ok::<Rays, anyhow::Error>(r * &splitter?)
            })
    }
}

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        Ok(Solver.solve(input)?.splits)
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        Ok(Solver.solve(input)?.ways)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Position {
    Empty,
    Split,
}

#[derive(Debug)]
struct Splitter(Vec<Position>);

impl Deref for Splitter {
    type Target = Vec<Position>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct Rays {
    splits: i64,
    ways: i64,
    rays: Vec<i64>,
}

impl Deref for Rays {
    type Target = Vec<i64>;

    fn deref(&self) -> &Self::Target {
        &self.rays
    }
}

impl TryFrom<&String> for Rays {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Ok(Rays {
            splits: 0,
            ways: 1,
            rays: value
                .chars()
                .map(|c| match c {
                    '.' => Ok(0),
                    'S' => Ok(1),
                    x => Err(anyhow!("Invalid input: {x}")),
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl Mul<&Splitter> for Rays {
    type Output = Rays;

    fn mul(mut self, rhs: &Splitter) -> Self::Output {
        for (i, pos) in rhs.iter().enumerate() {
            if self.rays[i] != 0 {
                if *pos == Position::Split {
                    let rays = self.rays[i];
                    self.splits += 1;
                    self.ways += rays;
                    self.rays[i] = 0;
                    if i > 0 {
                        self.rays[i - 1] += rays;
                    };
                    if i < self.rays.len() + 1 {
                        self.rays[i + 1] += rays;
                    }
                }
            }
        }
        self
    }
}

impl TryFrom<&String> for Splitter {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Ok(Splitter(
            value
                .chars()
                .map(|c| match c {
                    '.' => Ok(Position::Empty),
                    '^' => Ok(Position::Split),
                    x => Err(anyhow!("Invalid input: {x}")),
                })
                .collect::<Result<Vec<_>>>()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_strings() -> Vec<String> {
        ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            .lines()
            .into_iter()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn test_solve_part1() -> anyhow::Result<()> {
        assert_eq!(Solver.solve_part1(&test_strings())?, 21);
        Ok(())
    }

    #[test]
    fn test_solve_part2() -> anyhow::Result<()> {
        assert_eq!(Solver.solve_part2(&test_strings())?, 40);
        Ok(())
    }
}
