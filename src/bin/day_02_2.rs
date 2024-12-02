use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::zip;

const OUTPUT_MESSAGE: &str = "Number of safe reports";
const INPUT_FILE: &str = "input/day_02.txt";

fn check_report_safety(report: &Vec<u32>) -> bool {
    let increasing = report[1] > report[0];
    let mut diffs: Vec<u32> = Vec::new();
    for i in 1..report.len() {
        if (increasing & (report[i] < report[i - 1]))
            | (!increasing & (report[i] > report[i - 1]))
        { return false; }
        diffs.push(report[i].abs_diff(report[i - 1]));
    }
    let diffs_ok = diffs.iter().map(|d| (*d >= 1) & (*d <= 3)).reduce(|acc, e| acc & e).unwrap();
    diffs_ok
}

fn clone_without(vec: &Vec<u32>, i: usize) -> Vec<u32> {
    let mut vec = vec.clone();
    vec.remove(i);
    vec
}

fn check_report_safety_with_dampener(report: &Vec<u32>) -> bool {
    for i in 0..report.len() {
        if check_report_safety(&clone_without(&report, i)) { return true; }
    }
    false
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u32, ()> {
    let reports: Vec<Vec<u32>> = lines
        .filter_map(|line| {
            let nums: Vec<u32> = line
                .unwrap()
                .split(" ")
                .filter_map(|n| n.parse::<u32>().ok())
                .collect();
            if nums.len() > 0 { Some(nums) } else { None }
        })
        .collect();
    let fully_safe_reports: Vec<bool> = reports.iter().map(check_report_safety).collect();
    let dampened_safe_reports: Vec<bool> = reports.iter().map(check_report_safety_with_dampener).collect();
    let overall_safe_reports: Vec<bool> = zip(fully_safe_reports, dampened_safe_reports).map(|(s1, s2)| s1 | s2).collect();
    Ok(overall_safe_reports.iter().map(|s| if *s { 1u32 } else { 0u32 }).sum())
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
