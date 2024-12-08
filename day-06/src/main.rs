use map::Map;
use utils::{part_selection, Part, Timer};

mod map;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-06/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => part_2(&text),
    }
}

fn part_1(input: &str) {
    let map = Map::from(input);

    let (_, unique_guard_indixes) = map.predict_guard();

    println!("{}", unique_guard_indixes.len());
}

fn part_2(_input: &str) {
    unimplemented!()
}
