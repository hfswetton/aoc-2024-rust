use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use aoc_2024_rust::coord_grid::{Direction, file_lines_to_char_grid, Grid};

const OUTPUT_MESSAGE: &str = "Total cost";
const INPUT_FILE: &str = "input/day_12.txt";

const MAP_WIDTH: usize = 140;
const MAP_HEIGHT: usize = 140;
type GardenGrid = Grid<MAP_WIDTH, MAP_HEIGHT, char>;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Region {
    plots: Vec<(usize, usize)>
}

impl Region {
    fn create_from_plot(plot: (usize, usize), grid: &GardenGrid) -> Self {
        let char = grid.get(plot);
        let directions = Direction::main_directions();
        let mut plots = HashSet::from([plot]);
        let mut old_plots = HashSet::new();
        while plots.len() != old_plots.len() {
            old_plots = plots.clone();
            for new_plot in &old_plots {
                for direction in directions {
                    if let Ok(coords) = grid.move_coords(*new_plot, direction) {
                        if grid.get(coords) == char { plots.insert(coords); }
                    }
                }
            }
        }
        Self { plots: plots.iter().cloned().collect() }
    }

    fn calculate_area(&self) -> usize {
        self.plots.len()
    }

    fn calculate_perimeter(&self, grid: &GardenGrid) -> usize {
        let directions = Direction::main_directions();
        let potential_perimeter_coords: Vec<(isize, isize)> =
            self
                .plots
                .iter()
                .flat_map(|plot| {
                    directions.iter().map(|direction| grid.force_move_coords(*plot, *direction))
                }).collect();
        // note: potential_perimeter_coords will contain the same plot multiple times,
        // e.g. around corners. This is intended, as these should be counted twice
        // (once from each side touching the current region),
        // and this is easier than trying to implement squares "between" each plot.
        potential_perimeter_coords.iter().filter(|&plot|
            ! grid.contains_coords_signed(*plot)
                || ! self.plots.contains(&(plot.0.try_into().unwrap(), plot.1.try_into().unwrap()))
        ).count()
    }
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<u32, ()> {
    let grid: GardenGrid = file_lines_to_char_grid(lines).expect("unable to read input");
    let mut covered_plots = [[false; MAP_WIDTH]; MAP_HEIGHT];
    let mut regions: Vec<Region> = Vec::new();
    for plot in grid.iter_coords() {
        if covered_plots[plot.0][plot.1] { continue; }
        let new_region = Region::create_from_plot(plot, &grid);
        new_region.plots.iter().for_each(|plot| covered_plots[plot.0][plot.1] = true);
        regions.push(new_region);
    }
    let total = regions.iter().map(|r| u32::try_from(r.calculate_area() * r.calculate_perimeter(&grid)).unwrap()).sum();
    Ok(total)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
