mod location;

use location::LocationPair;
use utils::{part_selection, Part, Timer};

fn main() {
    let _timer = Timer::new();
    
    let text = std::fs::read_to_string("./day-01/input.txt").unwrap();

    let pair = LocationPair::from(text);

    let part = part_selection();

    match part {
        Part::One => part_1(pair),
        Part::Two => part_2(pair),
    }
}

fn part_1(pair: LocationPair) {
    let total_distance = pair.total_pair_distance();

    println!("{total_distance}")
}

fn part_2(pair: LocationPair) {
    let similarity_score = pair.similarity_score();

    println!("{similarity_score}")
}