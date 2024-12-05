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

pub fn part2() -> Result<()> {
    let file = File::open("inputs/day_03.txt").expect("File not found");
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    reader
        .read_to_string(&mut buf)
        .expect("Failed to read file");

    let instruction_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let start_regex = Regex::new(r"do\(\)").unwrap();
    let stop_regex = Regex::new(r"don't\(\)").unwrap();

    let mut sum_of_valid_instructions = 0;
    let mut next_start_index = 0;
    let mut next_stop_index = stop_regex
        .find_at(&buf, 0)
        .map_or(usize::MAX, |m| m.start());

    for capture in instruction_regex.captures_iter(&buf) {
        let capture_start = capture.get(0).unwrap().start();
        if capture_start >= next_start_index && capture_start < next_stop_index {
            let a = capture[1].parse::<i32>().unwrap();
            let b = capture[2].parse::<i32>().unwrap();
            sum_of_valid_instructions += a * b;
        } else {
            next_start_index = start_regex
                .find_at(&buf, capture_start)
                .map_or(usize::MAX, |m| m.start());
            next_stop_index = stop_regex
                .find_at(&buf, capture_start)
                .map_or(usize::MAX, |m| m.start());
        }
    }

    println!(
        "Day 3, Part 2: Sum of valid instructions: {}",
        sum_of_valid_instructions
    );

    Ok(())
}
