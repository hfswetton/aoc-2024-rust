use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use aoc_2024_rust::coord_grid::{Direction, file_lines_to_char_grid, Grid};

const OUTPUT_MESSAGE: &str = "Visited positions";
const INPUT_FILE: &str = "input/day_06.txt";

const GRID_WIDTH: usize = 130;
const GRID_HEIGHT: usize = 130;

type MapGrid = Grid<GRID_WIDTH, GRID_HEIGHT, char>;

fn is_infinite_loop(starting_coords: (usize, usize), starting_direction: Direction, grid: &MapGrid) -> bool {
    let mut coords = starting_coords;
    let mut direction = starting_direction;
    let mut visited_positions = HashSet::from([(starting_coords, starting_direction)]);
    while let Ok(new_coords) = grid.move_coords(coords, direction) {
        coords = new_coords;
        if visited_positions.contains(&(coords, direction)) { return true; }
        visited_positions.insert((coords, direction));
        while grid.move_coords(coords, direction).is_ok() && grid.get(grid.move_coords(coords, direction).unwrap()).unwrap() == '#' {
            direction = direction.turn_90_clockwise();
        }
    }
    false
}

fn find_obstacle_positions(starting_coords: (usize, usize), starting_direction: Direction, grid: &MapGrid) -> Vec<(usize, usize)> {
    let mut valid_positions = Vec::new();
    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            if (i, j) == starting_coords || grid.get((i, j)).unwrap() == '#' { continue; }
            let mut new_grid = grid.clone();
            let _ = new_grid.set((i, j), '#');
            if is_infinite_loop(starting_coords, starting_direction, &new_grid) { valid_positions.push((i, j)) }
        }
        println!("Row {i} tested.")
    }
    valid_positions
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let grid: MapGrid = file_lines_to_char_grid(lines).expect("unable to construct grid");
    let starting_coords = grid.position('^').expect("unable to find starting position");
    let valid_positions = find_obstacle_positions(starting_coords, Direction::North, &grid);
    Ok(valid_positions.len())
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
