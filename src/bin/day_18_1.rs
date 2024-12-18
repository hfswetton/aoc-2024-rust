use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use itertools::Itertools;
use aoc_2024_rust::coord_grid::{Direction, Grid};

const OUTPUT_MESSAGE: &str = "Shortest path length";
const INPUT_FILE: &str = "input/day_18.txt";

const GRID_WIDTH: usize = 71;
const GRID_HEIGHT: usize = 71;

type PathLength = u32;

type MemorySpace = Grid<GRID_WIDTH, GRID_HEIGHT, bool>;  // true if accessible, false if corrupted
type ShortestPaths = Grid<GRID_WIDTH, GRID_HEIGHT, PathLength>;
type LastCoords = Grid<GRID_WIDTH, GRID_HEIGHT, Option<(usize, usize)>>;

const START_COORDS: (usize, usize) = (0, 0);
const END_COORDS: (usize, usize) = (70, 70);

fn dijkstra_len(grid: &MemorySpace) -> PathLength {
    let mut shortest_paths = ShortestPaths::create();
    shortest_paths.set_all(PathLength::MAX);
    shortest_paths.set(START_COORDS, 0).unwrap();
    let mut last_coords = LastCoords::create();
    let mut unchecked_coords: HashSet<(usize, usize)> = grid.iter_coords().filter(|&c| grid.get(c).unwrap()).collect();

    while unchecked_coords.len() > 0 {
        let closest = unchecked_coords.iter().sorted_by_key(|&c| shortest_paths.get(*c).unwrap()).next().unwrap().clone();
        if closest == END_COORDS { break; }
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

    shortest_paths.get(END_COORDS).expect("unable to get path length at end")
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<PathLength, ()> {
    let mut mem = MemorySpace::create();
    mem.set_all(true);
    lines.take(1024).for_each(|l| {
        let coords = l.expect("unable to read line").split(",").map(|n| str::parse::<usize>(n).expect("invalid number")).collect::<Vec<usize>>();
        if coords.len() != 2 { panic!("invalid coordinates"); }
        let _ = mem.set((coords[0], coords[1]), false);
    });
    let shortest_path_length = dijkstra_len(&mem);
    Ok(shortest_path_length)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
