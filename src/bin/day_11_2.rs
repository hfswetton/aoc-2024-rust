use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Lines};

const OUTPUT_MESSAGE: &str = "Number of stones";
const INPUT_FILE: &str = "input/day_11.txt";

const N_BLINKS: usize = 75;

type StoneType = u64;
type Stones = HashMap<StoneType, u64>;

fn parse_input(mut lines: Lines<BufReader<File>>) -> Stones {
    let line = lines.next().expect("unable to read line").expect("unable to parse line");
    line.split(" ").map(|e| (*(&str::parse::<StoneType>(e).expect("invalid number")), 1)).collect()
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

fn insert_or_add<K: Eq + Hash + Copy>(map: &mut HashMap<K, u64>, key: &K, value: &u64) {
    let old_value = map.get(key).unwrap_or(&0);
    map.insert(*key, *value + *old_value);
}

fn blink(stones: Stones) -> Stones {
    let mut new_stones = Stones::new();
    for (stone, old_count) in &stones {
        let count = *old_count;
        match split_num(*stone) {
            Ok((left, right)) => {
                insert_or_add(&mut new_stones, &left, &count);
                insert_or_add(&mut new_stones, &right, &count);
            },
            Err(_) => {
                let new_s = if *stone == 0 { 1 } else { stone * 2024 };
                insert_or_add(&mut new_stones, &new_s, &count);
            },
        };
    }
    new_stones
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u64, ()> {
    let mut stones = parse_input(lines);
    for i in 0..N_BLINKS {
        stones = blink(stones);
        println!("Blinked {} times", i+1);
    }
    let mut total = 0;
    for (_, v) in &stones {
        total += *v
    }
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
