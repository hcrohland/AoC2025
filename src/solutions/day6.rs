use crate::aoc::DaySolver;
use anyhow::{Ok, bail};

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let input = parse_input(input);
        input.into_iter().map(calculate_row).sum()
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        let mut input: Vec<String> = input.to_owned();
        let ops = input.pop().unwrap();
        let transposed: Vec<Vec<char>> = transpose(
            input.into_iter().map(|s| s.chars().collect()).collect(),
            ' ',
        );
        let cols: Vec<String> = transposed
            .into_iter()
            .map(|v| v.into_iter().filter(|c| !c.is_whitespace()).collect())
            .collect();
        cols.split(|s| s == "")
            .zip(ops.split_whitespace())
            .map(calculate_col)
            .sum()
    }
}

fn calculate_row(mut x: Vec<&str>) -> anyhow::Result<i64> {
    match x.pop() {
        Some("+") => Ok(x.into_iter().map(|x| x.parse::<i64>().unwrap()).sum()),
        Some("*") => Ok(x.into_iter().map(|x| x.parse::<i64>().unwrap()).product()),
        _ => bail!("invalid input"),
    }
}

fn calculate_col((vec, op): (&[String], &str)) -> anyhow::Result<i64> {
    match op {
        "+" => Ok(vec.into_iter().map(|x| x.parse::<i64>().unwrap()).sum()),
        "*" => Ok(vec.into_iter().map(|x| x.parse::<i64>().unwrap()).product()),
        _ => bail!("invalid input"),
    }
}

fn transpose<T>(v: Vec<Vec<T>>, default: T) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| {
            v.iter()
                .map(|inner| inner.get(i).unwrap_or(&default).clone())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_input(input: &Vec<String>) -> Vec<Vec<&str>> {
    transpose(
        input
            .iter()
            .map(|s| s.split_whitespace().collect())
            .collect(),
        "",
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    fn test_strings() -> Vec<String> {
        "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
        .lines()
        .into_iter()
        .map(|x| x.to_string())
        .collect()
    }

    #[test]
    fn test_solve_part1() -> anyhow::Result<()> {
        assert_eq!(Solver.solve_part1(&test_strings())?, 4277556);
        Ok(())
    }

    #[test]
    fn test_solve_part2() -> anyhow::Result<()> {
        assert_eq!(Solver.solve_part2(&test_strings())?, 3263827);
        Ok(())
    }
}
