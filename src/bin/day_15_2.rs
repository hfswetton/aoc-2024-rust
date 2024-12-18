use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use aoc_2024_rust::coord_grid::{Direction, Grid};

const OUTPUT_MESSAGE: &str = "Sum of coordinates";
const INPUT_FILE: &str = "input/day_15.txt";

const MAP_WIDTH: usize = 50*2;
const MAP_HEIGHT: usize = 50;

type Move = ((usize, usize), (usize, usize));

enum MoveError {
    MoreMovesRequired(Vec<Move>),
    Impossible,
}

type MoveRequirements = HashMap<Move, Vec<Move>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum FactoryTile {
    None,
    Wall,
    BoxLeft,
    BoxRight,
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
            let tile_types = match c {
                '.' => (FactoryTile::None, FactoryTile::None),
                '#' => (FactoryTile::Wall, FactoryTile::Wall),
                'O' => (FactoryTile::BoxLeft, FactoryTile::BoxRight),
                '@' => (FactoryTile::Robot, FactoryTile::None),
                _ => panic!("invalid tile type"),
            };
            let _ = factory_map.set((i, j*2), tile_types.0);
            let _ = factory_map.set((i, j*2 + 1), tile_types.1);
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
    if let Ok(new_map) = push(direction, &factory_map, robot_position) {
        *factory_map = new_map
    }
}

fn push(direction: &Direction, factory_map: &FactoryMap, robot_position: (usize, usize)) -> Result<FactoryMap, ()> {
    let first_move = (robot_position, factory_map.move_coords(robot_position, *direction)?);
    let mut move_requirements: MoveRequirements = HashMap::new();
    let mut moves_to_calculate: Vec<Move> = Vec::new();
    moves_to_calculate.push(first_move);
    while let Some((start_coords, end_coords)) = moves_to_calculate.pop() {
        match calculate_move(&factory_map, start_coords, end_coords) {
            Ok(_) => { move_requirements.insert((start_coords, end_coords), Vec::new()); },
            Err(MoveError::MoreMovesRequired(moves)) => {
                move_requirements.insert((start_coords, end_coords), moves.clone());
                let mut new_moves_to_calculate = moves.iter().filter(|&m| ! move_requirements.contains_key(m)).cloned().collect();
                moves_to_calculate.append(&mut new_moves_to_calculate);
            },
            Err(MoveError::Impossible) => { return Err(()); },
        }
    }
    // println!("Required moves:");
    // for (mv, req) in move_requirements.clone() {
    //     println!("- {mv:?}: {req:?}");
    // }
    let mut new_map = factory_map.clone();
    let mut completed_moves: HashSet<Move> = HashSet::new();
    while move_requirements.len() > 0 {
        let (start_coords, end_coords) = move_requirements
            .iter()
            .filter_map(|(&mv, req)| {
                if completed_moves.contains(&mv) || (req.len() > 0 && !req.iter().all(|r| completed_moves.contains(r))) {
                    None
                } else {
                    Some(mv)
                }
            })
            .next()
            .expect("some required moves not fulfillable");
        new_map.set(end_coords, new_map.get(start_coords).unwrap()).unwrap();
        new_map.set(start_coords, FactoryTile::None).unwrap();
        completed_moves.insert((start_coords, end_coords));
        move_requirements.remove(&(start_coords, end_coords));
    }
    Ok(new_map)
}

fn calculate_move(factory_map: &FactoryMap, start_coords: (usize, usize), end_coords: (usize, usize)) -> Result<(), MoveError> {
    let end_tile = factory_map.get(end_coords).expect("invalid coordinates");
    match end_tile {
        FactoryTile::None => {
            Ok(())
        },
        FactoryTile::Robot => panic!("two robots?"),
        FactoryTile::Wall => Err(MoveError::Impossible),
        FactoryTile::BoxLeft | FactoryTile::BoxRight => {
            let box_this_half_start = end_coords;
            let box_this_half_end = (
                end_coords.0 + box_this_half_start.0 - start_coords.0,
                end_coords.1 + box_this_half_start.1 - start_coords.1,
            );
            let other_half_direction = match end_tile {
                FactoryTile::BoxLeft => Direction::East,
                FactoryTile::BoxRight => Direction::West,
                _ => unreachable!(),
            };
            let box_other_half_start = factory_map.move_coords(end_coords, other_half_direction).expect("box cut in half at map edge");
            let box_other_half_end = (
                end_coords.0 + box_other_half_start.0 - start_coords.0,
                end_coords.1 + box_other_half_start.1 - start_coords.1,
            );
            if ! factory_map.contains_coords(box_this_half_end) {
                Err(MoveError::Impossible)
            } else {
                let mut moves_required = Vec::new();
                if box_this_half_end != box_other_half_start && box_other_half_end != box_this_half_start {
                    moves_required.push((box_other_half_start, box_other_half_end));
                }
                moves_required.push((box_this_half_start, box_this_half_end));
                Err(MoveError::MoreMovesRequired(moves_required))
            }
        }
    }
}

fn calculate_total_gps_value(factory_map: &FactoryMap) -> usize {
    factory_map.iter_coords().filter_map(|(i, j)| match factory_map.get((i, j)).unwrap() { FactoryTile::BoxLeft => Some(100 * i + j), _ => None }).sum()
}

fn print_map(factory_map: &FactoryMap) {
    for row in factory_map.iter_rows() {
        println!("{}", row.iter().map(|&t| match t {
            FactoryTile::None => '.',
            FactoryTile::Robot => '@',
            FactoryTile::Wall => '#',
            FactoryTile::BoxLeft => '[',
            FactoryTile::BoxRight => ']',
        }).collect::<String>());
    }
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let (mut factory_map, moves) = parse_input(lines);
    print_map(&factory_map);
    moves.iter().for_each(|m| move_robot(m, &mut factory_map));
    print_map(&factory_map);
    Ok(calculate_total_gps_value(&factory_map))
    // 1442998 is too high!
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
