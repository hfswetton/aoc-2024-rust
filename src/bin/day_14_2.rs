use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use itertools::Itertools;
use regex::Regex;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use aoc_2024_rust::coord_grid::Grid;

const OUTPUT_MESSAGE: &str = "Total";
const INPUT_FILE: &str = "input/day_14.txt";

const AREA_WIDTH: usize = 101;
const AREA_HEIGHT: usize = 103;
const N_SECONDS: usize = 100;

type RobotGrid = Grid<AREA_HEIGHT, AREA_WIDTH, bool>;

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq, EnumIter)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

struct Robot {
    location: (usize, usize),
    velocity: (isize, isize),
}

impl Robot {
    fn next_1d_coord(p: usize, v: isize, p_max: usize) -> usize {
        if v == 0 {
            p
        } else if v < 0 {
            let v_abs = usize::try_from(-v).unwrap();
            if v_abs > p {
                let diff = v_abs - p;
                p_max - diff
            } else {
                p - v_abs
            }
        } else {
            let sum = usize::try_from(v).unwrap() + p;
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
        let p_x = str::parse::<usize>(mat.get(1).expect("item not found in line").as_str()).unwrap();
        let p_y = str::parse::<usize>(mat.get(2).expect("item not found in line").as_str()).unwrap();
        let v_x = str::parse::<isize>(mat.get(3).expect("item not found in line").as_str()).unwrap();
        let v_y = str::parse::<isize>(mat.get(4).expect("item not found in line").as_str()).unwrap();
        Robot {
            location: (p_x, p_y),
            velocity: (v_x, v_y),
        }
    }).collect()
}

fn print_grid(grid: &RobotGrid) {
    grid.iter_rows().for_each(|r| {
        println!("{}", r.iter().map(|&v| if v { "█" } else { " " }).collect::<String>());
    });
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let mut robots = parse_input(lines);
    let mut grid: RobotGrid = Grid::create();
    let mut i = 0;
    loop {
        i += 1;
        grid.set_all(false);
        robots.iter_mut().enumerate().for_each(|(j, r)| {
            r.tick();
            grid.set((r.location.0, r.location.1), true).unwrap();
        });
        if grid.iter_rows().any(|r| r.iter().filter(|&v| *v).count() > 15) {
            print_grid(&grid);
        }
        println!("{i} seconds passed.");
        if i > 10000 { panic!(); }
    }
    // Solution found at 7892 seconds
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
