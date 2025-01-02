use std::collections::HashSet;

use utils::Grid;

pub struct Maze {
    grid: Grid,
    wall_locations: HashSet<usize>,
    start_location: usize,
    end_location: usize,
}

const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';
const EMPTY: char = '.';

impl From<&str> for Maze {
    fn from(text: &str) -> Self {
        let (flattened_input, grid) = Grid::parse_input(text);

        let mut wall_locations = HashSet::new();
        let mut start_location = None;
        let mut end_location = None;

        for (index, content) in flattened_input.chars().enumerate() {
            match content {
                WALL => {
                    wall_locations.insert(index);
                },
                START => {
                    if start_location.is_some() {
                        panic!("Maze should only have a single start");
                    }
                    start_location.replace(index);
                },
                END => {
                    if end_location.is_some() {
                        panic!("Maze should only have a single end");
                    }
                    end_location.replace(index);
                },
                EMPTY => {},
                _ => {
                    panic!("Maze contains unknown content '{content}' as index {index}");
                }
            }
        }

        let start_location = start_location.unwrap();
        let end_location = end_location.unwrap();

        Self { grid, wall_locations, start_location, end_location }
    }
}