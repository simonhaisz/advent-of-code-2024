use std::collections::HashMap;

use utils::{Direction, Grid};

pub type Path = Vec<usize>;

pub struct Map {
    flattened_topography: String,
    grid: Grid,
    trailhead_locations: Vec<usize>,
    destination_locations: Vec<usize>
}

impl Map {
    pub fn find_all_trails(&self) -> HashMap<usize, Vec<Path>> {
        let mut all_trails = HashMap::new();

        for trailhead in self.trailhead_locations.iter() {
            for destination in self.destination_locations.iter() {
                let trails = find_trails(&self.grid, &self.flattened_topography, *trailhead, *destination);

                all_trails.insert(*trailhead, trails);
            }
        }

        all_trails
    }
}

pub fn find_trails(grid: &Grid, flattened_topography: &str, start: usize, destination: usize) -> Vec<Path> {
    let mut final_paths = vec![];

    let mut working_paths = vec![];

    let path = vec![start];
    working_paths.push(path);

    while !working_paths.is_empty() {
        let path = working_paths.remove(0);

        let last = *path.last().unwrap();

        if last == destination {
            // do nothing?
            final_paths.push(path);
            continue;;
        }

        let last_position = grid.get_position(last).unwrap();
        let last_value = flattened_topography.chars().nth(last).unwrap();

        for direction in Direction::orthogonal() {
            let adjacent_position = last_position.adjacent(*direction);
            if !grid.validate_position(&adjacent_position, false) {
                continue;;
            }
            let adjacent_value = flattened_topography.chars().nth(last).unwrap();

            if valid_move(last_value, adjacent_value) {
                let adjacent = grid.get_index(&adjacent_position).unwrap();

                if path.contains(&adjacent) {
                    continue;
                }

                let mut adjacent_path = path.clone();
                adjacent_path.push(adjacent);

                working_paths.push(adjacent_path);
            }
        }
    }

    final_paths
}

fn valid_move(from: char, to: char) -> bool {
    let from = from.to_digit(10).unwrap();
    let to = to.to_digit(10).unwrap();

    to > from && to - from == 1
}

pub fn score_trails(trails: HashMap<usize, Vec<Path>>) -> usize {
    trails.values()
        .map(|p| p.len())
        .sum()
}

const TRAILHEAD: char = '0';
const DESTINATION: char = '9';

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let (flattened_topography, grid) = Grid::parse_input(input);

        let trailhead_locations = flattened_topography.chars().enumerate()
            .filter(|(_, c)| TRAILHEAD == *c)
            .map(|(i, _)| i)
            .collect();

        let destination_locations = flattened_topography.chars().enumerate()
            .filter(|(_, c)| DESTINATION == *c)
            .map(|(i, _)| i)
            .collect();

        Self { flattened_topography, grid, trailhead_locations, destination_locations }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_EXAMPLE: &'static str = r"
0123
1234
8765
9876
    ";

    #[test]
    fn example() {
        let map = Map::from(BASIC_EXAMPLE);

        let all_trails = map.find_all_trails();

        let trails = all_trails.get(&0).unwrap();

        assert_eq!(4, trails.len());
    }
}