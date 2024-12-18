use std::collections::HashMap;

use utils::Grid;

pub type Path = Vec<usize>;

pub struct Map {
    flattened_topography: String,
    grid: Grid,
    trailhead_locations: Vec<usize>,
}

impl Map {
    pub fn find_all_trails(&self) -> HashMap<usize, Vec<Path>> {
        let mut all_trails = HashMap::new();

        for trailhead in self.trailhead_locations.iter() {
            let trails = find_trailhead_trails(&self.grid, &self.flattened_topography, *trailhead);

            all_trails.insert(*trailhead, trails);
        }

        all_trails
    }
}

pub fn find_trailhead_trails(grid: &Grid, flattened_topography: &str, trailhead: usize) -> Vec<Path> {
    
}

fn find_trailhead_trails_inner(grid: &Grid, flattened_topography: &str, paths: Vec<Path>) {

}

pub fn score_trails(trails: HashMap<usize, Vec<Path>>) -> usize {
    trails.values()
        .map(|p| p.len())
        .sum()
}

const TRAILHEAD: char = '0';

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let (flattened_topography, grid) = Grid::parse_input(input);

        let trailhead_locations = flattened_topography.chars().enumerate()
            .filter(|(_, c)| TRAILHEAD == *c)
            .map(|(i, _)| i)
            .collect();

        Self { flattened_topography, grid, trailhead_locations }
    }
}