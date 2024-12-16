use std::collections::{HashMap, HashSet};

use utils::{Grid, Position};

pub struct Array {
    grid: Grid,
    antenna_indices_map: HashMap<char, Vec<usize>>,
}

#[derive(Clone, Copy)]
pub enum AntinodeRule {
    TwiceDistance,
    Inline
}

impl Array {
    pub fn find_all_antinodes(&self, rule: AntinodeRule) -> HashMap<char, Vec<usize>> {
        let mut antenna_antinodes_map = HashMap::new();

        for (antenna, locations) in self.antenna_indices_map.iter() {
            if locations.len() < 2 {
                continue;
            }

            let mut antinodes = vec![];

            let locations_count = locations.len();

            for a in 0..locations_count - 1 {
                for b in a + 1..locations_count {
                    let pair = (locations[a], locations[b]);

                    let pair_nodes = find_antinodes(&self.grid, rule, pair);

                    antinodes.extend(pair_nodes);
                }
            }

            if antinodes.len() > 0 {
                antenna_antinodes_map.insert(*antenna, antinodes);
            }
        }

        antenna_antinodes_map
    }

    pub fn find_unique_antinode_locations(&self, rule: AntinodeRule) -> Vec<usize> {
        let antenna_antinodes_map = self.find_all_antinodes(rule);

        let mut unique_locations = HashSet::new();

        for (_, antinodes) in antenna_antinodes_map {
            unique_locations.extend(antinodes);
        }

        unique_locations.into_iter().collect::<Vec<_>>()
    }
}

fn find_antinodes(grid: &Grid, rule: AntinodeRule, pair: (usize, usize)) -> Vec<usize> {
    let a = pair.0.min(pair.1);
    let b = pair.0.max(pair.1);
    
    let a_position = grid.get_position(a).unwrap();
    let b_position = grid.get_position(b).unwrap();

    let delta_row = b_position.0 - a_position.0;
    let delta_column = b_position.1 - a_position.1;

    let mut antinodes = vec![];

    match rule {
        AntinodeRule::TwiceDistance => {
            let antinode_a_side = Position(a_position.0 - delta_row, a_position.1 - delta_column);
            if grid.validate_position(&antinode_a_side, false) {
                let antinode_a_side = grid.get_index(&antinode_a_side).unwrap();
                antinodes.push(antinode_a_side);
            }
            
            let anti_node_b_side = Position(b_position.0 + delta_row, b_position.1 + delta_column);
            if grid.validate_position(&anti_node_b_side, false) {
                let antinode_b_side = grid.get_index(&anti_node_b_side).unwrap();
                antinodes.push(antinode_b_side);
            }
        },
        AntinodeRule::Inline => {
            let mut current_position = a_position;

            // go 'backwards' until we go off the grid

            loop {
                current_position = Position(current_position.0 - delta_row, current_position.1 - delta_column);

                if !grid.validate_position(&current_position, false) {
                    break;
                }
            }

            /*
             * from that starting position follow the line until we go off the grid on the other side
             * this will automatically include the antenna locations themselves
             */

            loop {
                current_position = Position(current_position.0 + delta_row, current_position.1 + delta_column);

                if let Ok(current_index) = grid.get_index(&current_position) {
                    antinodes.push(current_index);
                } else {
                    break;
                }
            }
        }
    }

    // TODO: antinodes can exist in between antenna if they are far enough apart

    antinodes
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
    ";

    #[test]
    fn example_twice_distance() {
        let array = Array::from(EXAMPLE);

        let locations = array.find_unique_antinode_locations(AntinodeRule::TwiceDistance);

        assert_eq!(14, locations.len());
    }

    #[test]
    fn example_inline() {
        let array = Array::from(EXAMPLE);

        let locations = array.find_unique_antinode_locations(AntinodeRule::Inline);

        assert_eq!(34, locations.len());
    }
}