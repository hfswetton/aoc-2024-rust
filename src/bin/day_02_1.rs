use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

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

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u32, ()> {
    let reports: Vec<Vec<u32>> = lines.map(|line| line.unwrap().split(" ").filter_map(|n| n.parse::<u32>().ok()).collect()).collect();
    Ok(reports.iter().map(check_report_safety).map(|s| if s { 1u32 } else { 0u32 }).sum())
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
