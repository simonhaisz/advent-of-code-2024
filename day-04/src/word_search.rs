use crate::grid::{Direction, Grid, Position};

pub struct WordSearch {
    search_text: String,
    flattened_input: String,
    grid: Grid,
}

impl WordSearch {
    pub fn new(search_text: &str, input_text: &str) -> Self {
        assert!(search_text.len() > 0);
        let (flattened_input, grid) = process_input(input_text);

        let search_text = search_text.to_string();

        Self { search_text, flattened_input, grid }
    }

    pub fn search_all(&self) -> usize {
        let mut count = 0;

        for row in 0..self.grid.row_count {
            for column in 0..self.grid.column_count {
                let position = Position(row, column);

                let found_directions = self.search(&position);

                count += found_directions.len()
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
            c.expect(&format!("Character index {i} is invalid"))
        };

        let at_end = |i| {
            let done = i == self.search_text.len();
            done
        };

        let mut search_position = position.clone();

        let mut input_index = self.grid.get_index(&search_position).unwrap();

        let get_input_char = |i| {
            let c = self.flattened_input.chars().nth(i);
            c.expect(&format!("Character index {i} is invalid"))
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
}

fn process_input(input_text: &str) -> (String, Grid) {
    let input_text = input_text.trim();
    let found_new_lines = input_text.match_indices('\n').collect::<Vec<_>>();
    assert!(found_new_lines.len() > 0, "Input text requires multiple lines");
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

    (input_text.replace('\n', ""), grid)
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
        let (flattened_input, grid) = process_input(EXAMPLE);

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
}