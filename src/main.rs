mod day_01;
mod day_02;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Welcome to Advent of Code 2024!");
    day_01::part1()?;
    day_01::part2()?;

    day_02::part1()?;
    day_02::part2()?;
    Ok(())
}
