use std::collections::{BinaryHeap, HashMap, HashSet};

use utils::{Direction, Grid, Position};

pub struct Maze {
    grid: Grid,
    wall_locations: HashSet<usize>,
    start_location: usize,
    end_location: usize,
}

impl Maze {
    pub fn find_lowest_score_route(&self) -> Option<u32> {
        find_lowest_score_route(self, self.start_location, self.end_location)
    }
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

#[derive(Clone, Debug)]
struct Vector {
    location: usize,
    position: Position,
    direction: Direction,
}

struct ScoreVector {
    score: u32,
    vector: Vector,
}

impl ScoreVector {
    fn new(location: usize, position: Position, direction: Direction, score: u32) -> Self {
        let vector = Vector { location, position, direction };

        Self { score, vector }
    }
}

impl PartialEq for ScoreVector {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for ScoreVector {}

impl PartialOrd for ScoreVector {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoreVector {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

fn find_lowest_score_route(maze: &Maze, start_location: usize, end_location: usize) -> Option<u32> {
    let start_position = maze.grid.get_position(start_location).unwrap();
    let start_direction = Direction::East;

    let mut frontier = BinaryHeap::new();
    frontier.push(ScoreVector::new(start_location, start_position, start_direction, 0));

    let mut came_from = HashMap::new();
    
    let mut score_so_far = HashMap::new();
    score_so_far.insert(start_location, 0);

    let mut score = None;

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current.vector.location == end_location {
            score.replace(current.score);
            break;
        }

        for next_direction in Direction::orthogonal() {
            let next_position = current.vector.position.adjacent(*next_direction);
            if let Ok(next_location) = maze.grid.get_index(&next_position) {
                let rotation = Direction::orthogonal_delta(current.vector.direction, *next_direction) as u32;
                let next_score = score_so_far[&current.vector.location] + 1 + rotation * 1000;
                
                if !score_so_far.contains_key(&next_location) || next_score < score_so_far[&next_location] {
                    score_so_far.insert(next_location, next_score);

                    let next_score_vector = ScoreVector::new(next_location, next_position, *next_direction, next_score);
                    frontier.push(next_score_vector);

                    came_from.insert(next_location, current.vector.location);
                }
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text = r"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
        ".trim();

        let maze = Maze::from(text);

        let lowest_score = maze.find_lowest_score_route();

        assert_eq!(Some(7036), lowest_score);
    }

    #[test]
    fn example_2() {
        let text = r"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
        ".trim();

        let maze = Maze::from(text);

        let lowest_score = maze.find_lowest_score_route();

        assert_eq!(Some(11048), lowest_score);
    }
}