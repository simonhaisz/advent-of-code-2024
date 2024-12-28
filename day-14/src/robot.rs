use std::collections::HashMap;

use lazy_regex::regex;
use utils::{Direction, Grid, Position, Velocity};

pub struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn do_move(&mut self, grid: &Grid) {
        let next_position = grid.predict_move(&self.position, &self.velocity);

        self.position = next_position;
    }
}

impl From<&str> for Robot {
    fn from(input: &str) -> Self {
        let robot_regex = regex!(r"^p=(?<px>\d+),(?<py>\d+)\s+v=(?<vx>-?\d+),(?<vy>-?\d+)$");

        let captures = robot_regex.captures(input);

        let captures = captures.unwrap();

        let parse_capture = |capture: Option<regex::Match<'_>>| {
            capture.unwrap().as_str().parse::<i32>().unwrap()
        };

        let px = parse_capture(captures.name("px"));
        let py = parse_capture(captures.name("py"));

        let position = Position(py, px);

        let vx = parse_capture(captures.name("vx"));
        let vy = parse_capture(captures.name("vy"));

        let velocity = Velocity(vy, vx);

        Self { position, velocity }
    }
}

pub struct Bathroom {
    grid: Grid,
    robots: Vec<Robot>,
}

impl Bathroom {
    pub fn predict_robot_movement(&mut self, move_count: u32) {
        for _ in 0..move_count {
            for robot in self.robots.iter_mut() {
                robot.do_move(&self.grid);
            }
        }
    }

    pub fn robot_locations(&self) -> Vec<(usize, u32)> {
        let mut locations = HashMap::new();

        for robot in self.robots.iter() {
            let location = self.grid.get_index(&robot.position).unwrap();

            let entry = locations.entry(location).or_default();

            *entry += 1;
        }

        let mut locations = locations.into_iter()
            .map(|(l, c)| (l, c))
            .collect::<Vec<_>>();

        locations.sort_by(|(a, _), (b, _)| a.cmp(b));

        locations
    }

    pub fn robot_quadrants(&self) -> Vec<(Direction, u32)> {
        let mut quadrants = HashMap::new();

        for robot in self.robots.iter() {
            let location = self.grid.get_index(&robot.position).unwrap();

            let quadrant = self.grid.quadrant(location);

            if let Some(quadrant) = quadrant {
                let entry = quadrants.entry(quadrant).or_default();

                *entry += 1;
            }
        }

        let mut quadrants = quadrants.into_iter()
            .map(|(q, c)| (q, c))
            .collect::<Vec<_>>();

        quadrants.sort_by(|(a, _), (b, _)| a.cmp(b));

        quadrants
    }

    pub fn safety_factor(&self) -> u64 {
        let quadrants = self.robot_quadrants();

        let safety_factor = quadrants.into_iter()
            .map(|(_, c)| c as u64)
            .product();

        safety_factor
    }
}

impl From<(Grid, &str)> for Bathroom {
    fn from((grid, text): (Grid, &str)) -> Self {
        let mut robots = vec![];

        for line in text.lines() {
            if line.is_empty() {
                continue;
            }
            let robot = Robot::from(line);
            robots.push(robot);
        }

        Self { grid, robots }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
    ";

    #[test]
    fn single_robot_5_moves() {
        let grid = Grid::new(7, 11);
        let mut robot = Robot::from("p=2,4 v=2,-3");

        for _ in 0..5 {
            robot.do_move(&grid);
        }

        assert_eq!(Position(3, 1), robot.position);
    }

    #[test]
    fn example() {
        let grid = Grid::new(7, 11);

        let mut bathroom = Bathroom::from((grid, EXAMPLE.trim()));

        bathroom.predict_robot_movement(100);

        let locations = bathroom.robot_locations();

        assert_eq!(
            [
                (6, 2),
                (9, 1),
                (22, 1),
                (34, 1),
                (35, 1),
                (49, 1),
                (58, 1),
                (59, 2),
                (67, 1),
                (72, 1),
            ],
            *locations
        );

        let quadrants = bathroom.robot_quadrants();

        assert_eq!(
            [
                (Direction::NorthEast, 3),
                (Direction::SouthEast, 1),
                (Direction::SouthWest, 4),
                (Direction::NorthWest, 1),
            ],
            *quadrants
        );

        let safety_factor = bathroom.safety_factor();

        assert_eq!(12, safety_factor);


    }
}
