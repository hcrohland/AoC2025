mod aoc;
mod solutions;

use std::{env, time::Instant};

use anyhow::{Context, bail};

use crate::aoc::Day;

pub fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect();
    let day: u8 = match args.len() {
        2 => args[1].parse().context("Please provide the day number")?,
        _ => bail!("We need exactly one integer argument"),
    };

    let day: Day = day.try_into()?;

    let input = aoc::filereader::read_file(&format!("input/{}.txt", day))
        .context("Could not read input file")?;

    println!("🎄 Selected day: {day}");
    let start = Instant::now();
    let i = day.solve_part1(&input)?;
    println!(
        "🛷 The result of part 1 is {i} (calculated in {} µs).",
        start.elapsed().as_micros()
    );

    let start = Instant::now();
    let i = day.solve_part2(&input)?;
    println!(
        "🛷 The result of part 2 is {i} (calculated in {} µs).",
        start.elapsed().as_micros()
    );
    Ok(())
}
