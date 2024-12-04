use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use strum_macros::EnumIter;

const OUTPUT_MESSAGE: &str = "Number of occurrences";
const INPUT_FILE: &str = "input/day_04.txt";

const GRID_WIDTH: usize = 141;
const GRID_HEIGHT: usize = 140;

#[derive(Debug)]
struct Grid {
    _grid: [[char; GRID_WIDTH]; GRID_HEIGHT],
}

impl Grid {
    fn create() -> Self {
        Self { _grid: [['.'; GRID_WIDTH]; GRID_HEIGHT] }
    }

    fn get_raw(&self, x: usize, y: usize) -> char {
        self._grid[x][y]
    }

    fn get(&self, coords: Coords) -> char {
        self.get_raw(coords.0, coords.1)
    }

    fn set_raw(&mut self, x: usize, y: usize, c: char) -> () {
        self._grid[x][y] = c;
    }

    fn iter(&self) -> impl Iterator<Item=&[char; GRID_WIDTH]> {
        self._grid.iter()
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter)]
enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Coords (usize, usize);

impl Coords {
    fn moved(&self, direction: Direction) -> Result<Self, ()> {
        match direction {
            Direction::North => if self.0 > 0               { Ok(Self(self.0 - 1, self.1)) } else { Err(()) },
            Direction::East  => if self.1 < GRID_WIDTH - 1  { Ok(Self(self.0, self.1 + 1)) } else { Err(()) },
            Direction::South => if self.0 < GRID_HEIGHT - 1 { Ok(Self(self.0 + 1, self.1)) } else { Err(()) },
            Direction::West  => if self.1 > 0               { Ok(Self(self.0, self.1 - 1)) } else { Err(()) },
            Direction::Northeast => self.moved(Direction::North)?.moved(Direction::East),
            Direction::Southeast => self.moved(Direction::South)?.moved(Direction::East),
            Direction::Southwest => self.moved(Direction::South)?.moved(Direction::West),
            Direction::Northwest => self.moved(Direction::North)?.moved(Direction::West),
        }
    }
}

fn find_a(grid: &Grid) -> Vec<Coords> {
    grid.iter().enumerate().flat_map(|(i, row)| {
        row.iter().enumerate().filter_map(move |(j, c)| if *c == 'A' { Some(Coords(i, j)) } else { None } )
    }).collect()
}

fn is_valid_x_mas(a_position: &Coords, grid: &Grid) -> bool {
    if let (Ok(ne), Ok(nw), Ok(se), Ok(sw)) = (
        a_position.moved(Direction::Northeast),
        a_position.moved(Direction::Northwest),
        a_position.moved(Direction::Southeast),
        a_position.moved(Direction::Southwest),
    ) {
        let c_ne = grid.get(ne);
        let c_nw = grid.get(nw);
        let c_se = grid.get(se);
        let c_sw = grid.get(sw);

        if
            ((c_ne, c_sw) == ('M', 'S') || (c_ne, c_sw) == ('S', 'M'))
            && ((c_nw, c_se) == ('M', 'S') || (c_nw, c_se) == ('S', 'M'))
        {
            return true;
        }
    }
    false
}

fn count_valid_x_mas(a_positions: &Vec<Coords>, grid: &Grid) -> usize {
    a_positions.iter().map(|a_pos| if is_valid_x_mas(a_pos, grid) { 1 } else { 0 }).sum()
}

fn calculate_result(lines: Lines<BufReader<File>>) -> Result<usize, ()> {
    let mut grid = Grid::create();
    lines.enumerate().for_each(|(i, l)| {
        let line = l.unwrap();
        line.chars().enumerate().for_each(|(j, c)| grid.set_raw(i, j, c));
    });
    let a_positions = find_a(&grid);
    let num_words = count_valid_x_mas(&a_positions, &grid);
    Ok(num_words)
}

fn main() {
    let file = File::open(INPUT_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let result = calculate_result(reader.lines()).expect("error calculating result");
    println!("{OUTPUT_MESSAGE}: {result}");
}
