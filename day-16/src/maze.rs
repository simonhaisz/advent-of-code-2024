use std::{collections::{BinaryHeap, HashMap, HashSet}, hash::Hash};

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

impl Vector {
    fn new(location: usize, position: Position, direction: Direction) -> Self {
        Self { location, position, direction }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location && self.direction == other.direction
    }
}

impl Eq for Vector {}

impl Hash for Vector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.location.hash(state);
        self.direction.hash(state);
    }
}

struct ScoreVector {
    score: u32,
    vector: Vector,
}

impl ScoreVector {
    fn new(vector: Vector, score: u32) -> Self {
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
        // self.score.cmp(&other.score)
        other.score.cmp(&self.score)
    }
}

fn find_lowest_score_route(maze: &Maze, start_location: usize, end_location: usize) -> Option<u32> {
    let start_position = maze.grid.get_position(start_location).unwrap();
    let start_direction = Direction::East;

    let start_vector = Vector::new(start_location, start_position, start_direction);

    let mut frontier = BinaryHeap::new();
    frontier.push(ScoreVector::new(start_vector.clone(), 0));

    let mut came_from = HashMap::new();
    
    let mut score_so_far = HashMap::new();
    score_so_far.insert(start_vector.clone(), 0);

    let mut score = None;

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();

        if current.vector.location == end_location {
            score.replace(current.score);
            break;
        }

        for next_direction in Direction::orthogonal() {
            let rotation = Direction::orthogonal_delta(current.vector.direction, *next_direction);
            if rotation == 2 {
                // println!("{} - {}", current.vector.direction.to_char(), next_direction.to_char());
                continue;
            }
            let next_position = current.vector.position.adjacent(*next_direction);
            if let Ok(next_location) = maze.grid.get_index(&next_position) {
                if maze.wall_locations.contains(&next_location) {
                    continue;
                }
                let mut next_score = score_so_far[&current.vector];
                next_score += 1;
                if rotation != 0 {
                    next_score += 1000;
                }

                let next_score = next_score;
                
                let next_vector = Vector::new(next_location, next_position, *next_direction);

                if !score_so_far.contains_key(&next_vector) || next_score < score_so_far[&next_vector] {

                    score_so_far.insert(next_vector.clone(), next_score);

                    let next_score_vector = ScoreVector::new(next_vector.clone(), next_score);
                    frontier.push(next_score_vector);

                    came_from.insert(next_vector, current.vector.clone());
                }
            }
        }
    }

    

    if let Some(end_vector) = came_from.keys().find(|k| k.location == end_location) {
        let mut path = vec![];
        let mut movement = HashMap::new();
        let mut current = end_vector;

        while *current != start_vector {
            if let Some(previous) = came_from.get(current) {
                path.insert(0, previous.clone());
                movement.insert(previous.location, previous.clone());
                current = previous
            } else {
                panic!("Cannot trace trail back to ({}, {})", current.position.0, current.position.1);
            }
        }

        for row in 0..maze.grid.row_count {
            for column in 0..maze.grid.column_count {

                let position = Position(row, column);
                let location = maze.grid.get_index(&position).unwrap();

                if maze.wall_locations.contains(&location) {
                    print!("{WALL}");
                // } else if maze.start_location == location {
                //     print!("{START}");
                // } else if maze.end_location == location {
                //     print!("{END}");
                } else if let Some(m) = movement.get(&location) {
                    let c = m.direction.to_char();

                    print!("{c}");
                } else {
                    print!("{EMPTY}");
                }
            }

            println!();
        }

        let mut s = 0;
        let mut previous = None;

        for (i, m) in path.iter().enumerate() {
            if i != 0 {
                let p: Vector = previous.take().unwrap();

                let rotation = Direction::orthogonal_delta(p.direction, m.direction) as u32;

                if rotation == 2 {
                    panic!("Too much rotation");
                }

                s += 1;

                if rotation != 0 {
                    s += 1000;
                }
            }

            previous.replace(m.clone());
        }

        println!("Computed score {s}");
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