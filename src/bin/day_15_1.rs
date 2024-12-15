use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use aoc_2024_rust::coord_grid::{Direction, Grid};

const OUTPUT_MESSAGE: &str = "Sum of coordinates";
const INPUT_FILE: &str = "input/day_15.txt";

const MAP_WIDTH: usize = 50;
const MAP_HEIGHT: usize = 50;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FactoryTile {
    None,
    Wall,
    Box,
    Robot,
}

impl Default for FactoryTile {
    fn default() -> Self {
        Self::None
    }
}

type FactoryMap = Grid<MAP_WIDTH, MAP_HEIGHT, FactoryTile>;

fn parse_input(lines: Lines<BufReader<File>>) -> (FactoryMap, Vec<Direction>) {
    let mut factory_map = FactoryMap::create();
    let lines_read = lines.map(|l| l.expect("unable to read line")).collect::<Vec<String>>();
    lines_read.iter().take(MAP_HEIGHT).enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            let tile_type = match c {
                '.' => FactoryTile::None,
                '#' => FactoryTile::Wall,
                'O' => FactoryTile::Box,
                '@' => FactoryTile::Robot,
                _ => panic!("invalid tile type"),
            };
            let _ = factory_map.set((i, j), tile_type);
        })
    });

    let moves = lines_read.iter().skip(MAP_HEIGHT + 1).map(|l| l.chars().map(|c| match c {
        '^' => Direction::North,
        '>' => Direction::East,
        'v' => Direction::South,
        '<' => Direction::West,
        _ => panic!("invalid direction"),
    })).flatten().collect();

    (factory_map, moves)
}

fn move_robot(direction: &Direction, factory_map: &mut FactoryMap) {
    let robot_position = factory_map.position(FactoryTile::Robot).expect("unable to find robot");
    if let Ok(new_map) = recurse(direction, factory_map, robot_position) {
        *factory_map = new_map
    }
}

fn recurse(direction: &Direction, factory_map: &FactoryMap, position_to_check: (usize, usize)) -> Result<FactoryMap, ()> {
    if let Ok(new_coords) = factory_map.move_coords(position_to_check, *direction) {
        match factory_map.get(new_coords).unwrap() {
            FactoryTile::None => {
                let mut new_map = factory_map.clone();
                let old_value = new_map.get(position_to_check).unwrap();
                let _ = new_map.set(new_coords, old_value);
                let _ = new_map.set(position_to_check, FactoryTile::None);
                Ok(new_map)
            }
            FactoryTile::Wall => Err(()),
            FactoryTile::Robot => panic!("two robots?"),
            FactoryTile::Box => {
                if let Ok(mut new_map) = recurse(direction, factory_map, new_coords) {
                    let old_value = new_map.get(position_to_check).unwrap();
                    let _ = new_map.set(new_coords, old_value);
                    let _ = new_map.set(position_to_check, FactoryTile::None);
                    Ok(new_map)
                } else { Err(()) }
            }
        }
    } else { Err(()) }
}

fn calculate_total_gps_value(factory_map: &FactoryMap) -> usize {
    factory_map.iter_coords().filter_map(|(i, j)| match factory_map.get((i, j)).unwrap() { FactoryTile::Box => Some(100 * i + j), _ => None }).sum()
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let (mut factory_map, moves) = parse_input(lines);
    moves.iter().for_each(|m| move_robot(m, &mut factory_map));
    Ok(calculate_total_gps_value(&factory_map))
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
