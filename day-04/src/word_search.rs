use utils::{Direction, Grid, Position};

pub struct WordSearch {
    search_text: String,
    flattened_input: String,
    grid: Grid,
}

impl WordSearch {
    pub fn new(search_text: &str, input_text: &str) -> Self {
        assert!(search_text.len() > 0);
        let (flattened_input, grid) = Grid::parse_input(input_text);

        let search_text = search_text.to_string();

        Self { search_text, flattened_input, grid }
    }

    pub fn search_all(&self) -> usize {
        let mut count = 0;

        for row in 0..self.grid.row_count {
            for column in 0..self.grid.column_count {
                let position = Position(row, column);

                let found_directions = self.search(&position);

                count += found_directions.len();
            }
        }

        count
    }

    fn search(&self, position: &Position) -> Vec<Direction> {
        let mut found_directions = vec![];

        for direction in Direction::all() {
            let (success, failure_index) = self.search_direction(position, *direction);
            if success {
                found_directions.push(*direction);
            } else if failure_index == 0 {
                break;
            }
        }

        found_directions
    }

    fn search_direction(&self, position: &Position, direction: Direction) -> (bool, usize) {
        let mut search_char_index: usize = 0;

        let get_search_char = |i| {
            let c = self.search_text.chars().nth(i);
            c.expect(&format!("Search index {i} is invalid"))
        };

        let at_end = |i| {
            let done = i == self.search_text.len();
            done
        };

        let mut search_position = position.clone();

        let mut input_index = self.grid.get_index(&search_position).unwrap();

        let get_input_char = |i| {
            let c = self.flattened_input.chars().nth(i);
            c.expect(&format!("Input index {i} is invalid"))
        };

        loop {
            if get_search_char(search_char_index) != get_input_char(input_index) {
                return (false, search_char_index);
            }

            search_char_index += 1;

            if at_end(search_char_index) {
                break;
            }

            search_position = search_position.adjacent(direction);
            if !self.grid.validate_position(&search_position, false) {
                return (false, search_char_index);
            }
            input_index = self.grid.get_index(&search_position).unwrap();
        }

        (true, 0)
    }

    pub fn search_x_all(&self) -> usize {
        let mut count = 0;

        for row in 0..self.grid.row_count {
            for column in 0..self.grid.column_count {
                let position = Position(row, column);

                if !self.grid.validate_position(&position, false) {
                    break;
                }

                if self.search_x(&position) {
                    count += 1;
                }
            }
        }

        count
    }

    fn search_x(&self, center: &Position) -> bool {
        let search_length = self.search_text.len();
        if search_length < 3 {
            panic!("Search X requires a search text with at least 3 characters '{}'", self.search_text)
        }
        if search_length % 2 == 0 {
            panic!("Search X requires a search text that has an odd number of characters '{}'", self.search_text);
        }

        let get_input_char = |i| {
            let c = self.flattened_input.chars().nth(i);
            c.expect(&format!("Input index {i} is invalid"))
        };

        let search_midpoint_index = search_length / 2;

        let center_char = self.search_text.chars().nth(search_midpoint_index).unwrap();

        let center_index = self.grid.get_index(center).unwrap();
        
        if center_char != get_input_char(center_index) {
            return false;
        }

        let left_search_text = self.search_text[0..search_midpoint_index].chars().rev().collect::<String>();
        let right_search_text = self.search_text[search_midpoint_index+1..search_length].to_string();

        let get_left_search_char = |i| {
            let c = left_search_text.chars().nth(i);
            c.expect(&format!("Left search {i} is invalid"))
        };

        let get_right_search_char = |i| {
            let c = right_search_text.chars().nth(i);
            c.expect(&format!("Left search {i} is invalid"))
        };

        let diagonals = &[
            (Direction::SouthWest, Direction::NorthEast),
            (Direction::NorthWest, Direction::SouthEast),
        ];

        let mut matches_count = 0;

        for (a, b) in diagonals.into_iter() {
            let mut flipped = false;
            loop {
                let (left, right) = if flipped {
                    (b, a)
                } else {
                    (a, b)
                };

                let mut center_offset = 0;

                let left_position = center.adjacent(*left);
                if !self.grid.validate_position(&left_position, false) {
                    return false;
                }

                let right_position = center.adjacent(*right);
                if !self.grid.validate_position(&right_position, false) {
                    return false;
                }

                loop {

                    let left_index = self.grid.get_index(&left_position).unwrap();
                    if get_left_search_char(center_offset) != get_input_char(left_index) {
                        break;
                    }
                    
                    let right_index = self.grid.get_index(&right_position).unwrap();
                    if get_right_search_char(center_offset) != get_input_char(right_index) {
                        break;
                    }

                    center_offset += 1;

                    if center_offset >= search_midpoint_index {
                        matches_count += 1;
                        break;
                    }
                }

                if flipped {
                    break;
                } else {
                    flipped = true;
                }
            }
        }

        matches_count == 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
    ";

    #[test]
    fn process_input_example() {
        let (flattened_input, grid) = Grid::parse_input(&EXAMPLE);

        assert_eq!(flattened_input.len(), 100);
        assert_eq!(grid.row_count, 10);
        assert_eq!(grid.column_count, 10);
    }

    #[test]
    fn search_example() {
        let word_search = WordSearch::new("XMAS", EXAMPLE);

        let position = Position(4, 0);
        let found_directions = word_search.search(&position);

        assert_eq!(found_directions.len(), 1);
        assert_eq!(found_directions[0], Direction::East);
    }

    #[test]
    fn search_all_example() {
        let word_search = WordSearch::new("XMAS", EXAMPLE);

        let count = word_search.search_all();

        assert_eq!(count, 18);
    }

    #[test]
    fn search_x_example() {
        let word_search = WordSearch::new("MAS", EXAMPLE);

        let position = Position(1, 2);
        let found_matches = word_search.search_x(&position);

        assert!(found_matches);
    }

    #[test]
    fn search_x_all_example() {
        let word_search = WordSearch::new("MAS", EXAMPLE);

        let count = word_search.search_x_all();

        assert_eq!(count, 9);
    }
}