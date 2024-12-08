use std::collections::HashSet;

use utils::{Direction, Grid, Position};

pub struct Map {
    starting_guard: Guard,
    obstacle_indices: Vec<usize>,
    grid: Grid,
}

impl Map {
    pub fn predict_guard(&self) -> (Vec<usize>, HashSet<usize>) {

        let mut guard = self.starting_guard.clone();
        let mut guard_indices = vec![guard.index];

        loop {
            let (new_index, movement, hit_obstacle) = self.move_guard(&guard);

            guard.index = new_index;

            guard_indices.extend(&movement);

            if hit_obstacle {
                guard.direction = guard.direction.clockwise_orthogonal();
            } else {
                break;
            }
        }

        let unique_guard_indices = guard_indices.clone().into_iter().collect::<HashSet<_>>();

        (guard_indices, unique_guard_indices)
    }

    fn move_guard(&self, guard: &Guard) -> (usize, Vec<usize>, bool) {
        let (next_index, distance, hit_obstacle) = self.find_next_index(guard.index, &guard.direction);

        let mut movement = vec![];

        let mut previous_index = guard.index;

        for _ in 1..=distance {
            let (same_column, increasing) = guard.direction.line();
            let step_index = if same_column {
                if increasing {
                    previous_index + (self.grid.column_count as usize)
                } else {
                    previous_index - (self.grid.column_count as usize)
                }
            } else {
                if increasing {
                    previous_index + 1
                } else {
                    previous_index - 1
                }
            };

            movement.push(step_index);

            previous_index = step_index;
        }
        
        (next_index, movement, hit_obstacle)
    }

    fn find_next_index(&self, start_index: usize, direction: &Direction) -> (usize, usize, bool) {
        let (same_column, increasing) = direction.line();

        let blocking_obstacles = self.obstacle_indices.iter()
            .filter(|&&oi| {
                if same_column  {
                    if !self.grid.same_column(start_index, oi) {
                        return false;
                    }
                } else {
                    if !self.grid.same_row(start_index, oi) {
                        return false;
                    }
                }

                if increasing {
                    oi > start_index
                } else {
                    oi < start_index
                }
            }).collect::<Vec<_>>();

        let starting_position = self.grid.get_position(start_index).unwrap();

        if blocking_obstacles.len() == 0 {
            let (end_position, distance) = if same_column {
                let end_position = Position(
                    if increasing {
                        self.grid.row_count - 1
                    } else {
                        0
                    },
                    starting_position.1
                );
                let distance = (end_position.0 - starting_position.0).abs() as usize;
                (end_position, distance)
            } else {
                let end_position = Position(
                    starting_position.0,
                    if increasing {
                        self.grid.column_count - 1
                    } else {
                        0
                    }
                );
                let distance = (end_position.1 - starting_position.1).abs() as usize;
                (end_position, distance)
            };

            let end_index = self.grid.get_index(&end_position).unwrap();

            (end_index, distance, false)
        } else {
            let blocking_index = if increasing {
                 **blocking_obstacles.first().unwrap()
            } else {
                **blocking_obstacles.last().unwrap()
            };

            let obstacle_position = self.grid.get_position(blocking_index).unwrap();

            let (end_position, distance) = if same_column {
                let end_position = Position(
                    if increasing {
                        obstacle_position.0 - 1
                    } else {
                        obstacle_position.0 + 1
                    },
                    starting_position.1
                );
                let distance = (end_position.0 - starting_position.0).abs() as usize;
                (end_position, distance)
            } else {
                let end_position = Position(
                    starting_position.0,
                    if increasing {
                        obstacle_position.1 - 1
                    } else {
                        obstacle_position.1 + 1
                    }
                );
                let distance = (end_position.1 - starting_position.1).abs() as usize;
                (end_position, distance)
            };
            
            let end_index = self.grid.get_index(&end_position).unwrap();

            (end_index, distance, true)
        }
    }
}

#[derive(Clone)]
struct Guard {
    index: usize,
    direction: Direction,
}

const GUARD_POINTING_NORTH: &'static str = "^";
const EMPTY_SPACE: &'static str = ".";
const OBSTACLE: &'static str = "#";

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let (mut flattened_input, grid) = Grid::parse_input(input);

        let guard_index = flattened_input.find(GUARD_POINTING_NORTH).expect("Guard should exist on the map pointing North");
        flattened_input.replace_range(guard_index..guard_index + 1, EMPTY_SPACE);

        let starting_guard = Guard { index: guard_index, direction: Direction::North };

        let obstacle_indices = flattened_input.match_indices(OBSTACLE).map(|(i, _)| i).collect::<Vec<_>>();

        Self { starting_guard, obstacle_indices, grid }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    const EXAMPLE: &'static str = r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
    ";
    #[test]
    fn map_example() {
        let map = Map::from(EXAMPLE);

        let guard_starting_position = map.grid.get_position(map.starting_guard.index).unwrap();

        println!("Guard starting position ({}, {})", guard_starting_position.0, guard_starting_position.1);

        for obstacle_index in map.obstacle_indices.iter() {
            let obstacle_postion = map.grid.get_position(*obstacle_index).unwrap();

            println!("Obstacle position ({}, {})", obstacle_postion.0, obstacle_postion.1);
        }

        let (guard_indexes, guard_unique_indexes) = map.predict_guard();

        let guard_positions = guard_indexes.iter().map(|i| map.grid.get_position(*i).unwrap()).collect::<Vec<_>>();

        for p in guard_positions.iter() {
            println!("Guard movement ({}, {})", p.0, p.1);
        }

        let unique_guard_positions = guard_unique_indexes.into_iter().collect::<HashSet<_>>();

        assert_eq!(unique_guard_positions.len(), 41);
    }
}