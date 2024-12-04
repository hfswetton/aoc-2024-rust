use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use regex::Regex;

const OUTPUT_MESSAGE: &str = "Calculated total";
const INPUT_FILE: &str = "input/day_03.txt";

fn remove_disabled_sections(memory: String) -> String {
    let mut include_current = true;
    memory.chars().enumerate().filter_map(|(i, char)| {
        if (i < memory.len() - 4) && (&memory[i..i+4] == "do()") { include_current = true; }
        else if (i < memory.len() - 7) && (&memory[i..i+7] == "don't()") { include_current = false; }
        if include_current { Some(char) } else { None }
    }).collect()
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u32, ()> {
    let full_text: String = lines.map(|l| l.unwrap()).collect();
    let enabled_memory = remove_disabled_sections(full_text);
    let instruction_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let total = instruction_regex
        .captures_iter(&enabled_memory)
        .map(|c| {
            let (_, [x, y]) = c.extract();
            &str::parse::<u32>(x).expect("invalid number")
                * &str::parse::<u32>(y).expect("invalid number")
        })
        .sum();
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
