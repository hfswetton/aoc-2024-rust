pub mod coord_grid {
    use std::fmt::Debug;
    use std::fs::File;
    use std::io::{BufReader, Lines};
    use strum_macros::EnumIter;

    #[derive(Debug, Clone)]
    pub struct Grid<const GRID_WIDTH: usize, const GRID_HEIGHT: usize, T: Copy + Debug + Default + PartialEq> {
        _grid: [[T; GRID_WIDTH]; GRID_HEIGHT],
    }

    impl<const GRID_WIDTH: usize, const GRID_HEIGHT: usize, T: Copy + Debug + Default + PartialEq> Grid<GRID_WIDTH, GRID_HEIGHT, T> {
        pub fn create() -> Self {
            Self { _grid: [[T::default(); GRID_WIDTH]; GRID_HEIGHT] }
        }

        fn contains_coords(&self, coords: (usize, usize)) -> bool {
            let (i, j) = coords;
            i < GRID_HEIGHT && j < GRID_WIDTH
        }

        pub fn get(&self, coords: (usize, usize)) -> Result<T, ()> {
            let (i, j) = coords;
            if self.contains_coords(coords) {
                Ok(self._grid[i][j])
            } else {
                Err(())
            }
        }

        pub fn set(&mut self, coords: (usize, usize), v: T) -> Result<(), ()> {
            let (i, j) = coords;
            if self.contains_coords(coords) {
                self._grid[i][j] = v;
                Ok(())
            } else {
                Err(())
            }
        }

        pub fn iter(&self) -> impl Iterator<Item=&[T; GRID_WIDTH]> {
            self._grid.iter()
        }
        
        pub fn move_coords(&self, coords: (usize, usize), direction: Direction) -> Result<(usize, usize), ()> {
            match direction {
                Direction::North => if coords.0 > 0 { Ok((coords.0 - 1, coords.1)) } else { Err(()) },
                Direction::East => if coords.1 < GRID_WIDTH - 1 { Ok((coords.0, coords.1 + 1)) } else { Err(()) },
                Direction::South => if coords.0 < GRID_HEIGHT - 1 { Ok((coords.0 + 1, coords.1)) } else { Err(()) },
                Direction::West => if coords.1 > 0 { Ok((coords.0, coords.1 - 1)) } else { Err(()) },
                Direction::Northeast => self.move_coords(self.move_coords(coords, Direction::North)?, Direction::East),
                Direction::Southeast => self.move_coords(self.move_coords(coords, Direction::South)?, Direction::East),
                Direction::Southwest => self.move_coords(self.move_coords(coords, Direction::South)?, Direction::West),
                Direction::Northwest => self.move_coords(self.move_coords(coords, Direction::North)?, Direction::West),
            }
        }

        pub fn position(&self, needle: T) -> Option<(usize, usize)> {
            for i in 0..GRID_HEIGHT {
                for j in 0..GRID_WIDTH {
                    if self.get((i, j)).unwrap() == needle {
                        return Some((i, j));
                    }
                }
            }
            None
        }
    }

    pub fn file_lines_to_char_grid<const GRID_WIDTH: usize, const GRID_HEIGHT: usize>(lines: Lines<BufReader<File>>) -> Result<Grid<GRID_WIDTH, GRID_HEIGHT, char>, String> {
        let mut grid = Grid::create();
        let mut num_lines = 0;
        let mut line_lengths_valid = true;
        lines.enumerate().for_each(|(i, l)| {
            num_lines += 1;
            let line = l.unwrap();
            if line.len() != GRID_WIDTH { line_lengths_valid = false }
            line.chars().enumerate().for_each(|(j, c)| { let _ = grid.set((i, j), c); });
        });
        if num_lines != GRID_HEIGHT {
            Err(format!("incorrect number of lines: found {num_lines}, expected {GRID_HEIGHT}"))
        } else if ! line_lengths_valid {
            Err(format!("incorrect line length: expected {GRID_WIDTH} characters"))
        } else {
            Ok(grid)
        }
    }

    #[derive(Debug, PartialEq, Eq, Copy, Clone, EnumIter, Hash)]
    pub enum Direction {
        North,
        Northeast,
        East,
        Southeast,
        South,
        Southwest,
        West,
        Northwest,
    }
    
    impl Direction {
        pub fn turn_90_clockwise(&self) -> Self {
            match self {
                Self::North => Self::East,
                Self::Northeast => Self::Southeast,
                Self::East => Self::South,
                Self::Southeast => Self::Southwest,
                Self::South => Self::West,
                Self::Southwest => Self::Northwest,
                Self::West => Self::North,
                Self::Northwest => Self::Northeast,
            }
        }

        pub fn turn_90_anticlockwise(&self) -> Self {
            match self {
                Self::North => Self::West,
                Self::Northeast => Self::Northwest,
                Self::East => Self::North,
                Self::Southeast => Self::Northeast,
                Self::South => Self::East,
                Self::Southwest => Self::Southeast,
                Self::West => Self::South,
                Self::Northwest => Self::Southwest,
            }
        }
    }
}
