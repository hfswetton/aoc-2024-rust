use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

const OUTPUT_MESSAGE: &str = "Total distance";

fn parse_sort_lists(reader: BufReader<File>) -> (Vec<u32>, Vec<u32>) {
    let mut list_1: Vec<u32> = Vec::new();
    let mut list_2: Vec<u32> = Vec::new();
    reader.lines().for_each(|l| {
        let nums: Vec<u32> = l
            .expect("unable to get line")
            .split(" ")
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();
        if nums.len() != 2 { panic!("incorrect number of numbers in line") }
        list_1.push(nums[0]);
        list_2.push(nums[1]);
    });
    list_1.sort();
    list_2.sort();
    (list_1, list_2)
}

fn calculate_result(reader: BufReader<File>) -> Result<u32, ()> {
    let sorted_values = parse_sort_lists(reader);
    Ok(zip(sorted_values.0, sorted_values.1).map(|(x, y)| x.abs_diff(y)).sum())
}

fn main() {
    let file = File::open("input/day_01.txt").expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
