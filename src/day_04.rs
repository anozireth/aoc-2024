use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn part1() -> Result<()> {
    let file = File::open("inputs/day_04.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut word_search = Vec::<Vec<char>>::new();

    for line_str in reader.lines() {
        let line_str = line_str.unwrap();
        let mut word = Vec::<char>::new();
        for c in line_str.chars() {
            word.push(c);
        }
        word_search.push(word);
    }

    let mut xmas_count: i32 = 0;

    for row in 0..word_search.len() as i32 {
        for col in 0..word_search[row as usize].len() as i32 {
            if check_xmas(&word_search, row, col, 1, 1) {
                xmas_count += 1;
            }
            if check_xmas(&word_search, row, col, 1, 0) {
                xmas_count += 1;
            }
            if check_xmas(&word_search, row, col, 1, -1) {
                xmas_count += 1;
            }
            if check_xmas(&word_search, row, col, 0, 1) {
                xmas_count += 1;
            }
            if check_xmas(&word_search, row, col, 0, -1) {
                xmas_count += 1;
            }
            if check_xmas(&word_search, row, col, -1, 1) {
                xmas_count += 1;
            }
            if check_xmas(&word_search, row, col, -1, 0) {
                xmas_count += 1;
            }
            if check_xmas(&word_search, row, col, -1, -1) {
                xmas_count += 1;
            }
        }
    }

    println!("Day 4, Part 1: XMAS count: {}", xmas_count);

    Ok(())
}

fn check_xmas(
    word_search: &Vec<Vec<char>>,
    row: i32,
    col: i32,
    row_increment: i32,
    col_increment: i32,
) -> bool {
    let search_x_size = word_search[0].len() as i32;
    let search_y_size = word_search.len() as i32;

    // First, check if the current position is an X
    if word_search[row as usize][col as usize] != 'X' {
        return false;
    }

    // Next, check bounds
    if (row + row_increment * 3) >= search_y_size
        || (row + row_increment * 3) < 0
        || (col + col_increment * 3) >= search_x_size
        || (col + col_increment * 3) < 0
    {
        return false;
    }

    // Finally, check if the next three positions are M, A and S
    if word_search[(row + row_increment) as usize][(col + col_increment) as usize] != 'M'
        || word_search[(row + row_increment * 2) as usize][(col + col_increment * 2) as usize]
            != 'A'
        || word_search[(row + row_increment * 3) as usize][(col + col_increment * 3) as usize]
            != 'S'
    {
        return false;
    }

    true
}

pub fn part2() -> Result<()> {
    let file = File::open("inputs/day_04.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut word_search = Vec::<Vec<char>>::new();

    for line_str in reader.lines() {
        let line_str = line_str.unwrap();
        let mut word = Vec::<char>::new();
        for c in line_str.chars() {
            word.push(c);
        }
        word_search.push(word);
    }

    let mut x_mas_count: i32 = 0;

    for row in 0..word_search.len() as i32 {
        for col in 0..word_search[row as usize].len() as i32 {
            if check_x_mas(&word_search, row, col) {
                x_mas_count += 1;
            }
        }
    }

    println!("Day 4, Part 2: X-MAS count: {}", x_mas_count);

    Ok(())
}

fn check_x_mas(word_search: &Vec<Vec<char>>, row: i32, col: i32) -> bool {
    let search_x_size = word_search[0].len() as i32;
    let search_y_size = word_search.len() as i32;

    // First, check if the current position is an A
    if word_search[row as usize][col as usize] != 'A' {
        return false;
    }

    // Next, check bounds
    if (row + 1) >= search_y_size || (row - 1) < 0 || (col + 1) >= search_x_size || (col - 1) < 0 {
        return false;
    }

    if word_search[(row - 1) as usize][(col - 1) as usize] == 'M'
        && word_search[(row + 1) as usize][(col + 1) as usize] == 'S'
        && word_search[(row + 1) as usize][(col - 1) as usize] == 'M'
        && word_search[(row - 1) as usize][(col + 1) as usize] == 'S'
    {
        return true;
    }

    if word_search[(row - 1) as usize][(col + 1) as usize] == 'M'
        && word_search[(row + 1) as usize][(col - 1) as usize] == 'S'
        && word_search[(row - 1) as usize][(col - 1) as usize] == 'M'
        && word_search[(row + 1) as usize][(col + 1) as usize] == 'S'
    {
        return true;
    }

    if word_search[(row + 1) as usize][(col - 1) as usize] == 'M'
        && word_search[(row - 1) as usize][(col + 1) as usize] == 'S'
        && word_search[(row + 1) as usize][(col + 1) as usize] == 'M'
        && word_search[(row - 1) as usize][(col - 1) as usize] == 'S'
    {
        return true;
    }

    if word_search[(row + 1) as usize][(col + 1) as usize] == 'M'
        && word_search[(row - 1) as usize][(col - 1) as usize] == 'S'
        && word_search[(row - 1) as usize][(col + 1) as usize] == 'M'
        && word_search[(row + 1) as usize][(col - 1) as usize] == 'S'
    {
        return true;
    }

    false
}
