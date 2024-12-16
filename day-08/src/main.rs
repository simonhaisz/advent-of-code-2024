use array::Array;
use utils::{part_selection, Part, Timer};

mod array;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-08/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => part_2(&text),
    }
}

fn part_1(input: &str) {
    let array = Array::from(input);

    let locations = array.find_unique_antinode_locations(array::AntinodeRule::TwiceDistance);

    println!("{}", locations.len());
}

fn part_2(input: &str) {
    let array = Array::from(input);

    let locations = array.find_unique_antinode_locations(array::AntinodeRule::Inline);

    println!("{}", locations.len());
}