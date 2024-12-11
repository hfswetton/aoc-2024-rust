use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Map;
use std::os::raw::c_uchar;
use aoc_2024_rust::coord_grid;
use aoc_2024_rust::coord_grid::{Direction, file_lines_to_char_grid, Grid};

const OUTPUT_MESSAGE: &str = "Visited positions";
const INPUT_FILE: &str = "input/day_06.txt";

const GRID_WIDTH: usize = 130;
const GRID_HEIGHT: usize = 130;

type MapGrid = Grid<GRID_WIDTH, GRID_HEIGHT, char>;

fn follow_path(starting_coords: (usize, usize), starting_direction: Direction, grid: &MapGrid) -> HashSet<(usize, usize)> {
    let mut coords = starting_coords;
    let mut direction = starting_direction;
    let mut visited_positions = HashSet::from([starting_coords]);
    while let Ok(new_coords) = grid.move_coords(coords, direction) {
        coords = new_coords;
        visited_positions.insert(coords);
        while grid.move_coords(coords, direction).is_ok() && grid.get(grid.move_coords(coords, direction).unwrap()).unwrap() == '#' {
            direction = direction.turn_90_clockwise();
        }
    }
    visited_positions
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let grid: MapGrid = file_lines_to_char_grid(lines).expect("unable to construct grid");
    let starting_coords = grid.position('^').expect("unable to find starting position");
    let visited_positions = follow_path(starting_coords, Direction::North, &grid);
    Ok(visited_positions.len())
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
