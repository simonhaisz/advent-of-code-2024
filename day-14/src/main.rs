use robot::Bathroom;
use utils::{part_selection, Grid, Part, Timer};

mod robot;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-14/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => unimplemented!(),
    }
}

fn part_1(input: &str) {
    let grid = Grid::new(103, 101);

    let mut bathroom = Bathroom::from((grid, input));

    bathroom.predict_robot_movement(100);

    let safety_factor = bathroom.safety_factor();

    println!("{safety_factor}");
}
