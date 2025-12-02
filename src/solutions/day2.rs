use std::collections::HashSet;

use anyhow::bail;

use crate::aoc::DaySolver;

pub struct Solver;

impl DaySolver for Solver {
    fn solve_part1(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        if input.len() != 1 {
            bail!("only one line expected")
        }
        let mut res = 0i64;
        for i in input[0].split(",") {
            let range = Range::from_str(i)?;
            res += range.get_sum() as i64;
        }
        Ok(res)
    }

    fn solve_part2(&self, input: &Vec<String>) -> anyhow::Result<i64> {
        if input.len() != 1 {
            bail!("only one line expected")
        }
        let mut res = 0i64;
        for i in input[0].split(",") {
            let range = Range::from_str(i)?;
            res += range.get_sum2() as i64;
        }
        Ok(res)
    }
}

struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn from_str(input: &str) -> anyhow::Result<Range> {
        let numbers: Vec<_> = input.split('-').collect();
        if numbers.len() != 2 {
            bail!("Wrong format for range: {}", input);
        }

        let min = numbers[0].parse()?;
        let max = numbers[1].parse()?;
        Ok(Range { min, max })
    }
    fn get_sum(self) -> u64 {
        (self.start()..)
            .try_fold(0, |acc, number| match self.check(number) {
                Some(x) => Ok(acc + x),
                None => Err(acc),
            })
            .unwrap_err()
    }

    fn get_sum2(self) -> u64 {
        let mut hash = HashSet::new();
        for number in 1.. {
            match self.check2(number) {
                Some(x) => hash.insert(x),
                None => break,
            };
        }
        hash.iter().sum()
    }

    fn start(&self) -> u64 {
        let digits = digits(self.min);
        if digits.is_multiple_of(2) {
            self.min / 10u64.pow(digits / 2)
        } else {
            10u64.pow(digits / 2)
        }
    }

    fn check(&self, num: u64) -> Option<u64> {
        let twin = num * 10u64.pow(digits(num)) + num;
        if twin > self.max {
            return None;
        }
        if twin >= self.min {
            Some(twin)
        } else {
            Some(0)
        }
    }

    fn check2(&self, pat: u64) -> Option<u64> {
        let pow = 10u64.pow(digits(pat));
        let mut twin = pat * pow + pat;
        if twin > self.max {
            return None;
        }
        loop {
            if twin >= self.min {
                return Some(twin);
            }
            twin = twin * pow + pat;
            if twin > self.max {
                return Some(0);
            }
        }
    }
}

fn digits(max: u64) -> u32 {
    (max as f64).log10() as u32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_getsum() {
        let range = Range { min: 11, max: 22 };
        assert_eq!(range.get_sum(), 33);
        let range = Range {
            min: 1188511880,
            max: 1188511890,
        };
        assert_eq!(range.get_sum(), 1188511885);
        let range = Range {
            min: 1698522,
            max: 1698528,
        };
        assert_eq!(range.get_sum(), 0);
    }

    #[test]
    fn test_start() {
        let range = Range { min: 99, max: 1999 };
        assert_eq!(range.start(), 9);
        let range = Range {
            min: 299,
            max: 1999,
        };
        assert_eq!(range.start(), 10);
        let range = Range {
            min: 3211,
            max: 19991,
        };
        assert_eq!(range.start(), 32);
    }

    #[test]
    fn test_solve_part1() {
        let input = vec![
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string()
        ];
        assert_eq!(Solver.solve_part1(&input).unwrap(), 1227775554);
    }

    #[test]
    fn test_solve_part2() {
        let input = vec![
         "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string()
        ];
        assert_eq!(Solver.solve_part2(&input).unwrap(), 4174379265);
    }
}
