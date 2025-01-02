use core::fmt;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match *self {
            Direction::North => "North",
            Direction::NorthEast => "NorthEast",
            Direction::East => "East",
            Direction::SouthEast => "SouthEast",
            Direction::South => "South",
            Direction::SouthWest => "SouthWest",
            Direction::West => "West",
            Direction::NorthWest => "NorthWest",
        };

        write!(f, "{text}")
    }
}

impl Direction {
    pub fn all() -> &'static [Direction] {
        &[
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
    }

    pub fn orthogonal() -> &'static [Direction] {
        &[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    pub fn orthogonal_delta(a: Direction, b: Direction) -> u8 {
        let delta = (a as isize - b as isize).abs();

        u8::try_from(delta).unwrap() / 2
    }

    pub fn orthogonal_next(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            _ => panic!("Orthogonal only supports North, East, South, and West")
        }
    }

    pub fn orthogonal_previous(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            _ => panic!("Orthogonal only supports North, East, South, and West")
        }
    }

    pub fn orthogonal_flip(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            _ => panic!("Orthogonal only supports North, East, South, and West")
        }
    }

    pub fn orthogonal_rotate(&self, clicks: u8) -> Direction {
        let mut direction = *self;

        for _ in 0..clicks {
            direction = direction.orthogonal_next();
        }

        direction
    }

    pub fn line(&self) -> (bool, bool) {
        match *self {
            Direction::North => {
                (true, false)
            },
            Direction::East => {
                (false, true)
            },
            Direction::South => {
                (true, true)
            },
            Direction::West => {
                (false, false)
            },
            _ => panic!("Guard can only travel ^>v<")
        }
    }

    pub fn clockwise_orthogonal(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::NorthEast => Direction::SouthEast,
            Direction::East => Direction::South,
            Direction::SouthEast => Direction::SouthWest,
            Direction::South => Direction::West,
            Direction::SouthWest => Direction::NorthWest,
            Direction::West => Direction::North,
            Direction::NorthWest => Direction::NorthEast,
        }
    }
}

const DIRECTION_UP: char = '^';
const DIRECTION_RIGHT: char = '>';
const DIRECTION_DOWN: char = 'v';
const DIRECTION_LEFT: char = '<';

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            DIRECTION_UP => Direction::North,
            DIRECTION_DOWN => Direction::South,
            DIRECTION_RIGHT => Direction::East,
            DIRECTION_LEFT => Direction::West,
            _ => panic!("Unexpected direction char '{c}' - expected {DIRECTION_UP}, {DIRECTION_RIGHT}, {DIRECTION_DOWN}, {DIRECTION_LEFT}")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn adjacent(&self, direction: Direction) -> Position {
        match direction {
            Direction::North => Position(self.0 - 1, self.1),
            Direction::NorthEast => Position(self.0 - 1, self.1 + 1),
            Direction::East => Position(self.0, self.1 + 1),
            Direction::SouthEast => Position(self.0 + 1, self.1 + 1),
            Direction::South => Position(self.0 + 1, self.1),
            Direction::SouthWest => Position(self.0 + 1, self.1 - 1),
            Direction::West => Position(self.0, self.1 - 1),
            Direction::NorthWest => Position(self.0 - 1, self.1 - 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub row_count: i32,
    pub column_count: i32,
}

impl Grid {
    pub fn new(row_count: i32, column_count: i32) -> Self {
        Self { row_count, column_count }
    }

    pub fn validate_position(&self, position: &Position, assert: bool) -> bool {
        let row_valid = position.0 >= 0 && position.0 < self.row_count;
        let column_valid = position.1 >= 0 && position.1 < self.column_count;
        if assert {
            assert!(row_valid, "Position row {} is out of bounds of the grid {}", position.0, self.row_count);
            assert!(column_valid, "Position column {} is out of bounds of the grid {}", position.1, self.column_count);
        }

        row_valid && column_valid
    }

    pub fn validate_index(&self, index: usize, assert: bool) -> bool {
        let index_limit = (self.row_count * self.column_count) as usize;
        if assert {
            assert!(index < index_limit, "Index {} is out of bounds of the grid {}", index, index_limit);
        }

        index < index_limit
    }

    pub fn get_index(&self, position: &Position) -> Result<usize, String> {
        if self.validate_position(position, false) {
            Ok((position.0 * self.column_count + position.1) as usize)
        } else {
            Err(format!("Position '{position:?}' invalid in grid '{self:?}'"))
        }
    }

    pub fn get_position(&self, index: usize) -> Result<Position, String> {
        if self.validate_index(index, false) {
            let index = index as i32;
            let row = index / self.column_count;
            let column = index % self.column_count;
    
            Ok(Position(row, column))
        } else {
            Err(format!("Index {index} is invalid in grid '{self:?}'"))
        }
    }

    pub fn same_row(&self, a: usize, b: usize) -> bool {
        (a as i32) / self.column_count == (b as i32) / self.column_count
    }

    pub fn same_column(&self, a: usize, b: usize) -> bool {
        (a as i32) % self.column_count == (b as i32) % self.column_count
    }

    pub fn parse_input(input: &str) -> (String, Grid) {
        let input = input.trim().replace("\r\n", "\n");

        let found_new_lines = input.match_indices("\n").collect::<Vec<_>>();
        
        if found_new_lines.len() == 0 {
            panic!("Input test requires multiple lines");
        }

        let row_count = found_new_lines.len() + 1; // assume trim is called so there is no final new-line
    
        let mut column_count = None;
    
        for (index, (new_line, _)) in found_new_lines.into_iter().enumerate() {
            if column_count.is_none() {
                column_count = Some(new_line)
            } else if let Some(column_count) = column_count {
                assert_eq!((index + 1) * column_count + index, new_line);
            }
        }
    
        let column_count = column_count.unwrap();
    
        let grid = Grid::new(row_count as i32, column_count as i32);
    
        (input.replace("\n", ""), grid)
    }

    pub fn predict_move(&self, origin: &Position, velocity: &Velocity) -> Position {
        let new_row = origin.0 + velocity.0;
        let new_column = origin.1 + velocity.1;

        let new_row = new_row.rem_euclid(self.row_count);
        let new_column = new_column.rem_euclid(self.column_count);

        Position(new_row, new_column)
    }

    pub fn quadrant(&self, index: usize) -> Option<Direction> {
        let center_skip_width = 1;

        let position = self.get_position(index).unwrap();

        let center_row = self.row_count / 2;

        let center_row_range = center_row - center_skip_width + 1..=center_row + center_skip_width - 1;

        let center_column = self.column_count / 2;

        let center_column_range = center_column - center_skip_width + 1..=center_column + center_skip_width - 1;

        if center_row_range.contains(&position.0) || center_column_range.contains(&position.1) {
            None
        } else {
            let top_side = if position.0 < *center_row_range.start() {
                true
            } else if position.0 > *center_row_range.end() {
                false
            } else {
                panic!("Position does not fit in left or right side")
            };

            let left_side = if position.1 < *center_column_range.start() {
                true
            } else if position.1 > *center_column_range.end() {
                false
            } else {
                panic!("Position does not fit in top or bottom side")
            };

            let quadrant = if top_side && left_side {
                Direction::NorthWest
            } else if top_side && !left_side {
                Direction::NorthEast
            } else if !top_side && !left_side {
                Direction::SouthEast
            } else if !top_side && left_side {
                Direction::SouthWest
            } else {
                panic!("Position does not fit into any quadrant")
            };

            Some(quadrant)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Velocity(pub i32, pub i32);