#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
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
}