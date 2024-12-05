use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Level = i32;
type Report = Vec<Level>;

pub fn part1() -> Result<()> {
    let file = File::open("inputs/day_02.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut reports = Vec::<Report>::new();

    for line in reader.lines() {
        let mut report = Report::new();
        for token in line?.split_whitespace() {
            report.push(token.parse().unwrap());
        }
        reports.push(report);
    }

    let mut safe_reports = 0;
    for report in reports {
        if is_safe(&report) {
            safe_reports += 1;
        }
    }

    println!("Day 2, Part 1: Safe Reports: {}", safe_reports);

    Ok(())
}

pub fn part2() -> Result<()> {
    let file = File::open("inputs/day_02.txt").expect("File not found");
    let reader = BufReader::new(file);

    let mut reports = Vec::<Report>::new();

    for line in reader.lines() {
        let mut report = Report::new();
        for token in line?.split_whitespace() {
            report.push(token.parse().unwrap());
        }
        reports.push(report);
    }

    let mut safe_reports = 0;
    for report in reports {
        if is_safe(&report) {
            safe_reports += 1;
        } else {
            for i in 0..report.len() {
                let mut report_copy = report.clone();
                report_copy.remove(i);
                if is_safe(&report_copy) {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }

    println!("Day 2, Part 2: Safe Reports: {}", safe_reports);

    Ok(())
}

fn is_safe(report: &Report) -> bool {
    let mut safe = true;
    let mut increasing = false;
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff.abs() < 1 || diff.abs() > 3 {
            safe = false;
            break;
        }

        if i > 1 {
            if increasing && diff < 0 {
                safe = false;
                break;
            } else if !increasing && diff > 0 {
                safe = false;
                break;
            }
        } else {
            increasing = diff > 0;
        }
    }

    safe
}
