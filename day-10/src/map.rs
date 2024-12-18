use std::collections::{BinaryHeap, HashMap};

use utils::{Direction, Grid};

pub type Path = Vec<usize>;

pub struct Map {
    flattened_topography: String,
    grid: Grid,
    trailhead_locations: Vec<usize>,
    destination_locations: Vec<usize>
}

#[derive(Clone, Copy)]
pub enum FindTrailRule {
    Any,
    All
}

impl Map {
    pub fn find_trailhead_trails(&self, rule: FindTrailRule) -> HashMap<usize, Vec<Path>> {
        let mut trailhead_trails: HashMap<usize, Vec<Path>> = HashMap::new();

        for trailhead in self.trailhead_locations.iter() {
            for destination in self.destination_locations.iter() {
                match rule {
                    FindTrailRule::Any => {
                        let trail = find_any_trail(&self.grid, &self.flattened_topography, *trailhead, *destination);

                        if let Some(trail) = trail {
                            let trails = trailhead_trails.entry(*trailhead).or_default();
                            trails.push(trail);
                        }
                    },
                    FindTrailRule::All => {
                        let found_trails = find_trails(&self.grid, &self.flattened_topography, *trailhead, *destination);

                        for trail in found_trails.into_iter() {
                            let existing_trails = trailhead_trails.entry(*trailhead).or_default();
                            existing_trails.push(trail);
                        }
                    }
                }

            }
        }

        trailhead_trails
    }
}

pub fn find_any_trail(grid: &Grid, flattened_topography: &str, start: usize, end: usize) -> Option<Path> {
    let mut trails: BinaryHeap<Location> = BinaryHeap::new();
    trails.push(Location::from(start));

    let mut came_from: HashMap<usize, usize> = HashMap::new();

    let mut found_end = false;

    while !trails.is_empty() {
        let current = trails.pop().unwrap();

        if current.index == end {
            found_end = true;
            break;
        }

        let current_position = grid.get_position(current.index).unwrap();
        let current_value = flattened_topography.chars().nth(current.index).unwrap();

        for direction in Direction::orthogonal() {
            let adjacent_position = current_position.adjacent(*direction);
            if !grid.validate_position(&adjacent_position, false) {
                continue;
            }
            let adjacent = grid.get_index(&adjacent_position).unwrap();
            let adjacent_value = flattened_topography.chars().nth(adjacent).unwrap();

            if valid_move(current_value, adjacent_value) {
                trails.push(Location::from(adjacent));
                came_from.insert(adjacent, current.index);
            }
        }
    }

    if !found_end {
        None
    } else {
        let mut path = vec![end];
        let mut current = end;

        while current != start {
            let prev = came_from[&current];
            path.insert(0, prev);
            current = prev;
        }

        Some(path)
    }
}

struct Location {
    index: usize
}

impl From<usize> for Location {
    fn from(value: usize) -> Self {
        Self { index: value }
    }
}

impl PartialEq for Location {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Location {}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Location {
    fn cmp(&self, _other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}



pub fn find_trails(grid: &Grid, flattened_topography: &str, start: usize, end: usize) -> Vec<Path> {
    let mut final_paths = vec![];

    let mut working_paths = vec![];

    let path = vec![start];
    working_paths.push(path);

    while !working_paths.is_empty() {
        let path = working_paths.remove(0);

        let last = *path.last().unwrap();

        if last == end {
            final_paths.push(path);
            continue;
        }

        let last_position = grid.get_position(last).unwrap();
        let last_value = flattened_topography.chars().nth(last).unwrap();

        for direction in Direction::orthogonal() {
            let adjacent_position = last_position.adjacent(*direction);
            if !grid.validate_position(&adjacent_position, false) {
                continue;
            }
            let adjacent = grid.get_index(&adjacent_position).unwrap();
            let adjacent_value = flattened_topography.chars().nth(adjacent).unwrap();

            if valid_move(last_value, adjacent_value) {
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
    if to.is_digit(10) {
        let from = from.to_digit(10).unwrap();
        let to = to.to_digit(10).unwrap();

        to > from && to - from == 1
    } else {
        false
    }
}

pub fn score_trails(trails: &HashMap<usize, Vec<Path>>) -> usize {
    trails.values()
        .map(|p| p.len())
        .sum()
}

pub fn rate_trails(trails: &HashMap<usize, Vec<Path>>) -> usize {
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

    #[test]
    fn basic_example() {
        let input = r"
0123
1234
8765
9876
        ".trim();
        let map = Map::from(input);

        let trailhead_trails = map.find_trailhead_trails(FindTrailRule::Any);

        assert_eq!(1, trailhead_trails.len());

        let trails = trailhead_trails.get(&0).unwrap();

        assert_eq!(1, trails.len());

        let score = score_trails(&trailhead_trails);
        assert_eq!(1, score);
    }

    #[test]
    fn simple_example() {
        let input = r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
    ".trim();

        let map = Map::from(input);

        let trailhead_trails = map.find_trailhead_trails(FindTrailRule::Any);

        let score = score_trails(&trailhead_trails);

        assert_eq!(36, score);
    }

    #[test]
    fn all_trails_single() {
        let input = r"
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
        ".trim();

        let map = Map::from(input);

        let trailhead_trails = map.find_trailhead_trails(FindTrailRule::All);
        assert_eq!(1, trailhead_trails.len());

        let trails = trailhead_trails.get(&5).unwrap();

        assert_eq!(3, trails.len());
    }

    #[test]
    fn all_trails_thirteen() {
        let input = r"
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
        ".trim();

        let map = Map::from(input);

        let trailhead_trails = map.find_trailhead_trails(FindTrailRule::All);
        assert_eq!(1, trailhead_trails.len());

        let trails = trailhead_trails.get(&3).unwrap();

        assert_eq!(13, trails.len());
    }

    #[test]
    fn all_trails_rating() {
        let input = r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
        ".trim();

        let map = Map::from(input);

        let trailhead_trails = map.find_trailhead_trails(FindTrailRule::All);
        
        let rating = rate_trails(&trailhead_trails);

        assert_eq!(81, rating);
    }
}