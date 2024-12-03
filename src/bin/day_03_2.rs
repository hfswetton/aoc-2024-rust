use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use regex::Regex;

const OUTPUT_MESSAGE: &str = "Calculated total";
const INPUT_FILE: &str = "input/day_03.txt";

fn remove_disabled_sections(memory: String) -> String {
    let haystack = &(memory.clone());
    
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();

    let mut switch_positions: Vec<Option<bool>> = (0..memory.len()).map(|_| None).collect();
    switch_positions[0] = Some(true);
    do_regex.find_iter(haystack).map(|m| m.end()).for_each(|i| switch_positions[i] = Some(true));
    dont_regex.find_iter(haystack).map(|m| m.start()).for_each(|i| switch_positions[i] = Some(false));
    // -> switch_positions is Some(true) where a "do" block starts, Some(false) where a "don't" block starts and None otherwise

    let mut include_current = true;
    let mut memory_processed = String::new();
    memory.chars().enumerate().for_each(|(i, char)| {
        if let Some(new_include_current) = switch_positions[i] { include_current = new_include_current; }
        if include_current { memory_processed.push(char); }
    });

    memory_processed
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u32, ()> {
    let mut full_text = String::new();
    lines.for_each(|l| {
        let line = l.expect("invalid string");
        full_text.push_str(&line);
    });
    let enabled_memory = remove_disabled_sections(full_text);
    let instruction_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let total = instruction_regex.captures_iter(&enabled_memory).map(|c| c.extract()).map(|(_, [x, y])| {
        &str::parse::<u32>(x).expect("invalid number") * &str::parse::<u32>(y).expect("invalid number")
    }).sum();
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
