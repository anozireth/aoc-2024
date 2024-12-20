use anyhow::Result;
use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn part1() -> Result<()> {
    let file = File::open("inputs/day_01.txt").expect("File not found");
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

pub fn part2() -> Result<()> {
    let file = File::open("inputs/day_01.txt").expect("File not found");
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

    let mut similarity_score = 0;
    for left in left_locations {
        let right_count = right_locations.iter().filter(|&r| *r == left).count();
        similarity_score += left * right_count as u32;
    }

    println!("Day 1, Part 2: Similarity Score: {}", similarity_score);

    Ok(())
}
