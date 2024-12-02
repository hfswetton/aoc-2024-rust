use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Total distance";
const INPUT_FILE: &str = "input/day_XX.txt";

fn calculate_result<T>(lines: Lines<BufReader<File>>) -> Result<T, ()> {
    Ok(())
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
