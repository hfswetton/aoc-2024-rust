use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use strum::IntoEnumIterator;
use aoc_2024_rust::coord_grid::{Direction, Grid};

const OUTPUT_MESSAGE: &str = "Lowest possible score";
const INPUT_FILE: &str = "input/day_16.txt";

type Score = u64;
const MOVE_SCORE: Score = 1;
const TURN_SCORE: Score = 1000;

#[derive(Copy, Clone, PartialEq, Debug)]
enum MazeTile {
    Start,
    End,
    Wall,
    None,
}

impl Default for MazeTile {
    fn default() -> Self {
        Self::None
    }
}

const MAZE_WIDTH: usize = 141;
const MAZE_HEIGHT: usize = 141;
type Maze = Grid<MAZE_WIDTH, MAZE_HEIGHT, MazeTile>;
type MazeScores = HashMap<((usize, usize), Direction), Score>;

fn parse_input(lines: Lines<BufReader<File>>) -> Maze {
    let mut maze = Maze::create();
    lines.enumerate().for_each(|(i, l)| {
        l.expect("unable to read line").chars().enumerate().for_each(|(j, c)| {
            let tile = match c {
                'S' => MazeTile::Start,
                'E' => MazeTile::End,
                '#' => MazeTile::Wall,
                '.' => MazeTile::None,
                _ => panic!("invalid character")
            };
            let _ = maze.set((i, j), tile);
        });
    });
    maze
}

fn find_lowest_score(maze: &Maze) -> Score {
    // note: this version takes ~2min due to slow recursive solution!
    let start_position: (usize, usize) = maze.position(MazeTile::Start).expect("unable to find start");
    let end_position: (usize, usize) = maze.position(MazeTile::End).expect("unable to find end");
    let mut scores = MazeScores::new();
    recurse(start_position, Direction::East, 0, &mut scores, maze);
    Direction::iter().filter_map(|d| scores.get(&(end_position, d))).min().expect("unable to calculate lowest score").clone()
}

fn recurse(position: (usize, usize), direction: Direction, current_score: Score, scores: &mut MazeScores, maze: &Maze) {
    if let Some(&previous_score) = scores.get(&(position, direction)) {
        if previous_score <= current_score {
            return;
        }
    }
    scores.insert((position, direction), current_score);
    match maze.get(position).expect("invalid coordinates") {
        MazeTile::Wall | MazeTile::End => (),
        MazeTile::Start | MazeTile::None => {
            [(0, 0), (90, TURN_SCORE), (180, 2 * TURN_SCORE), (270, TURN_SCORE)].iter().for_each(|(ang, turn_score)| {
                let new_direction = direction.turn_anticlockwise(*ang).unwrap();
                if let Ok(new_position) = maze.move_coords(position, new_direction) {
                    recurse(
                        new_position,
                        new_direction,
                        current_score + turn_score + MOVE_SCORE,
                        scores,
                        maze
                    );
                }
            });
        }
    }
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<Score, ()> {
    let maze = parse_input(lines);
    let lowest_score = find_lowest_score(&maze);
    Ok(lowest_score)
    // should be 111480
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
