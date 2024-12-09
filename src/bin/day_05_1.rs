use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Sum of middle page numbers";
const INPUT_FILE: &str = "input/day_05.txt";

const NUM_ORDERING_RULES: usize = 1176;
const NUM_UPDATES: usize = 210;

#[derive(Copy, Clone)]
struct OrderingRule (usize, usize);

impl OrderingRule {
    fn is_violated(&self, update: &Vec<usize>) -> bool {
        if let Some(pos_0) = update.iter().position(|&n| n == self.0) {
            if let Some(pos_1) = update.iter().position(|&n| n == self.1) {
                if pos_0 > pos_1 { return true; }
            }
        }
        false
    }
}

fn parse_ordering_rules(lines: &Vec<&str>) -> Vec<OrderingRule>{
    let ordering_rules = lines.iter().take(NUM_ORDERING_RULES).map(|&line| {
        let nums = line
            .split("|")
            .map(|v| *(&str::parse::<usize>(v).unwrap()))
            .collect::<Vec<usize>>();
        OrderingRule (nums[0], nums[1])  // note: not checking whether nums has >2 elements
    }).collect();
    ordering_rules
}

fn parse_updates(lines: &Vec<&str>) -> Vec<Vec<usize>> {
    lines.iter().skip(NUM_ORDERING_RULES + 1).take(NUM_UPDATES).map(|&line| {
        line.split(",").map(|n| *(&str::parse::<usize>(n).expect("invalid number"))).collect()
    }).collect()
}

fn middle_element<T: Clone>(vec: &Vec<T>) -> T {
    vec[vec.len() / 2].clone()
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let lines_vec: Vec<String> = lines.map(|l| l.expect("unable to read line")).collect();
    let lines_str_vec: Vec<&str> = lines_vec.iter().map(|s| &(s[..])).collect();
    let ordering_rules = parse_ordering_rules(&lines_str_vec);
    let updates = parse_updates(&lines_str_vec);
    let fulfilled_middle_nums = updates.iter().filter_map(|update| {
        if ordering_rules.iter().map(|&r| r.is_violated(update)).any(|v| v) { None }
        else { Some(middle_element(update)) }
    }).collect::<Vec<usize>>();
    Ok(fulfilled_middle_nums.iter().sum())
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
