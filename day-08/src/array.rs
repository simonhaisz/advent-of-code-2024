use std::collections::HashMap;

use utils::Grid;

pub struct Array {
    grid: Grid,
    antenna_indices_map: HashMap<char, Vec<usize>>,
}

const EMPTY_SPACE: char = '.';

impl From<&str> for Array {
    fn from(input: &str) -> Self {
        let (flattened_input, grid) = Grid::parse_input(input);

        let mut antenna_indices_map = HashMap::new();

        for (index, c) in flattened_input.chars().enumerate() {
            if c != EMPTY_SPACE {
                let antenna: &mut Vec<usize> = antenna_indices_map.entry(c).or_default();
                antenna.push(index);
            }
        }

        Array { grid, antenna_indices_map }
    }
}