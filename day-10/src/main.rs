use map::{score_trails, Map};
use utils::{part_selection, Part, Timer};

mod map;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-10/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => unimplemented!(),
    }
}

fn part_1(input: &str) {
    let map = Map::from(input);

    let trailhead_trails = map.find_trailhead_trails();

    let score = score_trails(&trailhead_trails);

    println!("{score}");
}
