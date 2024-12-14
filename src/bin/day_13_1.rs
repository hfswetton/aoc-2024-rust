use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use itertools::{Chunk, Itertools};
use regex::Regex;

const OUTPUT_MESSAGE: &str = "Minimum nr. of tokens";
const INPUT_FILE: &str = "input/day_13.txt";

const MAX_PRESSES_PER_BUTTON: usize = 100;
const BUTTON_A_COST: u64 = 3;
const BUTTON_B_COST: u64 = 1;

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: (usize, usize),
    button_b: (usize, usize),
    prize: (usize, usize),
}

impl ClawMachine {
    fn find_all_solutions(&self) -> Vec<(usize, usize)> {
        let mut solutions = Vec::new();
        for a in 0..MAX_PRESSES_PER_BUTTON {
            for b in 0..MAX_PRESSES_PER_BUTTON {
                let position = (self.button_a.0 * a + self.button_b.0 * b, self.button_a.1 * a + self.button_b.1 * b);
                if position == self.prize {
                    solutions.push((a, b));
                } else if position.0 >= self.prize.0 || position.1 >= self.prize.1 {
                    break
                }
            }
        }
        solutions
    }

    fn get_solution_cost(&self, solution: &(usize, usize)) -> u64 {
        u64::try_from(solution.0).unwrap() * BUTTON_A_COST
            + u64::try_from(solution.1).unwrap() * BUTTON_B_COST
    }

    fn find_minimum_cost(&self) -> Option<u64> {
        let solutions = self.find_all_solutions();
        let costs = solutions.iter().map(|s| self.get_solution_cost(s)).collect::<Vec<u64>>();
        costs.iter().cloned().min()
    }
}

fn parse_single_claw_machine(mut input: Chunk<Lines<BufReader<File>>>) -> ClawMachine {
    let button_regex = Regex::new(r"Button \w: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let button_a_inp = input.next().unwrap().unwrap();
    let button_b_inp = input.next().unwrap().unwrap();
    let prize_inp = input.next().unwrap().unwrap();
    let button_a_cap = button_regex.captures(&button_a_inp).unwrap();
    let button_b_cap = button_regex.captures(&button_b_inp).unwrap();
    let prize_cap = prize_regex.captures(&prize_inp).unwrap();
    ClawMachine {
        button_a: (
            str::parse(button_a_cap.get(1).unwrap().as_str()).unwrap(),
            str::parse(button_a_cap.get(2).unwrap().as_str()).unwrap()
        ),
        button_b: (
            str::parse(button_b_cap.get(1).unwrap().as_str()).unwrap(),
            str::parse(button_b_cap.get(2).unwrap().as_str()).unwrap()
        ),
        prize: (
            str::parse(prize_cap.get(1).unwrap().as_str()).unwrap(),
            str::parse(prize_cap.get(2).unwrap().as_str()).unwrap()
        ),
    }
}

fn parse_claw_machines(lines: Lines<BufReader<File>>) -> Vec<ClawMachine> {
    lines.chunks(4).into_iter().map(parse_single_claw_machine).collect()
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u64, ()> {
    let claw_machines = parse_claw_machines(lines);
    let total = claw_machines.iter().filter_map(|cm| cm.find_minimum_cost()).sum();
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
