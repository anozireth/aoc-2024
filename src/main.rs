mod day_01;

use anyhow::Result;

fn main() -> Result<()> {
    println!("Welcome to Advent of Code 2024!");
    day_01::part1()?;
    day_01::part2()?;
    Ok(())
}
