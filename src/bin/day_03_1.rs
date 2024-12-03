use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use regex::Regex;

const OUTPUT_MESSAGE: &str = "Calculated total";
const INPUT_FILE: &str = "input/day_03.txt";

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u32, ()> {
    let mut text = String::from("");
    for l in lines {
        let line = l.unwrap();
        text.push_str(" ");  // avoid line break removal causing unwanted matches
        text.push_str(&line);
    }
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let total = re
        .captures_iter(&text)
        .map(|c| c.extract())
        .map(|(_, [x, y])| &str::parse::<u32>(x).expect("invalid number") * &str::parse::<u32>(y).expect("invalid number"))
        .sum();
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
