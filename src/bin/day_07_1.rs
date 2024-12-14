use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::repeat_n;

const OUTPUT_MESSAGE: &str = "Total calibration result";
const INPUT_FILE: &str = "input/day_07.txt";

#[derive(Copy, Clone, Debug)]
enum Operator {
    Add,
    Multiply,
}

fn parse_input(lines: Lines<BufReader<File>>) -> Vec<(u64, Vec<u64>)> {
    lines.map(|l| {
        let line = l.unwrap();
        let (test_value_str, numbers_str) = line.split_once(": ").unwrap();
        let test_value = str::parse(test_value_str).expect("unable to parse test value");
        let numbers = numbers_str.split(" ").map(|n| str::parse(n).expect("unable to parse number")).collect();
        (test_value, numbers)
    }).collect()
}

fn can_be_calculated(data_item: &&(u64, Vec<u64>)) -> bool {
    let (test_value, numbers) = data_item;
    let operators = repeat_n(Operator::Add, numbers.len() - 1).collect();
    recurse(test_value, numbers, &operators, 0)
}

fn recurse(test_value: &u64, numbers: &Vec<u64>, operators: &Vec<Operator>, test_pos: usize) -> bool {
    let total = calculate(numbers, operators).unwrap();
    if total == *test_value {
        true
    } else if test_pos == operators.len() {
        false
    } else {
        let mut new_operators = operators.clone();
        new_operators[test_pos] = Operator::Multiply;
        recurse(test_value, numbers, &operators, test_pos + 1)
            || recurse(test_value, numbers, &new_operators, test_pos + 1)
    }
}

fn calculate(numbers: &Vec<u64>, operators: &Vec<Operator>) -> Result<u64, String> {
    if numbers.len() != operators.len() + 1 {
        return Err(format!("incorrect vector lengths: {} (numbers) & {} (operators)", numbers.len(), operators.len()));
    }
    let mut total = numbers[0];
    for i in 0..operators.len() {
        match operators[i] {
            Operator::Add => { total += numbers[i + 1] },
            Operator::Multiply => { total *= numbers[i + 1] },
        }
    }
    Ok(total)
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u64, ()> {
    let data = parse_input(lines);
    let total = data.iter().filter(can_be_calculated).map(|(test_value, _)| *test_value).sum();
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
