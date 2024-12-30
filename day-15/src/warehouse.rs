use std::collections::HashSet;

use utils::{Direction, Grid, Position};

pub struct Warehouse {
    grid: Grid,
    walls: HashSet<usize>,
    boxes: HashSet<usize>,
    robot_location: usize,
    robot_movement: Vec<Direction>,
    
}

enum WarehouseContent {
    Wall,
    Box,
    Empty,
}

impl Warehouse {
    pub fn move_robot(&mut self, movements: &[Direction]) {
        let mut robot_position = self.grid.get_position(self.robot_location).unwrap();

        for m in movements.iter() {
            let possible_position = robot_position.adjacent(*m);

            let possible_content = self.get_content(&possible_position);

            match possible_content {
                WarehouseContent::Wall => {
                    continue;
                },
                WarehouseContent::Empty => {
                    robot_position = possible_position;
                },
                WarehouseContent::Box => {
                    let boxes_start_position = possible_position.clone();
                        
                    let boxes_start_location = self.grid.get_index(&boxes_start_position).unwrap();
                    let mut boxes_to_move = HashSet::new();
                    boxes_to_move.insert(boxes_start_location);

                    let mut boxes_end_position = boxes_start_position;

                    let do_move = loop {
                        let next_position = boxes_end_position.adjacent(*m);
    
                        let next_content = self.get_content(&next_position);

                        match next_content {
                            WarehouseContent::Wall => {
                                break false;
                            },
                            WarehouseContent::Empty => {
                                break true;
                            },
                            WarehouseContent::Box => {
                                let next_location = self.grid.get_index(&next_position).unwrap();
                                boxes_to_move.insert(next_location);

                                boxes_end_position = next_position;
                                continue;
                            }
                        }
                    };

                    if do_move {
                        robot_position = possible_position;

                        // remove all of them first before inserting any back to avoid having to order them properly to avoid conflicts
                        self.boxes.retain(|b| !boxes_to_move.contains(b));

                        for box_location in boxes_to_move.into_iter() {
                            let box_position = self.grid.get_position(box_location).unwrap();
                            let moved_box_position = box_position.adjacent(*m);
                            let moved_box_location = self.grid.get_index(&moved_box_position).unwrap();

                            self.boxes.insert(moved_box_location);
                        }

                    }
                },
            }
        }
    }

    pub fn get_content(&self, position: &Position) -> WarehouseContent {
        // not worrying about out-of-bounds because assuming that there are walls all around
        let location = self.grid.get_index(position).unwrap();
        
        if self.walls.contains(&location) {
            WarehouseContent::Wall
        } else if self.boxes.contains(&location) {
            WarehouseContent::Box
        } else {
            WarehouseContent::Empty
        }
    }
}

const WALL: char = '#';
const BOX: char = 'O';
const ROBOT: char = '@';
const EMPTY: char = '.';

impl From<&str> for Warehouse {
    fn from(text: &str) -> Self {
        let mut completed_warehouse = false;

        let mut warehouse_lines = String::new();
        let mut movement_line = String::new();

        for line in text.lines() {
            if line.is_empty() {
                if !completed_warehouse {
                    completed_warehouse = true;
                }
                continue;
            }

            if completed_warehouse {
                movement_line.push_str(line.trim());
            } else {
                warehouse_lines.push_str(line.trim());
                warehouse_lines.push('\n');
            }
        }

        let (flattened_input, grid) = Grid::parse_input(&warehouse_lines);

        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot_location: Option<usize> = None;

        for (index, content) in flattened_input.chars().enumerate() {
            match content {
                WALL => {walls.insert(index);},
                BOX => {boxes.insert(index);},
                ROBOT => {
                    if robot_location.is_none() {
                        robot_location.replace(index);
                    } else {
                        panic!("There should only be a single robot in the warehouse");
                    }
                },
                EMPTY => {},
                _ => panic!("Unexpected content in warehouse at index {index}: {content}")
            }
        }

        let robot_location = robot_location.unwrap()

        let robot_movement = movement_line.chars()
            .map(|c| Direction::from(c))
            .collect::<Vec<_>>();

        Self { grid, walls, boxes, robot_location, robot_movement }

    }
}