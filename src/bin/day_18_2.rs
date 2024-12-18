use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use itertools::Itertools;
use aoc_2024_rust::coord_grid::{Direction, Grid};

const OUTPUT_MESSAGE: &str = "First byte causing failure";
const INPUT_FILE: &str = "input/day_18.txt";

const GRID_WIDTH: usize = 71;
const GRID_HEIGHT: usize = 71;

type PathLength = u128;

type MemorySpace = Grid<GRID_WIDTH, GRID_HEIGHT, bool>;  // true if accessible, false if corrupted
type ShortestPaths = Grid<GRID_WIDTH, GRID_HEIGHT, PathLength>;
type LastCoords = Grid<GRID_WIDTH, GRID_HEIGHT, Option<(usize, usize)>>;

const START_COORDS: (usize, usize) = (0, 0);
const END_COORDS: (usize, usize) = (70, 70);

fn dijstra_path_exists(grid: &MemorySpace) -> bool {
    let mut shortest_paths = ShortestPaths::create();
    shortest_paths.set_all(PathLength::MAX);
    shortest_paths.set(START_COORDS, 0).unwrap();
    let mut last_coords = LastCoords::create();
    let mut unchecked_coords: HashSet<(usize, usize)> = grid.iter_coords().filter(|&c| grid.get(c).unwrap()).collect();

    while unchecked_coords.len() > 0 {
        let closest = unchecked_coords.iter().sorted_by_key(|&c| shortest_paths.get(*c).unwrap()).next().unwrap().clone();
        if closest == END_COORDS { break; }
        else if shortest_paths.get(closest).unwrap() == PathLength::MAX { return false; }
        unchecked_coords.remove(&closest);
        Direction::main_directions().iter().for_each(|d| {
            if let Ok(neighbour) = grid.move_coords(closest, *d) {
                let new_dist = shortest_paths.get(closest).unwrap() + 1;
                if new_dist < shortest_paths.get(neighbour).unwrap() {
                    shortest_paths.set(neighbour, new_dist).unwrap();
                    last_coords.set(neighbour, Some(closest)).unwrap();
                }
            }
        });
    }

    true
}

fn test_fails_at_n(n: usize, byte_coords: &Vec<(usize, usize)>, cache: &mut HashMap<usize, bool>) -> bool {
    println!("Testing with {n} bytes...");
    if let Some(res) = cache.get(&n) { return *res; }
    let mut mem = MemorySpace::create();
    mem.set_all(true);
    byte_coords[..=n].iter().for_each(|&c| mem.set(c, false).unwrap());
    let res = ! dijstra_path_exists(&mem);
    cache.insert(n, res);
    res
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<String, ()> {
    let byte_coords: Vec<(usize, usize)> = lines.map(|l| {
        let coords = l.expect("unable to read line").split(",").map(|n| str::parse::<usize>(n).expect("invalid number")).collect::<Vec<usize>>();
        if coords.len() != 2 { panic!("invalid coordinates"); }
        (coords[0], coords[1])
    }).collect();
    println!("Byte positions: {byte_coords:?} ({} total)", byte_coords.len());
    let mut n_min = 1024;
    let mut n_max = byte_coords.len();
    let mut n = (n_min + n_max) / 2;
    let mut cache: HashMap<usize, bool> = HashMap::new();
    let n_fail = loop {
        if test_fails_at_n(n, &byte_coords, &mut cache) {
            if ! test_fails_at_n(n - 1, &byte_coords, &mut cache) {
                break n;
            } else {
                n_max = n;
                n = (n_min + n) / 2;
            }
        } else {
            n_min = n;
            n = (n_max + n) / 2;
        }
    };
    let coords_fail = byte_coords[n_fail];
    Ok(format!("{},{}", coords_fail.0, coords_fail.1))
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
