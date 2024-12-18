use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::zip;
use itertools::{Itertools, repeat_n};
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

    fn calculate_n_sides(&self) -> usize {
        // move "ruler" over grid vertically and horizontally,
        // and detect whenever the "ruler" covers one or more sides of the region
        let mut num_sides = 0;

        let plot_boundary_n = self.plots.iter().map(|&(i, _)| i).min().unwrap();
        let plot_boundary_e = self.plots.iter().map(|&(_, j)| j).max().unwrap();
        let plot_boundary_s = self.plots.iter().map(|&(i, _)| i).max().unwrap();
        let plot_boundary_w = self.plots.iter().map(|&(_, j)| j).min().unwrap();

        let range_height = plot_boundary_n..=(plot_boundary_s+1);
        let range_width = plot_boundary_w..=(plot_boundary_e+1);
        let height = plot_boundary_s - plot_boundary_n + 1;
        let width = plot_boundary_e - plot_boundary_w + 1;

        range_height.clone().for_each(|i| {
            let is_region_above: Vec<bool> =
                if i == 0 {
                    repeat_n(false, width).collect()
                } else {
                    range_width.clone().map(|j| self.plots.contains(&(i-1, j))).collect()
                };
            let is_region_below: Vec<bool> =
                if i == MAP_HEIGHT {
                    repeat_n(false, width).collect()
                } else {
                    range_width.clone().map(|j| self.plots.contains(&(i, j))).collect()
                };
            let is_boundary_1: Vec<bool> = zip(is_region_above.clone(), is_region_below.clone()).map(|(a, b)| a && !b).collect();
            let is_boundary_2: Vec<bool> = zip(is_region_above.clone(), is_region_below.clone()).map(|(a, b)| (!a) && b).collect();
            //println!("{:?}", zip(is_boundary_1.clone(), is_boundary_2.clone()).map(|(b1, b2)| if b1 { '^' } else if b2 { 'v' } else { ' ' }).collect::<String>());
            let num_sides_in_row_1 = is_boundary_1
                .iter()
                .coalesce(|current, previous| if *current == *previous { Ok(current) } else { Err((current, previous)) })
                .filter(|&b| *b)
                .count();
            let num_sides_in_row_2 = is_boundary_2
                .iter()
                .coalesce(|current, previous| if *current == *previous { Ok(current) } else { Err((current, previous)) })
                .filter(|&b| *b)
                .count();
            num_sides += num_sides_in_row_1 + num_sides_in_row_2;
        });

        range_width.clone().for_each(|j| {
            let is_region_left: Vec<bool> =
                if j == 0 {
                    repeat_n(false, height).collect()
                } else {
                    range_height.clone().map(|i| self.plots.contains(&(i, j-1))).collect()
                };
            let is_region_right: Vec<bool> =
                if j == MAP_WIDTH {
                    repeat_n(false, height).collect()
                } else {
                    range_height.clone().map(|i| self.plots.contains(&(i, j))).collect()
                };
            let is_boundary_1: Vec<bool> = zip(is_region_left.clone(), is_region_right.clone()).map(|(a, b)| a && !b).collect();
            let is_boundary_2: Vec<bool> = zip(is_region_left.clone(), is_region_right.clone()).map(|(a, b)| (!a) && b).collect();
            let num_sides_in_col_1 = is_boundary_1
                .iter()
                .coalesce(|current, previous| if *current == *previous { Ok(current) } else { Err((current, previous)) })
                .filter(|&b| *b)
                .count();
            let num_sides_in_col_2 = is_boundary_2
                .iter()
                .coalesce(|current, previous| if *current == *previous { Ok(current) } else { Err((current, previous)) })
                .filter(|&b| *b)
                .count();
            num_sides += num_sides_in_col_1 + num_sides_in_col_2;
        });
        num_sides
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
    let total = regions.iter().map(|r| u32::try_from(r.calculate_area() * r.calculate_n_sides()).unwrap()).sum();
    Ok(total)
    // 5_498_242 too high!
    // 5_389 too low!
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
