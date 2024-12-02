use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const OUTPUT_MESSAGE: &str = "Similarity score";

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

fn calculate_similarity_score(list_1: Vec<u32>, list_2: Vec<u32>) -> u32 {
    let mut occurrences: HashMap<u32, u32> = HashMap::new();
    list_2.iter().for_each(|&num| {
        let mut v_new = 1;
        if let Some(v_old) = occurrences.get(&num) { v_new += v_old; }
        occurrences.insert(num, v_new);
    });
    list_1
        .iter()
        .map(|&num| if let Some(occ) = occurrences.get(&num) { num * occ } else { 0 })
        .sum()
}

fn calculate_result(reader: BufReader<File>) -> Result<u32, ()> {
    let (list_1, list_2) = parse_sort_lists(reader);
    Ok(calculate_similarity_score(list_1, list_2))
}

fn main() {
    let file = File::open("input/day_01.txt").expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
