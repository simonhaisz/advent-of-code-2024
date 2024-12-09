use equation::Calibration;
use utils::{part_selection, Part, Timer};

mod equation;
mod operator;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-07/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => part_2(&text),
    }
}

fn part_1(input: &str) {
    let calibration = Calibration::from(input);

    let result = calibration.result();

    println!("{result}");
}

fn part_2(_input: &str) {
    unimplemented!();
}
