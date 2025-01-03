use std::collections::{BinaryHeap, HashMap, HashSet};

use utils::{Direction, Grid, Position};

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

struct ScoreLocation {
    location: usize,
    position: Position,
    score: u32,
}

impl ScoreLocation {
    fn new(location: usize, position: Position, score: u32) -> Self {
        Self { location, position, score }
    }
}

impl PartialEq for ScoreLocation {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for ScoreLocation {}

impl PartialOrd for ScoreLocation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoreLocation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

fn find_lowest_score_route(maze: &Maze, start_location: usize, end_location: usize) {
    let start_position = maze.grid.get_position(start_location).unwrap();

    let mut frontier = BinaryHeap::new();
    frontier.push(ScoreLocation::new(start_location, start_position, 0));

    let mut came_from = HashMap::new();
    
    let mut movement_so_far = HashMap::new();
    movement_so_far.insert(start_location, 0);

    let mut current_direction = Direction::East;

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current.location == end_location {
            break;
        }

        for next_direction in Direction::orthogonal() {
            let next_position = current.position.adjacent(*next_direction);
            if let Some(next_location) = maze.grid.get_index(&next_position) {
                
            }
        }
    }
}