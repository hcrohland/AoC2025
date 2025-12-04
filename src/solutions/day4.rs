use std::ops::{Deref, DerefMut};

use crate::aoc::DaySolver;
use anyhow::bail;

pub struct Solver;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(i8)]
enum Position {
    Empty = 0,
    Roll = 1,
}

impl TryFrom<char> for Position {
    type Error = anyhow::Error;

    fn try_from(value: char) -> std::result::Result<Self, Self::Error> {
        use Position::*;
        Ok(match value {
            '.' => Empty,
            '@' => Roll,
            _ => bail!("Invalid character"),
        })
    }
}

impl Position {
    fn parse<S: AsRef<str>>(input: S) -> anyhow::Result<Vec<Position>> {
        input.as_ref().chars().map(|c| c.try_into()).collect()
    }
}

#[derive(Debug, PartialEq)]
struct Area(Vec<Vec<Position>>);

impl Area {
    fn new<S: AsRef<str>>(input: &Vec<S>) -> anyhow::Result<Area> {
        Ok(Area(
            input
                .iter()
                .map(Position::parse)
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }

    fn solve(&self) -> anyhow::Result<i64> {
        let mut res = 0i64;
        for row in 0..self.len() {
            for col in 0..self[row].len() {
                if self.check_position(row, col) {
                    res += 1;
                }
            }
        }
        Ok(res)
    }

    fn check_position(&self, row: usize, col: usize) -> bool {
        if self[row][col] == Position::Empty {
            return false;
        }
        let mut neighbors = 0;
        let rstart = if row > 0 { row - 1 } else { 0 };
        let cstart = if col > 0 { col - 1 } else { 0 };
        for r in rstart..row + 2 {
            for c in cstart..col + 2 {
                if self.get(r).and_then(|col| col.get(c)) == Some(&Position::Roll) {
                    neighbors += 1;
                }
            }
        }
        if neighbors < 5 { true } else { false }
    }

    fn solve2(&mut self) -> Result<i64, anyhow::Error> {
        let mut res = 0i64;
        loop {
            let last = res;
            for row in 0..self.len() {
                for col in 0..self[row].len() {
                    if self.check_position(row, col) {
                        self[row][col] = Position::Empty;
                        res += 1;
                    }
                }
            }
            if last == res {
                return Ok(res);
            }
        }
    }
}

impl Deref for Area {
    type Target = Vec<Vec<Position>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Area {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        Area::new(input)?.solve()
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        Area::new(input)?.solve2()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_strings() -> Vec<String> {
        "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .lines()
            .map(|x| x.to_string())
            .collect()
    }

    #[test]
    fn test_utils() -> anyhow::Result<()> {
        let area = Area::new(&test_strings())?;
        // assert_eq!(area.check_position(1, 1), 0);
        // assert_eq!(area.check_position(0, 0), 0);
        assert_eq!(area.check_position(0, 3), true);
        // assert_eq!(area.check_position(0, 1), 0);
        // assert_eq!(area.check_position(2, 6), 1);
        Ok(())
    }

    #[test]
    fn test_solve_part1() -> anyhow::Result<()> {
        assert_eq!(Solver.solve_part1(&test_strings())?, 13);
        Ok(())
    }

    #[test]
    fn test_solve_part2() -> anyhow::Result<()> {
        assert_eq!(Solver.solve_part2(&test_strings())?, 43);
        Ok(())
    }
}
