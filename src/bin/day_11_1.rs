use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Number of stones";
const INPUT_FILE: &str = "input/day_11.txt";

const N_BLINKS: usize = 25;

type StoneType = u64;
type Stones = Vec<StoneType>;

fn parse_input(mut lines: Lines<BufReader<File>>) -> Stones {
    let line = lines.next().expect("unable to read line").expect("unable to parse line");
    line.split(" ").map(|e| *(&str::parse::<StoneType>(e).expect("invalid number"))).collect()
}

fn split_num(num: StoneType) -> Result<(StoneType, StoneType), ()> {
    let log = match num.checked_ilog10() {
        Some(v) => v,
        None => { return Err(()); },
    };
    if log % 2 == 0 { return Err(()); }
    let divisor = (10 as StoneType).pow(log / 2 + 1);
    Ok((num / divisor, num % divisor))
}

fn blink(stones: &mut Stones) {
    let mut i = 0;
    while i < stones.len() {
        match split_num(stones[i]) {
            Ok((left, right)) => {
                stones[i] = right;
                stones.insert(i, left);
                i += 1;
            },
            Err(_) => {
                stones[i] = if stones[i] == 0 { 1 } else { stones[i] * 2024 };
            },
        }
        i += 1;
    }
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let mut stones = parse_input(lines);
    for _ in 0..N_BLINKS {
        blink(&mut stones);
    }
    Ok(stones.len())
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
