mod day_01_part1;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Welcome to Advent of Code 2024!");
    day_01_part1::day_01_part1()?;

    Ok(())
}
