use anyhow::Result;
use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn day_01_part1() -> Result<()> {
    let file = File::open("inputs/day_01_part1.txt").expect("File not found");
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;

    let mut left_locations = Vec::<u32>::new();
    let mut right_locations = Vec::<u32>::new();

    for (i, token) in buf
        .split(&[' ', '\n'])
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
        .enumerate()
    {
        if i % 2 == 0 {
            left_locations.push(token.parse().unwrap());
        } else {
            right_locations.push(token.parse().unwrap());
        }
    }

    left_locations.sort();
    right_locations.sort();

    let mut distance_sum = 0;
    for (i, left) in left_locations.iter().enumerate() {
        let right = right_locations[i];
        let distance = right.abs_diff(*left);
        distance_sum += distance;
    }

    println!("Day 1, Part 1: Distance sum: {}", distance_sum);

    Ok(())
}
