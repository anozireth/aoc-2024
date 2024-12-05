use anyhow::Result;
use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn part1() -> Result<()> {
    let file = File::open("inputs/day_03.txt").expect("File not found");
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .expect("Failed to read file");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let sum_of_valid_instructions: i32 = re
        .captures_iter(&buf)
        .map(|captures| {
            let a = captures[1].parse::<i32>().unwrap();
            let b = captures[2].parse::<i32>().unwrap();
            a * b
        })
        .sum();

    println!(
        "Day 3, Part 1: Sum of valid instructions: {}",
        sum_of_valid_instructions
    );

    Ok(())
}
