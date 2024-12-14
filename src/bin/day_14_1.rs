use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use itertools::Itertools;
use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_14.txt";

const AREA_WIDTH: u32 = 101;
const AREA_HEIGHT: u32 = 103;
const N_SECONDS: u32 = 100;

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

struct Robot {
    location: (u32, u32),
    velocity: (i32, i32),
}

impl Robot {
    fn next_1d_coord(p: u32, v: i32, p_max: u32) -> u32 {
        if v == 0 {
            p
        } else if v < 0 {
            let v_abs = u32::try_from(-v).unwrap();
            if v_abs > p {
                let diff = v_abs - p;
                p_max - diff
            } else {
                p - v_abs
            }
        } else {
            let sum = u32::try_from(v).unwrap() + p;
            if sum >= p_max {
                sum - p_max
            } else {
                sum
            }
        }
    }

    fn tick(&mut self) {
        self.location = (
            Self::next_1d_coord(self.location.0, self.velocity.0, AREA_WIDTH),
            Self::next_1d_coord(self.location.1, self.velocity.1, AREA_HEIGHT),
        )
    }

    fn get_quadrant(&self) -> Option<Quadrant> {
        if self.location.0 == AREA_WIDTH / 2 || self.location.1 == AREA_HEIGHT / 2 {
            None
        } else {
            let left = self.location.0 < AREA_WIDTH / 2;
            let top = self.location.1 < AREA_HEIGHT / 2;
            Some(
                match (top, left) {
                    (true, true) => Quadrant::TopLeft,
                    (true, false) => Quadrant::TopRight,
                    (false, true) => Quadrant::BottomLeft,
                    (false, false) => Quadrant::BottomRight,
                }
            )
        }
    }
}

fn parse_input(lines: Lines<BufReader<File>>) -> Vec<Robot> {
    let robot_regex  = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    lines.map(|l| {
        let line = l.expect("invalid line");
        let mat = robot_regex.captures(&line).expect("unable to parse line");
        let p_x = str::parse::<u32>(mat.get(1).expect("item not found in line").as_str()).unwrap();
        let p_y = str::parse::<u32>(mat.get(2).expect("item not found in line").as_str()).unwrap();
        let v_x = str::parse::<i32>(mat.get(3).expect("item not found in line").as_str()).unwrap();
        let v_y = str::parse::<i32>(mat.get(4).expect("item not found in line").as_str()).unwrap();
        Robot {
            location: (p_x, p_y),
            velocity: (v_x, v_y),
        }
    }).collect()
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u32, ()> {
    let mut robots = parse_input(lines);
    for i in 1..=N_SECONDS {
        robots.iter_mut().for_each(|r| r.tick());
        println!("{i} seconds passed.")
    }
    let robots_per_quadrant = robots.iter().filter_map(|r| r.get_quadrant()).counts();
    let safety_factor = Quadrant::iter().map(|q| u32::try_from(*robots_per_quadrant.get(&q).unwrap_or(&0)).unwrap()).product();
    Ok(safety_factor)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
