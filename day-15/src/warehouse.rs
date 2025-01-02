use std::{collections::{HashMap, HashSet}, fmt};

use utils::{Direction, Grid, Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarehouseType {
    Small,
    Embiggened,
}

pub struct Warehouse {
    grid: Grid,
    walls: HashSet<usize>,
    boxes: HashMap<usize, Box>,
    robot_location: usize,
    robot_movement: Vec<Direction>,
    warehouse_type: WarehouseType,
}

enum WarehouseContent {
    Wall,
    Box(usize),
    Robot,
    Empty,
}

struct Box {
    width: u8,
    height: u8,
    location: usize,
}

impl Box {
    fn single(location: usize) -> Self {
        Self { width: 1, height: 1, location }
    }

    fn wide(location: usize) -> Self {
        Self { width: 2, height: 1, location }
    }

    fn contains(&self, warehouse: &Warehouse, other: usize) -> bool {
        let Position(box_row, box_column) = warehouse.grid.get_position(self.location).unwrap();
        let box_height_offset = (self.height as i32) - 1;
        let box_width_offset = (self.width as i32) - 1;

        let Position(other_row, other_column) = warehouse.grid.get_position(other).unwrap();

        other_row >= box_row && other_row <= (box_row + box_height_offset) && other_column >= box_column && other_column <= (box_column + box_width_offset)
    }

    fn adjacent_content(&self, warehouse: &Warehouse, direction: Direction) -> Vec<WarehouseContent> {
        let mut adjacent_positions = vec![];

        let box_position = warehouse.grid.get_position(self.location).unwrap();

        let row_range = box_position.0..(box_position.0 + self.height as i32);
        let column_range = box_position.1..(box_position.1 + self.width as i32);

        match direction {
            Direction::North => {
                let row = box_position.0 - 1;

                for column in column_range {
                    adjacent_positions.push(Position(row, column));
                }
            },
            Direction::East => {
                let column = box_position.1 + self.width as i32;

                for row in row_range {
                    adjacent_positions.push(Position(row, column));
                }
            },
            Direction::South => {
                let row = box_position.0 + self.height as i32;

                for column in column_range {
                    adjacent_positions.push(Position(row, column));
                }
            },
            Direction::West => {
                let column = box_position.1 - 1;

                for row in row_range {
                    adjacent_positions.push(Position(row, column));
                }
            },
            _ => {
                panic!("Unexpected direction {direction:?}")
            }
        }

        adjacent_positions.iter()
            .map(|p| warehouse.get_content(p))
            .collect::<Vec<_>>()
    }
}

impl Warehouse {
    pub fn embiggen(self) -> Self {
        assert_ne!(WarehouseType::Embiggened, self.warehouse_type);

        let small_grid = self.grid;
        let small_walls = self.walls;
        let small_boxes = self.boxes;
        let small_robot_location = self.robot_location;
        let robot_movement = self.robot_movement;

        let grid = Grid::new(small_grid.row_count, small_grid.column_count * 2);

        let mut walls = HashSet::new();
        for index in small_walls.into_iter() {
            walls.insert(index * 2);
            walls.insert(index * 2 + 1);
        }

        let boxes = small_boxes.into_iter()
            .map(|(l, b)| (l * 2, Box::wide(b.location * 2)))
            .collect::<HashMap<_, _>>();

        let robot_location = small_robot_location * 2;

        let warehouse_type = WarehouseType::Embiggened;

        Self { grid, walls, boxes, robot_location, robot_movement, warehouse_type }
    }

    pub fn move_robot(&mut self) {

        let mut robot_position = self.grid.get_position(self.robot_location).unwrap();

        for (_count, m) in self.robot_movement.iter().enumerate() {
            // println!("{self}");

            // println!("{count} -> {m}");
            // println!();

            let possible_position = robot_position.adjacent(*m);

            let possible_content = self.get_content(&possible_position);

            match possible_content {
                WarehouseContent::Wall => {
                    continue;
                },
                WarehouseContent::Empty => {
                    robot_position = possible_position;
                    let robot_location = self.grid.get_index(&robot_position).unwrap();
                    self.robot_location = robot_location;
                },
                WarehouseContent::Box(box_location) => {
                    let mut boxes_to_move = vec![];

                    let mut boxes_to_process = HashSet::new();

                    boxes_to_process.insert(box_location);

                    let do_move = loop {

                        let mut found_wall = false;

                        let mut next_boxes_to_process = HashSet::new();

                        for l in boxes_to_process.into_iter() {
                            let b = self.boxes.get(&l).unwrap();

                            let adjacent_content = b.adjacent_content(&self, *m);

                            for content in adjacent_content.into_iter() {
                                match content {
                                    WarehouseContent::Box(bl) => {
                                        next_boxes_to_process.insert(bl);
                                    },
                                    WarehouseContent::Wall => {
                                        found_wall = true;
                                        break;
                                    },
                                    WarehouseContent::Empty => {},
                                    WarehouseContent::Robot => {
                                        panic!("Robot cannot be in two places at once")
                                    }
                                }
                            }

                            if found_wall {
                                break;
                            }

                            boxes_to_move.push(l);
                        }

                        if found_wall {
                            break false;
                        }

                        if next_boxes_to_process.is_empty() {
                            break true;
                        }

                        boxes_to_process = next_boxes_to_process;
                        
                    };

                    if do_move {
                        robot_position = possible_position;
                        let robot_location = self.grid.get_index(&robot_position).unwrap();
                        self.robot_location = robot_location;

                        let mut moved_boxes = vec![];

                        for box_location in boxes_to_move {
                            let box_position = self.grid.get_position(box_location).unwrap();

                            let mut box_to_move = self.boxes.remove(&box_location).expect(&format!("No box found at {box_position:?}"));

                            assert_eq!(box_location, box_to_move.location);

                            let moved_box_position = box_position.adjacent(*m);
                            let moved_box_location = self.grid.get_index(&moved_box_position).unwrap();
                            box_to_move.location = moved_box_location;

                            moved_boxes.push(box_to_move);
                        }

                        for b in moved_boxes.into_iter() {
                            self.boxes.insert(b.location, b);
                        }
                    }
                },
                WarehouseContent::Robot => {
                    panic!("Robot should not find itself")
                }
            }
        }
    }

    fn get_content(&self, position: &Position) -> WarehouseContent {
        // not worrying about out-of-bounds because assuming that there are walls all around
        let location = self.grid.get_index(position).unwrap();
        
        if self.walls.contains(&location) {
            WarehouseContent::Wall
        } else if let Some((l, _)) = self.boxes.iter().find(|(_, b)| b.contains(self, location)) {
            WarehouseContent::Box(*l)
        } else if self.robot_location == location {
            WarehouseContent::Robot
        } else {
            WarehouseContent::Empty
        }
    }

    pub fn robot_gps_total(&self) -> u32 {
        match self.warehouse_type {
            WarehouseType::Small => {
                self.boxes.iter()
                    .map(|(_, b)| self.grid.get_position(b.location).unwrap())
                    .map(|p| {
                        (p.0 * 100 + p.1) as u32
                    })
                    .sum()
            },
            WarehouseType::Embiggened => {
                self.boxes.iter()
                    .map(|(_, b)| self.grid.get_position(b.location).unwrap())
                    .map(|p| {
                        let robot_gps = (p.0 * 100 + p.1) as u32;

                        robot_gps
                    })
                    .sum()
            }
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
        let mut boxes = HashMap::new();
        let mut robot_location: Option<usize> = None;

        for (index, content) in flattened_input.chars().enumerate() {
            match content {
                WALL => {
                    walls.insert(index);
                },
                BOX => {
                    boxes.insert(index, Box::single(index));
                },
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

        let robot_location = robot_location.unwrap();

        let robot_movement = movement_line.chars()
            .map(|c| Direction::from(c))
            .collect::<Vec<Direction>>();

        let warehouse_type = WarehouseType::Small;

        Self { grid, walls, boxes, robot_location, robot_movement, warehouse_type }

    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.grid.row_count {
            for column in 0..self.grid.column_count {

                let position = Position(row, column);

                let content = self.get_content(&position);

                let c = match content {
                    WarehouseContent::Empty => EMPTY,
                    WarehouseContent::Wall => WALL,
                    WarehouseContent::Box(_) => BOX,
                    WarehouseContent::Robot => ROBOT,
                };

                write!(f, "{c}")?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_example() {
        let text = r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
        ".trim();

        let mut warehouse = Warehouse::from(text);

        warehouse.move_robot();

        let robot_gps_total = warehouse.robot_gps_total();

        assert_eq!(2028, robot_gps_total);
    }

    #[test]
    fn example() {
        let text = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        ".trim();

        let mut warehouse = Warehouse::from(text);

        warehouse.move_robot();

        let robot_gps_total = warehouse.robot_gps_total();

        assert_eq!(10092, robot_gps_total);
    }

    #[test]
    fn small_embiggened_example() {
        let text = r"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
        ".trim();

        let warehouse = Warehouse::from(text);

        let mut warehouse = warehouse.embiggen();

        warehouse.move_robot();

        let robot_gps_total = warehouse.robot_gps_total();

        assert_eq!(618, robot_gps_total);
    }

    #[test]
    fn embiggened_example() {
        let text = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
        ".trim();

        let warehouse = Warehouse::from(text);
        let mut warehouse = warehouse.embiggen();

        warehouse.move_robot();

        let robot_gps_total = warehouse.robot_gps_total();

        assert_eq!(9021, robot_gps_total);
    }
}