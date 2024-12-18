use stone::{StoneCollection, StoneLine};
use utils::{part_selection, Part, Timer};

mod stone;

fn main() {
    let _timer = Timer::new();

    let text = std::fs::read_to_string("./day-11/input.txt").unwrap();

    let part = part_selection();

    match part {
        Part::One => part_1(&text),
        Part::Two => part_2(&text),
    }
}

fn part_1(input: &str) {
    let stone_line = StoneLine::from(input);

    let count = StoneLine::multi_blink_stone_count(stone_line, 25);

    println!("{count}");
}

fn part_2(input: &str) {
    let stone_line = StoneLine::from(input);
    let stone_collection = StoneCollection::from(stone_line);

    let count = StoneCollection::multi_blink_stone_count(stone_collection, 75);

    println!("{count}");
}