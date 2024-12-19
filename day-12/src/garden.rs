use std::collections::{HashMap, HashSet};

use utils::{Direction, Grid};

pub struct Garden {
    plants: Vec<char>,
    flattened_plants: String,
    grid: Grid,
}

pub struct Plot {
    plant: char,
    locations: Vec<usize>,
    perimeter: u32,
}

impl Garden {
    pub fn map_plots(&self) -> Vec<Plot> {
        let mut plots = vec![];

        for plant in self.plants.iter() {
            let mut all_locations = self.flattened_plants.match_indices(*plant).map(|(i, s)| i).collect::<Vec<_>>();

            while !all_locations.is_empty() {
                let start = all_locations.remove(0);

                let plot_locations = map_plot(&self.grid, &self.flattened_plants, *plant, start);

                all_locations.retain(|l| !plot_locations.contains(l));

                let plot = Plot::new(*plant, plot_locations, self);

                plots.push(plot);
            }
        }

        plots
    }

    pub fn fencing_price(&self) -> u32 {
        let plots = self.map_plots();

        plots.iter()
            .map(|p| p.fencing_price())
            .sum()
    }
}

impl Plot {
    pub fn new(plant: char, locations: Vec<usize>, garden: &Garden) -> Self {
        let mut perimeter = 0;

        for location in locations.iter() {
            let position = garden.grid.get_position(*location).unwrap();

            for direction in Direction::orthogonal() {
                let adjacent_position = position.adjacent(*direction);

                let needs_fence = if garden.grid.validate_position(&adjacent_position, false) {
                    let adjacent = garden.grid.get_index(&adjacent_position).unwrap();
                    let adjacent_plant = garden.flattened_plants.chars().nth(adjacent).unwrap();
                    adjacent_plant != plant
                } else {
                    true
                };

                if needs_fence {
                    perimeter += 1;
                }
            }
        }

        Self { plant, locations, perimeter }
    }
    pub fn contains(&self, location: usize) -> bool {
        self.locations.contains(&location)
    }

    pub fn fencing_price(&self) -> u32 {
        u32::try_from(self.locations.len()).unwrap() * self.perimeter
    }
}

fn map_plot(grid: &Grid, flattened_plants: &str, plant: char, start: usize) -> Vec<usize> {

    let mut final_locations = vec![];

    let mut working_locations = vec![start];

    while !working_locations.is_empty() {

        let current = working_locations.remove(0);
        let current_position = grid.get_position(current).unwrap();

        for direction in Direction::orthogonal() {
            let adjacent_position = current_position.adjacent(*direction);

            if !grid.validate_position(&adjacent_position, false) {
                continue;
            }

            let adjacent = grid.get_index(&adjacent_position).unwrap();

            if final_locations.contains(&adjacent) || working_locations.contains(&adjacent) {
                continue;
            }

            let adjacent_plant = flattened_plants.chars().nth(adjacent).unwrap();

            if plant == adjacent_plant {
                working_locations.push(adjacent);
            }
        }

        final_locations.push(current);
    }

    final_locations
}

impl From<&str> for Garden {
    fn from(input: &str) -> Self {
        let (flattened_plants, grid) = Grid::parse_input(input);

        let mut plants = HashSet::new();

        for p in flattened_plants.chars() {
            plants.insert(p);
        }

        let mut plants = plants.into_iter()
            .collect::<Vec<_>>();

        plants.sort();

        Self { plants, flattened_plants, grid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_EXAMPLE: &'static str = r"
AAAA
BBCD
BBCC
EEEC
    ";

    const SIMPLE_EXAMPLE: &'static str = r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
    ";

    #[test]
    fn find_plots_basic_example() {
        let garden = Garden::from(BASIC_EXAMPLE.trim());

        let plots = garden.map_plots();

        assert_eq!(5, plots.len());

        let fencing_price: u32 = plots.iter()
            .map(|p| p.fencing_price())
            .sum();

        assert_eq!(4 * 10 + 1 * 4 + 4 * 8 + 4 * 10 + 3 * 8, fencing_price);
    }

    #[test]
    fn find_plots_simple_example() {
        let garden = Garden::from(SIMPLE_EXAMPLE.trim());

        let plots = garden.map_plots();

        assert_eq!(11, plots.len());

        let fencing_price: u32 = plots.iter()
            .map(|p| p.fencing_price())
            .sum();

        assert_eq!(1930, fencing_price)
    }
}